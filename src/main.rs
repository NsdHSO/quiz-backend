// src/main.rs

use crate::components::config::ConfigService;
use crate::security::jwt::JwtAuth;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::Local;
use dotenv::dotenv;
use env_logger::{Builder, Env};
use jsonwebtoken::DecodingKey;
use listenfd::ListenFd;
use utoipa_swagger_ui::SwaggerUi;

mod components;
mod db;
mod entity;
mod http_response;
mod security;
mod shared;
mod tests;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Build config once
    let cfg: ConfigService = ConfigService::new().await;

    // Initialize DB using clones so cfg remains usable
    let conn: sea_orm::DatabaseConnection = db::config::init(cfg.database_url.clone(), cfg.sqlx_log)
        .await
        .expect("Failed to initialize database connection");

    // Logging
    Builder::from_env(Env::default().default_filter_or("debug"))
        .format(|buf, record| {
            use std::io::Write;
            let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%.3f");
            writeln!(
                buf,
                "[{}] {} {} - {}",
                timestamp,
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    let mut listened = ListenFd::from_env();

    // Extract needed cfg fields as owned clones BEFORE the closure
    let auth_base_url = cfg.auth_base_url.clone();
    let host = cfg.host.clone();
    let port = cfg.port;

    // Build decoding key from cfg
    let pem_bytes = STANDARD
        .decode(&cfg.access_token_public_key)
        .expect("ACCESS_TOKEN_PUBLIC_KEY is not valid base64");
    let decoding_key =
        DecodingKey::from_rsa_pem(&pem_bytes).expect("ACCESS_TOKEN_PUBLIC_KEY is not a valid PEM");

    // Shared state wrapped in web::Data (Arc) so we can cheaply clone inside the closure
    let db_data = web::Data::new(conn.clone());
    let cfg_data = web::Data::new(cfg.clone());
    let decoding_key_data = web::Data::new(decoding_key.clone());

    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req| origin.as_bytes().starts_with(b"http://"))
            .allowed_origin("https://tevet-troc-client.vercel.app")
            .allowed_origin("https://nsdhso.github.io")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT, header::AUTHORIZATION])
            .supports_credentials();

        App::new()
            .wrap(cors)
            // Clone the Arc wrappers for each new App instance
            .app_data(db_data.clone())
            .app_data(cfg_data.clone())
            .app_data(decoding_key_data.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("")
                            // auth_base_url was cloned outside; we can clone the String again here
                            .wrap(JwtAuth::new(auth_base_url.clone()))
                            .configure(components::graphql::init_routes)
                    )
            )
    });

    server = match listened.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            // Use previously cloned host/port; no borrowing from cfg here
            server
                .bind(format!("{host}:{port}"))
                .unwrap_or_else(|_| panic!("host: {host}> Port {port}"))
        }
    };

    server.run().await
}

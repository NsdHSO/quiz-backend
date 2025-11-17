pub mod routes;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/graphql", web::post().to(routes::graphql_handler))
        .route("/graphql", web::get().to(routes::graphql_playground));
}

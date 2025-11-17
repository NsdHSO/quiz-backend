use actix_web::{web, HttpResponse};
use reqwest::Client;

use crate::components::config::ConfigService;

/// Proxy handler: Forwards all GraphQL POST requests to Strapi
pub async fn graphql_handler(body: web::Bytes, cfg: web::Data<ConfigService>) -> HttpResponse {
    let strapi_url = cfg.strapi_api_url.clone() + "/graphql";
    let client = Client::new();
    let resp = client
        .post(strapi_url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;

    match resp {
        Ok(r) => {
            let status = r.status();
            let bytes = r.bytes().await.unwrap_or_default();
            let actix_status = actix_web::http::StatusCode::from_u16(status.as_u16())
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
            HttpResponse::build(actix_status).body(bytes)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn graphql_playground() -> HttpResponse {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>GraphQL Playground</title>
            <meta charset=utf-8/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css"/>
            <link rel="shortcut icon" href="//cdn.jsdelivr.net/npm/graphql-playground-react/build/favicon.png"/>
            <script src="//cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
        </head>
        <body>
            <div id="root"></div>
            <script>
                window.addEventListener('load', function (event) {
                    GraphQLPlayground.init(document.getElementById('root'), {
                        endpoint: '/v1/graphql',
                        settings: {
                            'editor.theme': 'dark',
                            'tracing.hideTracingResponse': true,
                        }
                    })
                })
            </script>
        </body>
        </html>
    "#;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

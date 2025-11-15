pub mod schema;
pub mod query;
pub mod mutation;
pub mod types;
pub mod context;
pub mod routes;

use actix_web::web;
use juniper::EmptySubscription;
use schema::{Query, Mutation};

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/graphql")
            .route("", web::post().to(routes::graphql_handler))
            .route("", web::get().to(routes::graphql_playground)),
    );
}

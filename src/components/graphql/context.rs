use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct GraphQLContext {
    pub db: DatabaseConnection,
}

impl juniper::Context for GraphQLContext {}

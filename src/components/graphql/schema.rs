use juniper::RootNode;
use crate::components::graphql::query::Query;
use crate::components::graphql::mutation::Mutation;

pub type Schema = RootNode<'static, Query, Mutation, juniper::EmptySubscription>;

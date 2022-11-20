mod handlers;

use crate::{axum::handlers::graphql_handler, graphql::query::QueryRoot};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{
    extract::Extension,
    routing::{get, IntoMakeService},
    Router,
};

pub fn make_service(
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
) -> IntoMakeService<Router> {
    Router::new()
        .route("/", get(handlers::graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
        .into_make_service()
}

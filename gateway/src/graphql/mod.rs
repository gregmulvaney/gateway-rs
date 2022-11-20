pub mod query;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub type GatewaySchema = Schema<query::QueryRoot, EmptyMutation, EmptySubscription>;

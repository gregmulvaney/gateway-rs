mod axum;
mod cli;
mod graphql;
mod grpc;
mod hybrid;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use dotenvy::dotenv;
use graphql::query::QueryRoot;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = std::env::var("GATEWAY_PORT").expect("GATEWAY_PORT must be set");
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    let axum_service = axum::make_service(schema.clone());
    let grpc_service = grpc::make_service().unwrap();
    let hybrid_service = hybrid::make_service(axum_service, grpc_service);

    let server = hyper::Server::bind(&addr).serve(hybrid_service);
    cli::listener(&schema);
    println!("Listening on {}", &addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e)
    }
}

mod handlers;

use crate::grpc::handlers::{gateway::gateway_server::GatewayServer, MyGateway};
use tonic::transport::{server::Routes, Server};

pub fn make_service() -> Result<Routes, anyhow::Error> {
    let server = Server::builder()
        .add_service(GatewayServer::new(MyGateway))
        .into_service();
    Ok(server)
}

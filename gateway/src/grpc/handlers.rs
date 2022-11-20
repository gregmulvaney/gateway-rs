pub mod gateway {
    tonic::include_proto!("gateway");
}

use tonic::{Request, Response, Status};

use gateway::{gateway_server::Gateway, GreetRequest, GreetResponse};

#[derive(Default, Debug)]
pub struct MyGateway;

#[tonic::async_trait]
impl Gateway for MyGateway {
    async fn greet(&self, req: Request<GreetRequest>) -> Result<Response<GreetResponse>, Status> {
        let reply = GreetResponse {
            message: format!("Hello {}", req.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

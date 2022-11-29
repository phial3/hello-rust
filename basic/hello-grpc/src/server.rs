use tonic::{Request, Response, Status, transport::Server as OtherServer};
use tonic::transport::Server;
use hello_rpc::protos::{HelloService, SayRequest, SayResponse, HelloServiceImpl, HelloServiceServer};

// defining a struct for our service
#[derive(Default)]
pub struct HelloServiceImpl {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl HelloService for HelloServiceImpl {
    // rpc impelemented as function
    async fn say(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        // returning a response as SayResponse message as defined in .proto
        Ok(Response::new(SayResponse {
            // reading data from request which is awrapper around our SayRequest message defined in .proto
            message: format!("hello {}", request.get_ref().name),
            special_fields: Default::default(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:50051".parse().unwrap();
    // creating a service
    let hello_service = HelloServiceImpl::default();
    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(HelloServiceServer::new(hello_service))
        .serve(addr)
        .await?;
    Ok(())
}

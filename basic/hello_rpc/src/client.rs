use hello_rpc::hello::{SayRequest, SayResponse};
use hello_rpc::hello_grpc::SayClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    // creating gRPC client from channel
    let mut client = SayClient::new(channel);

    // creating a new Request
    let request = tonic::Request::new(SayRequest {
        name: String::from("anshul"),
        unknown_fields: Default::default(),
        cached_size: ()
    });

    // sending request and waiting for response
    let response = client.send(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}

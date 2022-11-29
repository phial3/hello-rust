use hello_rpc::protos::{HelloService, SayRequest, SayResponse, HelloServiceClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    // creating gRPC client from channel
    let mut client = HelloServiceClient::new(channel);

    // creating a new Request
    let request = tonic::Request::new(SayRequest {
        name: String::from("anshul"),
        special_fields: Default::default(),
    });

    // sending request and waiting for response
    let response = client.send(request).await?.into_inner();
    println!("response={:?}", response);
    Ok(())
}

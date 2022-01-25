use rpcworker_tst::rpcworker_client::RpcWorkerClient;
use rpcworker_tst::RpcWorkerRequest;

pub mod rpcworker_tst {
    tonic::include_proto!("rpcworker");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RpcWorkerClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(RpcWorkerRequest {
        name: "Tonic".into(),
    });

    let response = client.start_job(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
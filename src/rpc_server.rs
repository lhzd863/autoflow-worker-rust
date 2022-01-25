use tonic::{transport::Server, Request, Response, Status};

use rpcworker_tst::rpc_worker_server::{RpcWorker, RpcWorkerServer};
use rpcworker_tst::{RpcWorkerReply, RpcWorkerRequest};

pub mod rpcworker_tst {
    tonic::include_proto!("rpcworker"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyRpcWorker {}

#[tonic::async_trait]
impl RpcWorker for MyRpcWorker {
    async fn start_job(
        &self,
        request: Request<RpcWorkerRequest>,
    ) -> Result<Response<RpcWorkerReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = rpcworker_tst::RpcWorkerReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let rpcworker = MyRpcWorker::default();

    Server::builder()
        .add_service(RpcWorkerServer::new(rpcworker))
        .serve(addr)
        .await?;

    Ok(())
}

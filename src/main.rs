use std::time::Duration;
use tonic::{Request, Response, Status};
use async_trait::async_trait;
use tonic::transport::Server;
use anyhow;

// 如果配置没有修改,可以直接这么写
// mod goods {
//     // tonic::include_proto!("goods");
// }


// 修改输出路径后,IDE提示更加友好
mod proto;

use proto::fetch::{FetchRequest, FetchResult};
use proto::fetch::apis_client::ApisClient;
use proto::fetch::apis_server::{Apis, ApisServer};

#[derive(Default)]
struct ApiImpl;

#[async_trait]
impl Apis for ApiImpl {
    async fn fetch(&self, request: Request<FetchRequest>)
                   -> Result<Response<FetchResult>, Status> {
        // Err(Status::unavailable("server not ready"))
        println!("server got fetch req:{:?}", request.get_ref());
        Ok(Response::new(FetchResult {
            code: 0,
            message: "ok".to_owned(),
            data: Some(String::from("i am data")),
        }))
    }
}

async fn server_task(barrier: std::sync::Arc<tokio::sync::Barrier>) {
    barrier.wait().await;
    Server::builder()
        .add_service(ApisServer::new(ApiImpl::default()))
        .serve("0.0.0.0:50051".parse().unwrap())
        .await.expect("server failed");
}

async fn client_task(barrier: std::sync::Arc<tokio::sync::Barrier>) {
    barrier.wait().await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let req = FetchRequest {
        url: "https://example.com/a.md".to_owned(),
        timeout: Some(30),
        retry: Some(true),
    };
    let rsp = ApisClient::connect("http://127.0.0.1:50051")
        .await.expect("connect")
        .fetch(req)
        .await.expect("fetch");
    println!("server response:{:?}", rsp.get_ref());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::sync::Arc;
    let bars = Arc::new(tokio::sync::Barrier::new(2));
    tokio::spawn(server_task(bars.clone()));
    tokio::spawn(client_task(bars.clone())).await.expect("client task err");
    Ok(())
}
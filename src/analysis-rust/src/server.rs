use tonic::{transport::Server, Request, Response, Status};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use std::net::SocketAddr;

pub mod pb {
    tonic::include_proto!("analysis");
}

use pb::analysis_service_server::{AnalysisService, AnalysisServiceServer};
use pb::{NetworkEvent, AnalysisResponse};

#[derive(Debug, Default)]
pub struct MyAnalysisServer {}

#[tonic::async_trait]
impl AnalysisService for MyAnalysisServer {
    async fn push_event(
        &self,
        request: Request<NetworkEvent>,
    ) -> Result<Response<AnalysisResponse>, Status> {
        let event = request.into_inner();
        
        let reply = AnalysisResponse {
            session_id: format!("sess_{}", event.timestamp),
            status: true,
        };

        Ok(Response::new(reply))
    }
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:50051".parse()?;
    let listener = TcpListener::bind(addr).await?;
    let stream = TcpListenerStream::new(listener);
    let server = MyAnalysisServer::default();

    Server::builder()
        .add_service(AnalysisServiceServer::new(server))
        .serve_with_incoming(stream)
        .await?;

    Ok(())
}
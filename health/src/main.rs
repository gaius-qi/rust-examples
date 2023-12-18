use std::error::Error;
use tonic::transport::Channel;
use tonic::transport::Server;
use tonic_health::pb::{
    health_client::HealthClient as HealthGRPCClient, HealthCheckRequest, HealthCheckResponse,
};

// HealthClient is a wrapper of HealthGRPCClient.
#[derive(Clone)]
pub struct HealthClient {
    // client is the grpc client of the certificate.
    client: HealthGRPCClient<Channel>,
}

// HealthClient implements the grpc client of the health.
impl HealthClient {
    // new creates a new HealthClient.
    pub async fn new(addr: &str) -> Result<Self, Box<dyn Error>> {
        let channel = Channel::from_shared(addr.to_string())?.connect().await?;
        let client = HealthGRPCClient::new(channel);
        Ok(Self { client })
    }

    // check checks the health of the server.
    pub async fn check(
        &self,
        request: HealthCheckRequest,
    ) -> Result<HealthCheckResponse, Box<dyn Error>> {
        let request = Self::make_request(request);
        let response = self.client.clone().check(request).await?;
        Ok(response.into_inner())
    }

    // make_request creates a new request with timeout.
    fn make_request<T>(request: T) -> tonic::Request<T> {
        tonic::Request::new(request)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_service_status("tonic.health.Health", tonic_health::ServingStatus::Serving)
        .await;

    let addr = "[::1]:50051".parse().unwrap();
    tokio::spawn(async move {
        let _ = Server::builder()
            .add_service(health_service)
            .serve(addr)
            .await;
    });

    health_reporter
        .set_service_status(
            "tonic.health.Health",
            tonic_health::ServingStatus::NotServing,
        )
        .await;

    health_reporter
        .set_service_status(
            "tonic.health.Health.B",
            tonic_health::ServingStatus::Serving,
        )
        .await;

    let client = HealthClient::new("http://[::1]:50051").await?;
    let request = client
        .check(HealthCheckRequest {
            service: "tonic.health.Health.B".to_string(),
        })
        .await?;

    println!("HealthCheckResponse: {:?}", request);

    Ok(())
}

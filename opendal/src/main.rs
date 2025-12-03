use opendal::{Operator, layers::HttpClientLayer, layers::TimeoutLayer, raw::HttpClient};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let builder = opendal::services::Oss::default();
    let builder = builder
        .access_key_id("")
        .access_key_secret("")
        .endpoint("")
        .root("/")
        .bucket("");

    let operator = Operator::new(builder)
        .unwrap()
        .finish()
        .layer(TimeoutLayer::new().with_timeout(Duration::from_secs(10)))
        .layer(HttpClientLayer::new(HttpClient::with(
            reqwest::Client::builder().build().unwrap(),
        )));

    let metadata = operator.stat_with("").await.unwrap();
    println!("content length: {:?}", metadata.content_length());
}

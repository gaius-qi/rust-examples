use opendal::{Operator, layers::HttpClientLayer, layers::TimeoutLayer, raw::HttpClient};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let builder = opendal::services::S3::default();
    let builder = builder
        .access_key_id("")
        .secret_access_key("")
        .endpoint("")
        .region("")
        .root("/")
        .bucket("");

    let operator = Operator::new(builder)
        .unwrap()
        .finish()
        .layer(TimeoutLayer::new().with_timeout(Duration::from_secs(10)))
        .layer(HttpClientLayer::new(HttpClient::with(
            reqwest::Client::builder().build().unwrap(),
        )));

    if let Err(err) = operator.stat("test_file").await {
        if err.kind() == opendal::ErrorKind::NotFound {
            println!("entry not exist")
        }
    };

    let mut w = operator
        .writer_with("gaius/data")
        .concurrent(4)
        .await
        .unwrap();

    w.write("hello world".as_bytes()).await.unwrap();
    w.write("yes!".as_bytes()).await.unwrap();
    w.close().await.unwrap();

    let bs = operator.read("gaius/data").await.unwrap();
    println!("read: {} bytes", bs.len());
}

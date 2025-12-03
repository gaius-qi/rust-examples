use opendal::Operator;
use tracing::info;

#[tokio::main]
async fn main() {
    let builder = opendal::services::Oss::default();
    let builder = builder
        .access_key_id("")
        .access_key_secret("")
        .endpoint("")
        .root("/")
        .bucket("");

    let operator = Operator::new(builder).unwrap().finish();
    let metadata = operator.stat_with("").await.unwrap();

    info!("metadata: {:?}", metadata.content_length());
}

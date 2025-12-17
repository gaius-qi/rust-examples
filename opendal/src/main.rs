use opendal::{Operator, layers::HttpClientLayer, layers::TimeoutLayer, raw::HttpClient};
use std::time::Duration;

const CHUNK_SIZE: u64 = 4 * 1024 * 1024;
const CONCURRENT: usize = 8;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s3_builder = opendal::services::S3::default()
        .access_key_id("")
        .secret_access_key("")
        .endpoint("")
        .region("")
        .root("/")
        .bucket("");

    let s3_operator = Operator::new(s3_builder)?
        .finish()
        .layer(TimeoutLayer::new().with_timeout(Duration::from_secs(10)))
        .layer(HttpClientLayer::new(HttpClient::with(
            reqwest::Client::builder().build()?,
        )));

    let fs_builder = opendal::services::Fs::default().root(".");
    let fs_operator = Operator::new(fs_builder)?.finish();

    let meta = fs_operator.stat("src/random.txt").await?;
    let file_size = meta.content_length();

    let reader = fs_operator
        .reader_with("src/random.txt")
        .concurrent(CONCURRENT)
        .chunk(CHUNK_SIZE as usize)
        .await?;

    let mut writer = s3_operator
        .writer_with("gaius/data")
        .concurrent(CONCURRENT)
        .chunk(CHUNK_SIZE as usize)
        .await?;

    let mut offset: u64 = 0;
    while offset < file_size {
        let end = std::cmp::min(offset + CHUNK_SIZE, file_size);
        let buf = reader.read(offset..end).await?;
        writer.write(buf).await?;
        offset = end;
    }

    writer.close().await?;
    println!("finished!");

    let stat = s3_operator.stat("gaius/data").await?;
    println!("file size: {} bytes", stat.content_length());

    Ok(())
}

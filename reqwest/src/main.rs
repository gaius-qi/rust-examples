#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build().unwrap();

    let response = client.head("http://antsys-linkw-offline-oss-sa128.cn-heyuan-alipay-b.oss-alipay.aliyuncs.com/gypsophila/drm/alipay/DEV/GROUP_META_CONFIG_SNAPSHOT/20250804/20250804014114_1754242874137.txt").header("Range", "bytes=0-").send().await.unwrap();
    println!("Response: {:?}", response.headers());
    println!(
        "Content-Length: {:?}",
        response.headers().get("content-length")
    );

    return Ok(());
}

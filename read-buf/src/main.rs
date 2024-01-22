use std::error::Error;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (mut reader, mut writer) = tokio::io::duplex(64);
    writer.write_all(b"hello world 1").await?;
    let mut buf = [0; 11];
    reader.read_exact(&mut buf).await?;
    println!("{:?}", buf);

    writer.write_all(b"hello world 2").await?;
    let mut buf = [0; 11];
    reader.read_exact(&mut buf).await?;
    println!("{:?}", buf);

    Ok(())
}

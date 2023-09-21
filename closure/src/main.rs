use sha2::{Digest, Sha256};
use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncSeekExt, SeekFrom};
use tokio_util::io::InspectReader;

#[tokio::main]
async fn main() -> Result<()> {
    let input = File::open("src/input").await.unwrap();

    let mut hasher = Sha256::new();
    let mut tee = InspectReader::new(input, |bytes| hasher.update(bytes));

    let mut output = OpenOptions::new()
        .write(true)
        .open("src/output")
        .await
        .unwrap();
    output.seek(SeekFrom::Start(3)).await.unwrap();

    io::copy(&mut tee, &mut output).await.unwrap();
    println!("hash: {:x}", hasher.finalize());
    Ok(())
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {}

use sha2::{Digest, Sha256};
use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncSeekExt, SeekFrom};
use tokio_util::io::InspectReader;
use base64ct::{Base64, Encoding};

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
    let hash = hasher.finalize();
    println!("hash: {}", hash);

    let base64_hash = Base64::encode_string(&hash);
    println!("Base64-encoded hash: {}", base64_hash);
    
    let hex_hash = base16ct::lower::encode_string(&hash);
    println!("Hex-encoded hash: {}", hex_hash);

    Ok(())
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {}

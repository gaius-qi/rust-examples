use sha2::{Digest, Sha256};
use tokio::fs::File;
use tokio::io;
use tokio_util::io::InspectReader;

#[tokio::main]
async fn main() -> Result<()> {
    let mut input = File::open("input").await.unwrap();

    let mut hasher = Sha256::new();
    let mut hashing = |bytes| hasher.update(bytes);
    let tee = InspectReader::new(input, hashing);

    let mut output = File::open("output").await.unwrap();
    io::copy(tee.into_inner(), &mut output).await.unwrap();

    println!("{:x}", hasher.finalize());
    Ok(())
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {}

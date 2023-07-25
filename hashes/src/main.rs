use sha2::{Digest, Sha256};

fn main() {
    let mut hasher = Sha256::new();
    hasher.update("127.0.0.1");
    hasher.update("foo");
    let hash = hasher.finalize();

    println!("hash: {:?}", hex::encode(hash));
}

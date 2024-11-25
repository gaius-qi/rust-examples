use sha2::{Digest, Sha256};

fn main() {
    let mut hasher = Sha256::new();
    hasher.update("https://example.com");
    hasher.update("foo");
    hasher.update("bar");
    let hash = hasher.finalize();

    println!("sha256 hash: {:?}", hex::encode(hash));

    let mut hasher1 = blake3::Hasher::new();
    hasher1.update("127.0.0.1".as_bytes());
    hasher1.update("foo".as_bytes());
    let hash1 = hasher1.finalize();

    println!(
        "blake3 hash: {:?}",
        base16ct::lower::encode_string(hash1.as_bytes())
    );
}

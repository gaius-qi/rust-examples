use crc::*;
use gxhash::GxHasher;
use sha2::{Digest, Sha256};
use std::hash::Hasher;
use std::io::Read;
use std::path::PathBuf;
use twox_hash::XxHash64;
use wyhash::WyHash;

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
    hasher1.update("bar".as_bytes());
    let hash1 = hasher1.finalize();

    println!(
        "blake3 hash: {:?}",
        base16ct::lower::encode_string(hash1.as_bytes())
    );

    let mut hasher2 = XxHash64::default();
    hasher2.write("https://example.com".as_bytes());
    hasher2.write("foo".as_bytes());
    hasher2.write("bar".as_bytes());
    let hash2 = hasher2.finish().to_string();

    println!("xxhash64 hash: {:?}", hash2);

    let mut hasher3 = WyHash::default();
    hasher3.write("https://example.com".as_bytes());
    hasher3.write("foo".as_bytes());
    hasher3.write("bar".as_bytes());
    let hash3 = hasher3.finish().to_string();

    println!("wyhash hash: {:?}", hash3);

    let path = PathBuf::from("./test");

    let now = std::time::Instant::now();
    let f = std::fs::File::open(path.as_path()).unwrap();
    let mut buffer = [0; 4096];
    let mut reader = std::io::BufReader::with_capacity(buffer.len(), f);
    let mut hasher = XxHash64::default();
    loop {
        let n = reader.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        hasher.write(&buffer[..n]);
    }

    println!(
        "xxhash64 hash: {:?}, cost: {:?}",
        hasher.finish().to_string(),
        now.elapsed()
    );

    let now = std::time::Instant::now();
    let f = std::fs::File::open(path.as_path()).unwrap();
    let mut buffer = [0; 4096];
    let mut reader = std::io::BufReader::with_capacity(buffer.len(), f);
    let crc = Crc::<u32, Table<16>>::new(&CRC_32_ISCSI);
    let mut digest = crc.digest();
    loop {
        let n = reader.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        digest.update(&buffer[..n]);
    }

    println!(
        "crc32 hash: {:?}, cost: {:?}",
        digest.finalize(),
        now.elapsed()
    );

    let now = std::time::Instant::now();
    let f = std::fs::File::open(path.as_path()).unwrap();
    let mut buffer = [0; 4096];
    let mut reader = std::io::BufReader::with_capacity(buffer.len(), f);
    let mut hasher = Sha256::new();
    loop {
        let n = reader.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        hasher.update(&buffer[..n]);
    }

    println!(
        "sha256 hash: {:?}, cost: {:?}",
        hex::encode(hasher.finalize()),
        now.elapsed()
    );

    let now = std::time::Instant::now();
    let f = std::fs::File::open(path.as_path()).unwrap();
    let mut buffer = [0; 4096];
    let mut reader = std::io::BufReader::with_capacity(buffer.len(), f);
    let mut hasher = blake3::Hasher::new();
    loop {
        let n = reader.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        hasher.update(&buffer[..n]);
    }

    println!(
        "blake3 hash: {:?}, cost: {:?}",
        hasher.finalize().to_hex(),
        now.elapsed()
    );

    let now = std::time::Instant::now();
    let f = std::fs::File::open(path.as_path()).unwrap();
    let mut buffer = [0; 4096];
    let mut reader = std::io::BufReader::with_capacity(buffer.len(), f);
    let mut hasher = GxHasher::default();
    loop {
        let n = reader.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        hasher.write(&buffer[..n]);
    }

    println!(
        "gxhash hash: {:x}, cost: {:?}",
        hasher.finish(),
        now.elapsed()
    );

    let now = std::time::Instant::now();
    let f = std::fs::File::open(path.as_path()).unwrap();
    let mut buffer = [0; 4096];
    let mut reader = std::io::BufReader::with_capacity(buffer.len(), f);
    let mut hasher = WyHash::default();
    loop {
        let n = reader.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        hasher.write(&buffer[..n]);
    }

    println!(
        "wyhash hash: {:?}, cost: {:?}",
        hasher.finish(),
        now.elapsed()
    );

    let now = std::time::Instant::now();
    let f = std::fs::File::open(path.as_path()).unwrap();
    let mut buffer = [0; 4096];
    let mut reader = std::io::BufReader::with_capacity(buffer.len(), f);
    let mut hasher = cityhasher::CityHasher::new();
    loop {
        let n = reader.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        hasher.write(&buffer[..n]);
    }

    println!(
        "cityhash hash: {:x}, cost: {:?}",
        hasher.finish(),
        now.elapsed()
    );
}

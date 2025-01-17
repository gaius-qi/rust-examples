use crc::*;
use crc32fast::Hasher;

fn main() {
    let message = b"Hello world!";
    let crc = crc32c::crc32c(message);
    println!("hash = {}", crc.to_string());

    let mut hasher = Hasher::new();
    hasher.update(b"Hello world!");
    let checksum = hasher.finalize();
    println!("hash = {}", checksum);

    const BYTES: &[u8] = "Hello world!".as_bytes();
    const CKSUM: u32 = const_crc32::crc32(BYTES);
    println!("hash = {}", CKSUM.to_string());

    let crc = Crc::<u32>::new(&CRC_32_ISCSI);
    let mut digest = crc.digest();
    digest.update(b"Hello world!");
    println!("hash = {}", digest.finalize().to_string());

    let crc = Crc::<u32, Table<16>>::new(&CRC_32_ISCSI);
    let mut digest = crc.digest();
    digest.update(b"Hello world!");
    println!("hash = {}", digest.finalize().to_string());
}

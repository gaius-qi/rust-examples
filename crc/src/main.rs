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
}

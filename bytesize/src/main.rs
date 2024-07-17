use bytesize::{to_string, ByteSize};

fn main() {
    println!("File size: {}", to_string(4194304, true));

    let size = ByteSize::gib(4);
    println!("File size: {}", size.as_u64());
}

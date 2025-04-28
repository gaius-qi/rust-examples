use rustix::fs::{FallocateFlags, fallocate};
use std::fs::File;
use std::io;
use std::os::unix::io::AsFd;

fn fallocate_zero(file_path: &str, length: u64) -> io::Result<()> {
    let file = File::options().write(true).create(true).open(file_path)?;

    let fd = file.as_fd();

    let offset = 0;
    file.set_len(length)?;
    let flags = FallocateFlags::ZERO_RANGE | FallocateFlags::KEEP_SIZE;
    println!("Attempting fallocate with zero range for {} bytes", length);

    // 调用 fallocate
    match fallocate(fd, flags, offset, length) {
        Ok(_) => {
            println!("Fallocate with zero range successful.");
            Ok(())
        }
        Err(err) => {
            eprintln!("fallocate failed: {}", err);
            Err(io::Error::from_raw_os_error(err.raw_os_error()))
        }
    }
}

fn main() -> io::Result<()> {
    let file_path = "zeroed_file.dat";
    let size_to_allocate = 1024 * 1024;

    match fallocate_zero(file_path, size_to_allocate) {
        Ok(_) => println!("File '{}' allocated and zeroed successfully.", file_path),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

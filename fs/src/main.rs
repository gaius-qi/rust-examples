use std::fs;
use std::io::prelude::*;
use std::io::SeekFrom;

fn main() {
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("content")
        .expect("open file failed");
    f.seek(SeekFrom::Start(10)).expect("seek failed");
    f.write_all("aaa".as_bytes()).expect("write failed");

    f.seek(SeekFrom::Start(20)).expect("seek failed");
    f.write_all("bbb".as_bytes()).expect("write failed");
    f.flush().expect("flush failed");

    let mut f = fs::OpenOptions::new()
        .read(true)
        .open("content")
        .expect("open file failed");

    f.seek(SeekFrom::Start(0)).expect("seek failed");
    let mut buf1 = [0; 3];
    f.read_exact(&mut buf1).expect("read failed");
    println!("{:?}", buf1);

    f.seek(SeekFrom::Start(10)).expect("seek failed");
    let mut buf2 = [0; 3];
    f.read_exact(&mut buf2).expect("read failed");
    println!("{:?}", buf2);

    f.seek(SeekFrom::Start(20)).expect("seek failed");
    let mut buf3 = [0; 3];
    f.read_exact(&mut buf3).expect("read failed");
    println!("{:?}", buf3);
}

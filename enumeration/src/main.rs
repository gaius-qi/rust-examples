use std::fmt;

enum IpAddrKind {
    V4,
    V6,
}

impl fmt::Display for IpAddrKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddrKind::V4 => write!(f, "V4"),
            IpAddrKind::V6 => write!(f, "V6"),
        }
    }
}

fn main() {
    println!("{}", IpAddrKind::V4 as u32);
}

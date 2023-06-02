use local_ip_address::{local_ip, local_ipv6};

fn main() {
    let local_ipv4 = local_ip().unwrap();
    let local_ipv6 = local_ipv6().unwrap();

    println!("This is my local IPv4 address: {:?}", local_ipv4);
    println!("This is my local IPv6 address: {:?}", local_ipv6);
}

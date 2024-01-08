extern crate hashring;

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use hashring::HashRing;

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
struct VNode {
    addr: SocketAddr,
}

impl VNode {
    fn new(ip: &str, port: u16) -> Self {
        let addr = SocketAddr::new(IpAddr::from_str(&ip).unwrap(), port);
        VNode { addr }
    }
}

fn main() {
    let mut ring: HashRing<VNode> = HashRing::new();

    let mut nodes = vec![];
    nodes.push(VNode::new("10.244.1.237", 8002));
    nodes.push(VNode::new("10.244.2.218", 8002));

    for node in nodes {
        ring.add(node);
    }

    let s1 = String::from("4088b9");
    let s2 = s1[0..1].to_string();
    println!("{}", s2);
    println!("{:?}", "4088b9".to_string()[0..1].to_string());

    println!("{:?}", ring.get(&"4088b9".to_string()));
    println!("{:?}", ring.get(&"c16b17".to_string()));
    println!("{:?}", ring.get(&"35fad1".to_string()));
}

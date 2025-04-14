use sysinfo::Networks;

fn main() {
    let mut networks = Networks::new_with_refreshed_list();
    let network_data = networks.get("en0").unwrap();
    println!("Network transmit speed: {}", network_data.transmitted());
    println!(
        "Network total bytes transmitted: {}",
        network_data.total_transmitted()
    );
    println!("Network receive speed: {}", network_data.received());
    println!(
        "Network total bytes received: {}",
        network_data.total_received()
    );

    std::thread::sleep(std::time::Duration::from_secs(5));
    networks.refresh();
    let network_data = networks.get("en0").unwrap();
    println!("Network transmit speed: {}", network_data.transmitted());
    println!(
        "Network total bytes transmitted: {}",
        network_data.total_transmitted()
    );
    println!("Network receive speed: {}", network_data.received());
    println!(
        "Network total bytes received: {}",
        network_data.total_received()
    );
}

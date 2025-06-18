use procfs;
use std::{
    thread::{self, sleep},
    time,
};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};

fn main() {
    // let mut networks = Networks::new_with_refreshed_list();
    // let network_data = networks.get("en0").unwrap();
    // println!("Network transmit speed: {}", network_data.transmitted());
    // println!(
    // "Network total bytes transmitted: {}",
    // network_data.total_transmitted()
    // );
    // println!("Network receive speed: {}", network_data.received());
    // println!(
    // "Network total bytes received: {}",
    // network_data.total_received()
    // );

    // std::thread::sleep(std::time::Duration::from_secs(5));
    // networks.refresh();
    // let network_data = networks.get("en0").unwrap();
    // println!("Network transmit speed: {}", network_data.transmitted());
    // println!(
    // "Network total bytes transmitted: {}",
    // network_data.total_transmitted()
    // );
    // println!("Network receive speed: {}", network_data.received());
    // println!(
    // "Network total bytes received: {}",
    // network_data.total_received()
    // );

    let pid = sysinfo::get_current_pid().unwrap();
    println!("I am: {:}", pid);
    let mut system = sysinfo::System::new_all();
    for i in 0..10 {
        let children: Vec<_> = (0..1000)
            .map(|_| {
                thread::spawn(move || {
                    // just having a bunch of threads doing nothing
                    sleep(time::Duration::from_secs(5));
                })
            })
            .collect();

        system.refresh_all();

        let process =
            procfs::process::Process::new(pid.as_u32() as i32).expect("Our process exists. qed.");
        let threads = process.stat().ok().map(|s| s.num_threads).unwrap();
        let open_fd = process.fd().map(|f| f.count()).unwrap();

        println!(
            "Iteration {:}: I have {:} threads and {:} open file handles",
            i, threads, open_fd
        );

        for t in children {
            t.join().unwrap();
        }
        // these threads are all gone now.
    }
}

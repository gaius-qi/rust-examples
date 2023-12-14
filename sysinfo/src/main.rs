use std::path::PathBuf;
use sysinfo::{DiskExt, System, SystemExt};

fn main() {
    let path = PathBuf::from("/System");

    let mut sys = System::new_all();
    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    println!("=> disks:");
    for disk in sys.disks() {
        if path.starts_with(disk.mount_point()) {
            println!("match {:?}", disk);
        }

        println!("{:?}", disk);
    }
}

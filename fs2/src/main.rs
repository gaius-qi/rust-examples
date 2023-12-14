fn main() {
    let stat = fs2::statvfs("/Applications").unwrap();
    println!("{:?}", stat.available_space());
    println!("{:?}", stat.free_space());
    println!("{:?}", stat.total_space());
}

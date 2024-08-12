fn main() {
    let a: u8 = 98;
    let b: u8 = 60;
    let c = (a - b) as f64 / 100 as f64;
    println!("c = {}", c);

    let d: usize = 10;
    println!("d = {}", d.next_power_of_two());
}

fn main() {
    let a: u8 = 98;
    let b: u8 = 60;
    let c = (a - b) as f64 / 100 as f64;
    println!("c = {}", c);

    let d: usize = 10;
    println!("d = {}", d.next_power_of_two());

    let total_length = 1000;
    let count = 7;

    let length = (total_length as f64 / count as f64) as u64;

    println!("length = {}", length.next_power_of_two());
}

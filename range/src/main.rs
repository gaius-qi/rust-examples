use std::cmp::{max, min};

fn main() {
    let length_part = 5;
    let start = 0;
    let length = 10;
    let end = start + length;

    let mut numbers = Vec::new();
    let mut current_length = 0;
    let mut number = 0;
    while current_length < end {
        current_length = (number + 1) * length_part;
        if current_length > start {
            println!("number: {}", number);
            println!("current_length: {}", current_length);
            println!("start: {}", start);
            numbers.push(number);
        }

        number += 1;
    }

    println!("numbers: {:?}", numbers);

    let a = 1..10;
    let b = 0..2;
    println!("a: {} {}", max(a.start, b.start), min(a.end, b.end));
}

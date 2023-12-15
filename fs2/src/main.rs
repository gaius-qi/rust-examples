fn main() {
    let stats = fs2::statvfs("/Applications").unwrap();
    let available_space = stats.available_space();
    println!("Available space: {}", available_space);

    let total_space = stats.total_space();
    println!("Total space: {}", total_space);

    let usage_percent = 100 - available_space * 100 / total_space;
    println!("Usage percent: {}%", usage_percent);
}

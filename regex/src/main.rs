use regex::Regex;

fn main() {
    let re = Regex::new(r"blobs.*").unwrap();
    let hay = "https://xxx/xx/blobs/xxx";
    println!("Found match? {}", re.is_match(hay));
}

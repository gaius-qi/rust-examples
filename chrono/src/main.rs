use chrono::prelude::*;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    println!("UTC: {}", utc);
    println!("Local: {}", local);
}

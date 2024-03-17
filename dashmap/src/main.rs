use dashmap::{DashMap, DashSet};
use std::sync::Arc;

fn main() {
    let reviews = Arc::new(DashMap::new());
    reviews.insert("Veloren", "What a fantastic game!");

    let reviews2 = reviews.clone();
    let handle = std::thread::spawn(move || {
        reviews2.insert("Jerry", "What a fantastic game!");
    });

    let reviews3 = reviews.clone();
    let handle2 = std::thread::spawn(move || {
        reviews3.insert("Terry", "What a fantastic game!");
    });

    handle.join().unwrap();
    handle2.join().unwrap();

    println!("{:?}", reviews);

    let blogs = DashSet::new();
    blogs.insert("Veloren");
    blogs.insert("Jerry");

    let blog = blogs.iter().next();

    println!("{:?}", blog.unwrap().to_string());
}

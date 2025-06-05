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
    println!("{:?}", reviews);

    let blogs = DashSet::new();
    blogs.insert("Veloren");
    blogs.insert("Jerry");

    let blog = blogs.iter().next();
    println!("{:?}", blog.unwrap().to_string());

    let files = Arc::new(DashMap::<u32, Vec<u32>>::new());
    match files.entry(1) {
        dashmap::mapref::entry::Entry::Vacant(_) => {}
        dashmap::mapref::entry::Entry::Occupied(mut entry) => {
            entry.get_mut().push(1);
        }
    }

    files.entry(2).or_default().push(2);
    match files.entry(2) {
        dashmap::mapref::entry::Entry::Vacant(_) => {}
        dashmap::mapref::entry::Entry::Occupied(mut entry) => {
            entry.get_mut().push(1);
        }
    }
    files.remove(&2);

    match files.entry(2) {
        dashmap::mapref::entry::Entry::Vacant(_) => {}
        dashmap::mapref::entry::Entry::Occupied(mut entry) => {
            entry.get_mut().push(1);
        }
    }

    println!("{:?}", files.is_empty());
    println!("{:?}", files);
}

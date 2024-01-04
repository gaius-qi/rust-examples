use std::error::Error;
use tokio::sync::RwLock;

#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut vals = Vec::new();
    vals.push(Person {
        name: String::from("John"),
        age: 32,
    });
    vals.push(Person {
        name: String::from("Melissa"),
        age: 33,
    });
    vals.push(Person {
        name: String::from("Adam"),
        age: 34,
    });
    vals.push(Person {
        name: String::from("Cindy"),
        age: 30,
    });

    vals.sort_by(|a, b| a.age.cmp(&b.age));

    println!("Sorted by age: {:?}", vals);

    let mut views = Vec::new();
    for val in vals.iter() {
        println!("Name: {}, Age: {}", val.name, val.age);
        views.push(val.name.clone());
    }

    vals.retain(|val| !views.contains(&val.name));

    println!("Filtered by name: {:?}", vals);

    let mut v1 = Vec::new();
    let v2 = RwLock::new(Vec::new());

    v1.push(Person {
        name: String::from("John"),
        age: 32,
    });
    v1.push(Person {
        name: String::from("Adam"),
        age: 34,
    });

    v2.write().await.push(Person {
        name: String::from("John"),
        age: 32,
    });
    v2.write().await.push(Person {
        name: String::from("Adam"),
        age: 34,
    });

    let v2 = v2.read().await;

    if v1.len() == v2.len() && v1.iter().all(|e| v2.contains(e)) {
        println!("v1 and v2 are equal");
    } else {
        println!("v1 and v2 are not equal");
    }

    Ok(())
}

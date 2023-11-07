#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
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
}

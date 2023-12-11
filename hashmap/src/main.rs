use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(1, 2);
    map.insert(2, 3);

    map.entry(1).and_modify(|v| *v += 1);
    map.entry(3).and_modify(|v| *v += 1).or_insert(1);

    println!("{:?}", map);
}

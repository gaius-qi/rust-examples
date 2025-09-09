use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;

fn main() {
    let choices = ['a', 'b', 'c'];
    let weights = [10000, 20000, 30000];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::rng();
    for _ in 0..100 {
        println!("{}", choices[dist.sample(&mut rng)]);
    }
}

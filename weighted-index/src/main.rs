use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;

fn main() {
    let weights = [184392642952, 162384693384, 112517285280];
    let dist = WeightedIndex::<u64>::new(weights).unwrap();
    let mut rng = rand::rng();

    let mut counter = [0; 3];
    for _ in 0..1000 {
        let index = dist.sample(&mut rng);
        counter[index] += 1;
    }

    println!("a: {}, b: {}, c: {}", counter[0], counter[1], counter[2]);
}

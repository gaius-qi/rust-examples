use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
    let bytes = "hello".as_bytes();
    cache.put("apple", bytes);

    println!("{:?}", cache.get("apple").unwrap());
}

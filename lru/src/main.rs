use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Cache is the cache for storing http response by LRU algorithm.
#[derive(Clone)]
pub struct Cache {
    /// pieces stores the piece cache data with piece id and value.
    pieces: Arc<Mutex<LruCache<String, bytes::Bytes>>>,
}

/// Cache implements the cache for storing http response by LRU algorithm.
impl Cache {
    /// new creates a new cache with the specified capacity.
    pub fn new(capacity: usize) -> Self {
        let capacity = NonZeroUsize::new(capacity).unwrap();
        let pieces = Arc::new(Mutex::new(LruCache::new(capacity)));
        Cache { pieces }
    }

    /// get_piece gets the piece content from the cache.
    pub fn get_piece(&self, id: &str) -> Option<bytes::Bytes> {
        let mut pieces = self.pieces.lock().unwrap();
        pieces.get(id).cloned()
    }

    /// add_piece create the piece content into the cache, if the key already exists, no operation will
    /// be performed.
    pub fn add_piece(&self, id: &str, content: bytes::Bytes) {
        let mut pieces = self.pieces.lock().unwrap();
        if pieces.contains(id) {
            return;
        }

        pieces.push(id.to_string(), content);
    }

    /// contains_piece checks whether the piece exists in the cache.
    pub fn contains_piece(&self, id: &str) -> bool {
        let pieces = self.pieces.lock().unwrap();
        pieces.contains(id)
    }
}

fn main() {
    let cache = Cache::new(20);
    for i in 0..200 {
        let mut buffer = bytes::BytesMut::with_capacity(4 * 1024 * 1024); // 4MiB
        buffer.resize(4 * 1024 * 1024, 0);
        cache.add_piece(&i.to_string(), buffer.freeze());
    }

    let process = psutil::process::Process::new(std::process::id()).unwrap();

    let memory_info = process.memory_info().unwrap();

    println!(
        "RSS (物理内存): {} 字节 ({:.2} MB)",
        memory_info.rss(),
        memory_info.rss() as f64 / 1024.0 / 1024.0
    );

    println!(
        "VMS (虚拟内存): {} 字节 ({:.2} MB)",
        memory_info.vms(),
        memory_info.vms() as f64 / 1024.0 / 1024.0
    );

    thread::sleep(Duration::from_secs(5));

    println!(
        "RSS (物理内存): {} 字节 ({:.2} MB)",
        memory_info.rss(),
        memory_info.rss() as f64 / 1024.0 / 1024.0
    );

    println!(
        "VMS (虚拟内存): {} 字节 ({:.2} MB)",
        memory_info.vms(),
        memory_info.vms() as f64 / 1024.0 / 1024.0
    );
}

use rocksdb::{Options, DB};
use std::str;

fn main() {
    let path = "./test";
    {
        let mut opts = Options::default();
        opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(6));
        opts.create_if_missing(true);

        let db = DB::open(&opts, path).unwrap();
        db.put(b"piece-1", b"1").unwrap();
        db.put(b"piece-2", b"2").unwrap();
        db.put(b"piece-3", b"3").unwrap();
        db.put(b"task-1", b"xxx-1").unwrap();
        db.put(b"task-2", b"xxx-2").unwrap();

        let mut iter = db.raw_iterator();
        iter.seek(b"piece-");
        while iter.valid() {
            println!(
                "Saw {:?} {:?}",
                str::from_utf8(iter.key().unwrap()),
                str::from_utf8(iter.value().unwrap())
            );
            iter.next();
        }

        match db.get(b"task") {
            Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
        db.delete(b"task").unwrap();
    }
    let _ = DB::destroy(&Options::default(), path);
}

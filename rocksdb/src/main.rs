use rand::Rng;
use rayon::prelude::*;
use rocksdb::{IteratorMode, Options, TransactionDB, TransactionDBOptions, WriteBatch, DB};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    value: i32,
    a: String,
    b: String,
    c: Vec<String>,
    d: u64,
    e: u64,
    f: u64,
    g: u64,
    h: HashMap<String, String>,
    ttl: std::time::Duration,
    created_at: std::time::SystemTime,
    updated_at: std::time::SystemTime,
}

fn main() {
    run1();
    // run2();
    // run3();
    // run4();
    // run5();
    // run6();
}

fn run1() {
    let path = "./test";
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(64));
        opts.set_memtable_prefix_bloom_ratio(0.2);

        let db = DB::open(&opts, path).unwrap();

        db.put(
            "2c85dfe26e28827cb26fe23e924dbfa150cc4969aea1e9e93c744d82fd4508fe-0".as_bytes(),
            b"ccc1",
        )
        .unwrap();
        db.put(
            "2c85dfe26e28827cb26fe23e924dbfa150cc4969aea1e9e93c744d82fd4508fe-1".as_bytes(),
            b"ccc2",
        )
        .unwrap();
        db.put(
            "b2b06d6960a4492b7ec559d57fd65e3b19b622db067852fad52b111ce1ee0cf0-0".as_bytes(),
            b"ddd1",
        )
        .unwrap();
        db.put(
            "b2b06d6960a4492b7ec559d57fd65e3b19b622db067852fad52b111ce1ee0cf0-1".as_bytes(),
            b"ddd2",
        )
        .unwrap();
        let iter = db.prefix_iterator(
            "b2b06d6960a4492b7ec559d57fd65e3b19b622db067852fad52b111ce1ee0cf0".as_bytes(),
        );
        for ele in iter {
            println!(
                "test - key: {:?} value: {:?}",
                str::from_utf8(&ele.clone().unwrap().0).unwrap(),
                str::from_utf8(&ele.clone().unwrap().1).unwrap()
            );
        }
    }

    let _ = DB::destroy(&Options::default(), path);
}

fn run2() {
    let path = "./test2";
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(3));
        opts.set_memtable_prefix_bloom_ratio(0.2);

        let cf_names = ["piece"];
        let db = DB::open_cf_with_opts(
            &opts,
            path,
            cf_names
                .iter()
                .map(|name| (name.to_string(), opts.clone()))
                .collect::<Vec<_>>(),
        )
        .unwrap();

        let handle = db.cf_handle("piece").unwrap();
        db.put_cf(handle, "aaa1".as_bytes(), b"aaa1").unwrap();
        db.put_cf(handle, "aaa2".as_bytes(), b"aaa2").unwrap();
        db.put_cf(handle, "bbb1".as_bytes(), b"bbb1").unwrap();
        db.put_cf(handle, "bbb2".as_bytes(), b"bbb2").unwrap();
        let iter = db.prefix_iterator_cf(handle, "aaa".as_bytes());
        for ele in iter {
            println!(
                "test2 - key: {:?} value: {:?}",
                str::from_utf8(&ele.clone().unwrap().0).unwrap(),
                str::from_utf8(&ele.clone().unwrap().1).unwrap()
            );
        }
    }

    let _ = DB::destroy(&Options::default(), path);
}

fn run3() {
    let path = "./test3";
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(3));
        opts.set_memtable_prefix_bloom_ratio(0.2);

        let db: TransactionDB =
            TransactionDB::open(&opts, &TransactionDBOptions::default(), path).unwrap();

        let txn = db.transaction();
        txn.put("aaa1".as_bytes(), b"aaa1").unwrap();
        txn.put("aaa2".as_bytes(), b"aaa2").unwrap();
        txn.put("bbb1".as_bytes(), b"bbb1").unwrap();
        txn.put("bbb2".as_bytes(), b"bbb2").unwrap();
        let mut value = txn
            .get_for_update("aaa1".as_bytes(), true)
            .unwrap()
            .unwrap();
        value.pop().unwrap();
        txn.put("aaa1".as_bytes(), value).unwrap();
        txn.commit().unwrap();

        let txn = db.transaction();
        let iter = txn.prefix_iterator("aaa".as_bytes());
        for ele in iter {
            println!(
                "test3-prefix - key: {:?} value: {:?}",
                str::from_utf8(&ele.clone().unwrap().0).unwrap(),
                str::from_utf8(&ele.clone().unwrap().1).unwrap()
            );
        }

        for ele in db.iterator(IteratorMode::Start) {
            println!(
                "test3-full - key: {:?} value: {:?}",
                str::from_utf8(&ele.clone().unwrap().0).unwrap(),
                str::from_utf8(&ele.clone().unwrap().1).unwrap()
            );
        }
    }

    let _ = DB::destroy(&Options::default(), path);
}

fn run4() {
    let path = "./test4";
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(3));
        opts.set_memtable_prefix_bloom_ratio(0.2);

        let cf_names = ["piece"];
        let db = DB::open_cf_with_opts(
            &opts,
            path,
            cf_names
                .iter()
                .map(|name| (name.to_string(), opts.clone()))
                .collect::<Vec<_>>(),
        )
        .unwrap();

        let mut rng = rand::thread_rng();
        let mut batch = WriteBatch::default();
        for i in 0..600_000 {
            let key = format!("key{:06}", i);
            let mut h = HashMap::new();
            h.insert("key".to_string(), "value".to_string());
            h.insert("key1".to_string(), "value1".to_string());
            h.insert("key2".to_string(), "value2".to_string());
            h.insert("key3".to_string(), "value3".to_string());
            h.insert("key4".to_string(), "value4".to_string());
            h.insert("key5".to_string(), "value5".to_string());
            h.insert("key6".to_string(), "value6".to_string());
            h.insert("key7".to_string(), "value7".to_string());
            h.insert("Content-Type".to_string(), "application/json".to_string());
            h.insert("Authorization".to_string(), "Bearer some_token".to_string());
            h.insert("User-Agent".to_string(), "MyRustApp/1.0".to_string());
            h.insert("Accept".to_string(), "application/json".to_string());
            h.insert("Cache-Control".to_string(), "no-cache".to_string());
            h.insert("Connection".to_string(), "keep-alive".to_string());
            h.insert("Host".to_string(), "example.com".to_string());
            h.insert(
                "Accept-Encoding".to_string(),
                "gzip, deflate, br".to_string(),
            );
            let value = Data {
                value: rng.gen::<i32>(),
                a: "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                b: "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                c: vec![
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                ],
                d: rng.gen::<u64>(),
                e: rng.gen::<u64>(),
                f: rng.gen::<u64>(),
                g: rng.gen::<u64>(),
                h,
                ttl: std::time::Duration::from_secs(60),
                created_at: std::time::SystemTime::now(),
                updated_at: std::time::SystemTime::now(),
            };
            let value_json = serde_json::to_string(&value).unwrap();
            batch.put(key.as_bytes(), value_json.as_bytes());
        }
        db.write(batch).unwrap();
    }

    {
        let db = DB::open_cf_with_opts(
            &Options::default(),
            path,
            ["piece"]
                .iter()
                .map(|name| (name.to_string(), Options::default()))
                .collect::<Vec<_>>(),
        )
        .unwrap();

        let start = std::time::Instant::now();
        println!("Start searching even numbers.");

        let iter = db.iterator(rocksdb::IteratorMode::Start);
        let iter_a = iter.map(|item| {
            item.ok().and_then(|(key, value)| {
                let value_str = String::from_utf8(value.to_vec()).ok()?;
                let data: Data = serde_json::from_str(&value_str).ok()?;
                Some((key, data))
            })
        });

        let even_numbers: Vec<Data> = iter_a
            .filter_map(|item| {
                item.and_then(|(_, data)| {
                    if data.value % 2 == 0 {
                        Some(data)
                    } else {
                        None
                    }
                })
            })
            .collect();

        println!("Time elapsed: {:?}", start.elapsed());
        println!("Found {} even numbers.", even_numbers.len());
    }

    let _ = DB::destroy(&Options::default(), path);
}

fn run5() {
    let path = "./test5";
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(3));
        opts.set_memtable_prefix_bloom_ratio(0.2);

        let cf_names = ["piece"];
        let db = DB::open_cf_with_opts(
            &opts,
            path,
            cf_names
                .iter()
                .map(|name| (name.to_string(), opts.clone()))
                .collect::<Vec<_>>(),
        )
        .unwrap();

        let mut rng = rand::thread_rng();
        let mut batch = WriteBatch::default();
        for i in 0..600_000 {
            let key = format!("key{:06}", i);
            let mut h = HashMap::new();
            h.insert("key".to_string(), "value".to_string());
            h.insert("key1".to_string(), "value1".to_string());
            h.insert("key2".to_string(), "value2".to_string());
            h.insert("key3".to_string(), "value3".to_string());
            h.insert("key4".to_string(), "value4".to_string());
            h.insert("key5".to_string(), "value5".to_string());
            h.insert("key6".to_string(), "value6".to_string());
            h.insert("key7".to_string(), "value7".to_string());
            h.insert("Content-Type".to_string(), "application/json".to_string());
            h.insert("Authorization".to_string(), "Bearer some_token".to_string());
            h.insert("User-Agent".to_string(), "MyRustApp/1.0".to_string());
            h.insert("Accept".to_string(), "application/json".to_string());
            h.insert("Cache-Control".to_string(), "no-cache".to_string());
            h.insert("Connection".to_string(), "keep-alive".to_string());
            h.insert("Host".to_string(), "example.com".to_string());
            h.insert(
                "Accept-Encoding".to_string(),
                "gzip, deflate, br".to_string(),
            );
            let value = Data {
                value: rng.gen::<i32>(),
                a: "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                b: "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                c: vec![
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                ],
                d: rng.gen::<u64>(),
                e: rng.gen::<u64>(),
                f: rng.gen::<u64>(),
                g: rng.gen::<u64>(),
                h,
                ttl: std::time::Duration::from_secs(60),
                created_at: std::time::SystemTime::now(),
                updated_at: std::time::SystemTime::now(),
            };
            let value = bincode::serialize(&value).unwrap();
            batch.put(key.as_bytes(), value);
        }
        db.write(batch).unwrap();
    }

    {
        let db = DB::open_cf_with_opts(
            &Options::default(),
            path,
            ["piece"]
                .iter()
                .map(|name| (name.to_string(), Options::default()))
                .collect::<Vec<_>>(),
        )
        .unwrap();

        let start = std::time::Instant::now();
        println!("Start searching even numbers.");

        let iter = db.iterator(rocksdb::IteratorMode::Start).par_bridge();
        let iter_a = iter.map(|item| {
            item.ok().and_then(|(key, value)| {
                let data: Data = bincode::deserialize(&value).unwrap();
                Some((key, data))
            })
        });

        let even_numbers: Vec<Data> = iter_a
            .filter_map(|item| {
                item.and_then(|(_, data)| {
                    if data.value % 2 == 0 {
                        Some(data)
                    } else {
                        None
                    }
                })
            })
            .collect();

        println!("Time elapsed: {:?}", start.elapsed());
        println!("Found {} even numbers.", even_numbers.len());
    }

    let _ = DB::destroy(&Options::default(), path);
}

fn run6() {
    let path = "./test6";
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(3));
        opts.set_memtable_prefix_bloom_ratio(0.2);

        let cf_names = ["piece"];
        let db = DB::open_cf_with_opts(
            &opts,
            path,
            cf_names
                .iter()
                .map(|name| (name.to_string(), opts.clone()))
                .collect::<Vec<_>>(),
        )
        .unwrap();

        let mut rng = rand::thread_rng();
        let mut batch = WriteBatch::default();
        for i in 0..6000_000 {
            let key = format!("key{:06}", i);
            let mut h = HashMap::new();
            h.insert("key".to_string(), "value".to_string());
            h.insert("key1".to_string(), "value1".to_string());
            h.insert("key2".to_string(), "value2".to_string());
            h.insert("key3".to_string(), "value3".to_string());
            h.insert("key4".to_string(), "value4".to_string());
            h.insert("key5".to_string(), "value5".to_string());
            h.insert("key6".to_string(), "value6".to_string());
            h.insert("key7".to_string(), "value7".to_string());
            h.insert("Content-Type".to_string(), "application/json".to_string());
            h.insert("Authorization".to_string(), "Bearer some_token".to_string());
            h.insert("User-Agent".to_string(), "MyRustApp/1.0".to_string());
            h.insert("Accept".to_string(), "application/json".to_string());
            h.insert("Cache-Control".to_string(), "no-cache".to_string());
            h.insert("Connection".to_string(), "keep-alive".to_string());
            h.insert("Host".to_string(), "example.com".to_string());
            h.insert(
                "Accept-Encoding".to_string(),
                "gzip, deflate, br".to_string(),
            );
            let value = Data {
                value: rng.gen::<i32>(),
                a: "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                b: "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                c: vec![
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                    "cdsacdsacdscdsacadscddddddddddddddddddddddddddddddddddddddd".to_string(),
                ],
                d: rng.gen::<u64>(),
                e: rng.gen::<u64>(),
                f: rng.gen::<u64>(),
                g: rng.gen::<u64>(),
                h,
                ttl: std::time::Duration::from_secs(60),
                created_at: std::time::SystemTime::now(),
                updated_at: std::time::SystemTime::now(),
            };
            let value = bincode::serialize(&value).unwrap();
            batch.put(key.as_bytes(), value);
        }
        db.write(batch).unwrap();
    }

    {
        let db = DB::open_cf_with_opts(
            &Options::default(),
            path,
            ["piece"]
                .iter()
                .map(|name| (name.to_string(), Options::default()))
                .collect::<Vec<_>>(),
        )
        .unwrap();

        let start = std::time::Instant::now();
        println!("Start searching even numbers.");

        let iter = db.iterator(rocksdb::IteratorMode::Start).par_bridge();
        let pieces = iter
            .map(|item| {
                let (_, value) = item?;
                Ok(value)
            })
            .collect::<Result<Vec<Box<[u8]>>, rocksdb::Error>>()
            .unwrap();
        println!("Time elapsed: {:?}", start.elapsed());

        let eles = pieces
            .par_iter()
            .map(|value| {
                let data: Data = bincode::deserialize(&value).unwrap();
                data
            })
            .filter(|data| data.value % 2 == 0)
            .collect::<Vec<_>>();

        println!("Time elapsed: {:?}", start.elapsed());
        println!("Found {} even numbers.", eles.len());
    }

    let _ = DB::destroy(&Options::default(), path);
}

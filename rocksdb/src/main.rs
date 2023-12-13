use rocksdb::{Options, TransactionDB, TransactionDBOptions, DB};
use std::str;

fn main() {
    let path = "./test";
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(3));
        opts.set_memtable_prefix_bloom_ratio(0.2);

        let db = DB::open(&opts, path).unwrap();

        db.put("aaa1".as_bytes(), b"aaa1").unwrap();
        db.put("aaa2".as_bytes(), b"aaa2").unwrap();
        db.put("bbb1".as_bytes(), b"bbb1").unwrap();
        db.put("bbb2".as_bytes(), b"bbb2").unwrap();
        let iter = db.prefix_iterator("aaa".as_bytes());
        for ele in iter {
            println!(
                "test - key: {:?} value: {:?}",
                str::from_utf8(&ele.clone().unwrap().0).unwrap(),
                str::from_utf8(&ele.clone().unwrap().1).unwrap()
            );
        }
    }

    let _ = DB::destroy(&Options::default(), path);

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

        let iter = db.prefix_iterator("aaa".as_bytes());
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

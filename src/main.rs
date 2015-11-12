extern crate rocksdb;
use rocksdb::{DB, Writable};

fn main() {
    let db = DB::open_default("/tmp/rocksdb.1").unwrap();
    db.put(b"my key", b"my value").unwrap();
    match db.get(b"my key") {
        Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    db.delete(b"my key").unwrap();
}

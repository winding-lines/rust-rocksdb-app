use std::fs::metadata;

use std::env;
use std::fs::{self};
use std::path::{Path};
use std::process::{Command};
// use std::io::{stderr};
// use std::io::{self,Write};

static ARCHIVE: &'static str = "librocksdb.a";
static LIBNAME: &'static str = "rocksdb";
static PROJECT: &'static str = "rocksdb";

fn main() {
    configure_rocksdb();
}

fn configure_rocksdb() {
    //let mut stderr = io::stderr();
    let src = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(PROJECT);
    let archive = src.join(ARCHIVE);
    //writeln!(&mut stderr,"looking for {}", archive.display()).unwrap();
    if !metadata(archive.clone()).is_ok() {
        let mut make = Command::new("make");
        make.current_dir(&src);
        // writeln!(&mut stderr,"running: {:?}", make).unwrap();
        let _ = make.arg("static_lib").status().unwrap();
    }
    // writeln!(&mut stderr, "validating that archive exists").unwrap();
    assert!(metadata(archive.clone()).is_ok(), "Error: archive does not exist after build");

    // copy to the output folder
    let out = &env::var("OUT_DIR").unwrap();
    let dst = Path::new(out);
    // writeln!(&mut stderr, "creating {}",dst.display()).unwrap();
    let _ = fs::create_dir_all(&dst).unwrap();
    match fs::copy(&archive, &dst.join(ARCHIVE)) {
        Ok(_) => {},
        Err(a) => {
            panic!(format!("Error {:?} when copying \n{} \nto {}", a,
                archive.display(), dst.display()));
            }
    }


    println!("cargo:rustc-flags=-L native={} -l static={} -l dylib=stdc++ -l dylib=z -l dylib=bz2",dst.display(), LIBNAME);

}

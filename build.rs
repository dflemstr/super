extern crate syntex;
extern crate serde_codegen;

use std::env;
use std::path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = path::Path::new("src/config.rs.in");
    let dst = path::Path::new(&out_dir).join("config.rs");

    let mut registry = syntex::Registry::new();

    serde_codegen::register(&mut registry);
    registry.expand("", &src, &dst).unwrap();
}

extern crate syntex;
extern crate hello_plugin;

use std::env;
use std::path::Path;

fn main() {
    let mut registry = syntex::Registry::new();
    hello_plugin::register(&mut registry);

    let src = Path::new("src/main.rs.in");
    let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("main.rs");

    registry.expand("compiler_plugin_stable", &src, &dst).unwrap();
}

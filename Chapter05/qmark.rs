use std::io::Read;
use std::path::Path;
use std::fs::File;

fn main() {
    let path = Path::new("data.txt");
    let mut s = String::new();
    File::open(&path)?.read_to_string(&mut s)?;

    println!("Read the string: {}", s);
}

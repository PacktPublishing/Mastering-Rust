use std::io::Read;
use std::path::Path;
use std::fs::File;

fn main() {
    let path = Path::new("data.txt");
    let mut file = File::open(&path)
        .expect("Error while opening data.txt");

    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("Error while reading file contents");

    println!("Read the string: {}", s);
}

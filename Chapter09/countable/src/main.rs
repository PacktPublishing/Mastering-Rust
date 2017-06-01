#![allow(dead_code)]

#[macro_use]
extern crate count_fields_plugin;

trait Countable {
    fn count_fields(&self) -> usize;
}

#[derive(Countable)]
struct S1 {
    x: u32,
    y: u8
}

#[derive(Countable)]
struct S2 {
    s: String,
    x: u64,
    y: i64
}

#[derive(Countable)]
struct S3 {
    s: String 
}

fn main() {
    let s1 = S1 { x: 32, y: 8 };
    let s2 = S2 { s: "String".to_string(), x: 64, y: -64 };
    let s3 = S3 { s: "String".to_string() };
    println!("s1 has {} fields", s1.count_fields());
    println!("s2 has {} fields", s2.count_fields());
    println!("s3 has {} fields", s3.count_fields());
}

#[macro_use]
extern crate chomp;

use chomp::prelude::{SimpleResult, U8Input, parse_only};
use chomp::parsers::take_while;
use chomp::ascii::{is_alphanumeric, is_whitespace};
use chomp::types::Buffer;

use std::str;

fn take_string<I: U8Input>(i: I) -> SimpleResult<I, String> {
    parse!{i;
           let str = take_while(|c| is_alphanumeric(c) | is_whitespace(c));
           ret (String::from_utf8(str.to_vec()).unwrap())
    }
}

fn main() {
    let parsing1 = parse_only(take_string, b"Abc string with alphanumerics 123 only");
    let parsing2 = parse_only(take_string, b"A string containing non-alphanumerics");

    println!("Parsing alphanumeric string: {:?}", parsing1);
    println!("Parsing non-alphanumeric string: {:?}", parsing2);
}

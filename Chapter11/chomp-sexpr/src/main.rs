#[macro_use]
extern crate chomp;

use std::rc::Rc;

use chomp::prelude::{string, SimpleResult, parse_only};
use chomp::ascii::{is_alphanumeric, is_whitespace};
use chomp::types::{Buffer, U8Input};
use chomp::parsers::{take_while, skip_while};

#[derive(Debug)]
enum Svalue {
    Sexpr(Rc<Sexpr>),
    String(String),
}

#[derive(Debug)]
struct Sexpr {
    values: Vec<Svalue>,
}

fn parse_empty_inner_list<I: U8Input>(i: I) -> SimpleResult<I, Svalue> {
    parse!{i;
        string(b"(");
        string(b")");
        ret (Svalue::Sexpr( Rc::new(Sexpr { values: vec!() })))
    }
}

fn parse_inner_list<I: U8Input>(i: I) -> SimpleResult<I, Svalue> {
    parse!{i;
        string(b"(");
        let values = parse_sexpr();
        string(b")");
        ret (Svalue::Sexpr(Rc::new(values)))
    }
}

fn parse_value<I: U8Input>(i: I) -> SimpleResult<I, Svalue> {
    parse!{i;
        let value = take_while(is_alphanumeric);
        ret (Svalue::String(String::from_utf8(value.to_vec()).unwrap()))
    }
}

fn parse_svalue<I: U8Input>(i: I) -> SimpleResult<I, Svalue> {
    parse!{i;
        let svalue = parse_empty_inner_list() <|> parse_inner_list() <|> parse_value();
        skip_while(is_whitespace);
        ret (svalue)
    }
}

fn parse_sexpr<I: U8Input>(i: I) -> SimpleResult<I, Sexpr> {
    parse!{i;
        let val1 = parse_svalue();
        let val2 = parse_svalue();
        let val3 = parse_svalue();
        let val4 = parse_svalue();
        let val5 = parse_svalue();
        ret (Sexpr { values: vec!(val1, val2, val3, val4, val5 )})
    }
}


fn main() {
    let sexpr = parse_only(parse_sexpr, b"()");
    println!("Parsed: {:?}", sexpr);

    let sexpr = parse_only(parse_sexpr, b"a bcd c d");
    println!("Parsed: {:?}", sexpr);

    let sexpr = parse_only(parse_sexpr, b"(a)");
    println!("Parsed: {:?}", sexpr);

    let sexpr = parse_only(parse_sexpr, b"(a bcd c d (e f))");
    println!("Parsed: {:?}", sexpr);
}

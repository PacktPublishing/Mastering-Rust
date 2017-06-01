extern crate syntex;
extern crate syntex_syntax;

use syntex_syntax::tokenstream::TokenTree;
use syntex_syntax::ext::base::{ExtCtxt, DummyResult, MacResult};
use syntex_syntax::ext::quote::rt::Span;
use syntex::Registry;

fn hello<'cx>(_: &'cx mut ExtCtxt, sp: Span, _: &[TokenTree])
         -> Box<MacResult + 'cx> {
    println!("Hello!");
    DummyResult::any(sp)
}

pub fn register(reg: &mut Registry) {
    reg.add_macro("hello", hello);
}

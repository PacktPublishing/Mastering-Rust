#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc_plugin;

use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, DummyResult, MacResult};
use syntax::ext::quote::rt::Span;
use rustc_plugin::Registry;

fn hello(_: &mut ExtCtxt, sp: Span, _: &[TokenTree])
         -> Box<MacResult + 'static> {
    println!("Hello!");
    DummyResult::any(sp)
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("hello", hello);
}

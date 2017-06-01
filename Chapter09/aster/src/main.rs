#![cfg_attr(feature = "nightly", feature(rustc_private))]

extern crate aster;

#[cfg(feature = "nightly")]
extern crate syntax;

#[cfg(not(feature = "nightly"))]
extern crate syntex_syntax as syntax;

fn main() {
    let builder = aster::AstBuilder::new();

    let expr1 = builder.expr()
        .add().u32(1).mul().u32(2).u32(3); // 1+2*3
    let expr2 = builder.expr()
        .mul().add().u32(1).u32(2).u32(3); // (1+2)*3

    println!("{}", syntax::print::pprust::expr_to_string(&expr1));
    println!("{}", syntax::print::pprust::expr_to_string(&expr2));
}

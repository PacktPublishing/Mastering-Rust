use std::cell::Cell;

struct Foo {
    number: u8
}

fn main() {
    let foo_one = Cell::new(Foo { number: 1 });
    let ref_to_foo_1 = &foo_one;
    let ref_to_foo_2 = &foo_one;

    foo_one.set( Foo { number: 2});
    foo_one.set( Foo { number: 3});
}

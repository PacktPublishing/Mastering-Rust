use std::cell::RefCell;

struct Foo {
    number: u8
}

fn main() {
    let foo_one = RefCell::new(Foo { number: 1 });

    let mut ref_to_foo_1 = foo_one.borrow_mut();
    ref_to_foo_1.number = 2;
    drop(ref_to_foo_1);

    let mut ref_to_foo_2 = foo_one.borrow_mut();
    ref_to_foo_2.number = 3;
}

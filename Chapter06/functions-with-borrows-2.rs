fn take_the_n(n: &mut u8) {
    println!("The n is {}", *n);
    *n=10;
}

fn take_the_s(s: &String) {
}

fn take_the_foo(f: &Foo) {
    println!("Foo: {:?}", *f);

}

#[derive(Debug)]
struct Foo {}

fn main() {
    let mut n = 5;
    let s = String::from("string");
    let f = Foo {};

    take_the_n(&n);
    take_the_s(&s);
    take_the_foo(&f);

    println!("n is {}", n);
    println!("s is {}", s);
}

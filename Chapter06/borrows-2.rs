fn main() {
    let x=1;
    let x1 = &x;
    let x2 = &x;

    println!("x1 says {}", *x1);
    println!("x2 says {}", *x2);
}

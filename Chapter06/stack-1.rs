fn f2(y: u8) -> u8 {
    let x = 2+y;
    return x;
}

fn f1(x: u8) -> u8 {
    let z = f2(5);
    return z+x;
}

fn main() {
    println!("f1(9) is {}", f1(9));
}

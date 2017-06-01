fn f1() -> &u8 {
    let x = 4;
    return &x;
}

fn main() {
    let f1s_x = f1();
    println!("f1 returned {:?}", f1s_x);
}

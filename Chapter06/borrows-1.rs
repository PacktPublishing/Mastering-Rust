fn get_a_borrowed_value<'a>() -> &'a u8 {
    let x = 1;
    &x
}

fn main() {
    let value = get_a_borrowed_value();
}

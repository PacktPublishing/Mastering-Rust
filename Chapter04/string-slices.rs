const CONSTANT_STRING: &'static str = "This is a constant string";

fn main() {
    let another_string = "This string is local to the main function";

    println!("Constant string says: {}", CONSTANT_STRING);
    println!("Another string says: {}", another_string);
}

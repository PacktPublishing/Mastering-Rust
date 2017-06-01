fn main() {
    let number_and_string: (u8, &str) = (40, "a static string");

    println!("Number and string in a tuple: {:?}", number_and_string);
}

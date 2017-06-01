fn main() {
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(1);
    numbers_vec.push(2);

    let mut numbers_vec_with_macro = vec![1];
    numbers_vec_with_macro.push(2);

    println!("Both vectors have equal contents: {}",
             numbers_vec == numbers_vec_with_macro);
}

fn main() {
    let target_inferred = "inferred world";              // these two variables
    let target: &'static str = "non-inferred world"; // have identical types
    println!("Hi there, {}", target_inferred);
    println!("Hi there, {}", target);
}

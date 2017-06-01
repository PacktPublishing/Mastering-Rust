use std::collections::HashMap;

fn main() {
    let mut configuration = HashMap::new();
    configuration.insert("path", "/home/user/".to_string());

    println!("Configured path is {:?}", configuration.get("path"));
}

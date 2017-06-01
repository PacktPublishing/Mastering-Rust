use std::panic;

fn main() {
    panic::catch_unwind(|| {
        panic!("Panicing!");

    }).ok();
    println!("Survived that panic.");
}

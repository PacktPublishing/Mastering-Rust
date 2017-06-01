use std::sync::Mutex;

fn main() {
    let mutexed_number = Mutex::new(5);

    println!("Mutexed number plus one equals {}", *mutexed_number + 1);
}

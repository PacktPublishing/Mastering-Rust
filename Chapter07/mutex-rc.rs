use std::sync::Mutex;
use std::thread;
use std::rc::Rc;

fn main() {
    let mutexed_number = Rc::new(Mutex::new(5));
    let mutexed_number_clone_1 = mutexed_number.clone();

    thread::spawn(move || {
        let number = mutexed_number_clone_1.lock().unwrap();
        println!("1 Rc/Mutexed number plus one equals {}", *number + 1);
    });
}

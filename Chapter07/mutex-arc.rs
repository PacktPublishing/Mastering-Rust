use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
use std::time;

const THREADS: u64 = 1_000_000;
const START_NUMBER: u64 = 1;

fn main() {
    let one_millisecond = time::Duration::from_millis(1);
    let one_second = time::Duration::from_millis(1000);

    let mutexed_number = Arc::new(Mutex::new(START_NUMBER));
    let mutexed_number_2 = mutexed_number.clone();

    thread::spawn(move || {
        for _ in 1..THREADS {
            let mutexed_number_clone = mutexed_number.clone();
            thread::spawn(move || {
                thread::sleep(one_millisecond);
                let mut number = mutexed_number_clone.lock().unwrap();
                *number += 1;
            });
        }
    });

    loop {
        thread::sleep(one_second);
        let number = mutexed_number_2.lock().unwrap();
        if *number != START_NUMBER + THREADS - 1 {
            println!("Not there yet, number is {}", *number);
        } else {
            println!("Got there! Number is {}", *number);
            break;
        }
    }
}

use std::thread;
use std::time;

fn main() {
    let mut num = 4; 

    for _ in 1..10 {
        thread::spawn(move || {
            num += 1;
            println!("String is {}", num);
        });
    }

    thread::sleep(time::Duration::from_secs(1));
    println!("In main thread: num is now {}", num);
}

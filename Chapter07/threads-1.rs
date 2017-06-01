use std::thread;

fn main() {
    let outside_string = String::from("outside");

    let thread = thread::spawn( move || {
        println!("Inside thread with string '{}'", outside_string);
    });

    thread.join();
}

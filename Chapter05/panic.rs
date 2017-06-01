use std::thread;

fn f1() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        f2();
    })
}

fn f2() {
    f3();
}

fn f3() {
    panic!("Panicking in f3!");
}

fn main() {
    let child = f1();
    child.join().ok();

    f2();
    println!("This is unreachable code");
}

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();
    tx.send(0);

    thread::spawn(move || {
        tx.send(1)
    });

    thread::spawn(move || {
        tx_clone.send(2)
    });

    println!("Received {} via the channel", rx.recv().unwrap());
    println!("Received {} via the channel", rx.recv().unwrap());
}

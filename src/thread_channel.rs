use std::thread;
use std::sync::mpsc;

pub fn run_thread_channels() {
    let (tx, rx) = mpsc::channel::<&str>();

    thread::spawn(move || {
       tx.send("Hello from thread!").unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got {}", received);
}
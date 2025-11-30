use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run_thread_channels_mutex() {
    let (tx, rx) = mpsc::channel::<&str>();
    let counter = Mutex::new(0);

    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        let mut num = counter.lock().unwrap();
        *num += 1;

        println!("Got: {}, Counter: {}", received, *num);
    }
}
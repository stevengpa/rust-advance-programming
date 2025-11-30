use std::fs::read;
use std::sync::{Arc, RwLock};
use std::thread;

pub fn run_thread_rwlock() {
    let data = Arc::new(RwLock::new(5));

    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut write_data = data_clone.write().unwrap();
        *write_data += 1;
    });

    {
        let read_data = data.read().unwrap();
        println!("Read data: {}", *read_data);
    }

    handle.join().unwrap();
    println!("Final data {}", data.read().unwrap());
}
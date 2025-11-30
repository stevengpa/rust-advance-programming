use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_combine_arc_mutex() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
           let mut vec = data.lock().unwrap();
            vec.push(4);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result {:?}", *data.lock().unwrap());
}
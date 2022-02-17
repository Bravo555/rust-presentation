use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100_000 {
                let mut guard = c.lock().unwrap();
                *guard += 1;
            }
        });
        handles.push(handle);
    }

    // join threads before exiting the main function.
    for handle in handles {
        handle.join().unwrap();
    }

    let counter = counter.lock().unwrap();
    println!("{counter}");
}

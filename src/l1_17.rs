use std::sync::{Arc, Mutex};
use std::thread;

fn counter() -> usize {
    let counter = Arc::new(Mutex::new(0usize));  // Put counter to arc mutes
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);  // Clone counter between threads
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter_clone.lock().unwrap();  // Lock counter in current thread
                *num += 1;  // Increment counter
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = counter.lock().unwrap();
    *final_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working_test() {
        assert_eq!(counter(), 10000);
    }
}

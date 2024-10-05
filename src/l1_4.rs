use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

pub fn infinity_write_read(workers_amount: usize) {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));  // Use Arc and Mutes to use receiver in different threads

    for id in 0..workers_amount {
        let rx = Arc::clone(&rx);  // Clone receiver
        thread::spawn(move || {
            loop {
                let data = rx.lock().expect("Error lock receiver").recv();  // Get data from receiver
                match data {
                    Ok(message) => {  // If no error, print message
                        println!("Recived: {}, by worker {}", message, id);
                    }
                    Err(_) => {
                        println!("Channel closed");
                        break;
                    }
                }
            }
        });
    }

    let mut count = 0;
    loop {  // Sending message endlessly
        let message = format!("Message {}", count);
        tx.send(message).expect("Fail to send message");
        count += 1;
    }
}

#[cfg(all(test, feature = "run-tests"))]
mod tests {
    use super::*;

    #[test]
     fn working_test() {
        infinity_write_read(3);
    }
}
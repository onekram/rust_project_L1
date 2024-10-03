use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

pub fn infinity_write_read(workers_amount: usize) {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    for id in 0..workers_amount {
        let rx = Arc::clone(&rx);
        thread::spawn(move || {
            loop {
                let data = rx.lock().unwrap().recv();
                match data {
                    Ok(message) => {
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
    loop {
        let message = format!("Message {}", count);
        tx.send(message).expect("Fail to send message");
        count += 1;
    }
}

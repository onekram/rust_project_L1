use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn for_a_while_writing(seconds: u64) {

    let (tx, rx) = mpsc::channel();  // Create sender and receiver

    let receiver = thread::spawn(move || {
        while let Ok(message) = rx.recv() {  // While can - read channel
            println!("Recived message: {}", message);
        }
    });


    let sender = thread::spawn(move || {
        let start_time = Instant::now();  // Store start time
        let mut i = 0;
        while start_time.elapsed() < Duration::from_secs(seconds) {  // Send message via channel while time is not over
            let message = format!("Message number: {}", i);
            tx.send(message).expect("Fail to send message");
            thread::sleep(Duration::from_millis(100));
            i += 1;
        }
    });

    receiver.join().expect("Couldn't join on the associated thread");
    sender.join().expect("Couldn't join on the associated thread");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn working_test() {
        for_a_while_writing(3);
    }
}

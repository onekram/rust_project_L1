use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::{Duration};

fn conveyor() { 
    let (tx1, rx1): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let (tx2, rx2): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let data = match rx1.recv() {
                Ok(data) => {
                    println!("Message {data} received via first channel");
                    data
                },
                Err(_) => {
                    println!("First channel closed");
                    break;
                }
            };
            tx2.send(data.wrapping_pow(2)).expect("Error sending message via second channel");
            println!("Message {data} send via first channel");
        }
    });

    thread::spawn(move || {
        loop {
            let data = match rx2.recv() {
                Ok(data) => {
                    println!("Message {data} received via second channel");
                    data
                },
                Err(_) => {
                    println!("Second channel closed");
                    break;
                }
            };
            println!("Received square data: {data}");
        }
    });

    for el in 1..=10 {
        tx1.send(el).expect("Error sending message via first channel");
        println!("Message {el} send via first channel");
        thread::sleep(Duration::from_millis(300));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn working_test() {
        conveyor();
    }
}

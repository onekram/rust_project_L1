use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

fn stop_by_channel() {
    let (tx, rx) = mpsc::channel();

    let receiver = thread::spawn(move || {
        loop {
            let data = rx.recv();
            match data {
                Ok(message) => {
                    println!("Recived: {}", message);
                }
                Err(_) => {
                    println!("Channel closed");
                    break;
                }
            }
        }
    });

    let sender = thread::spawn(move || {
        let start_time = Instant::now();
        let mut count = 0;
        while start_time.elapsed() < Duration::from_secs(3) {
            let message = format!("Message {}", count);
            tx.send(message).expect("Fail to send message");
            thread::sleep(std::time::Duration::from_millis(300));
            count += 1;
        }
        drop(tx);
    });


    receiver.join().expect("Couldn't join on the associated thread");
    sender.join().expect("Couldn't join on the associated thread");
}

async fn stop_by_cancel_token() {
    let token = CancellationToken::new();
    let token_clone = token.clone();

    let task1 = tokio::spawn(async move {
        let mut count = 0;
        loop { 
            tokio::select! {
                _ = token.cancelled() => {
                    println!("Task 1 cancelled.");
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    println!("Task 1 working {}", count);
                    count += 1;
                }
            }
        }
    });

    let task2= tokio::spawn(async move {
        sleep(Duration::from_secs(4)).await;
        token_clone.cancel();
    });

    task1.await.expect("Error while waiting for task 1");
    task2.await.expect("Error while waiting for task 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn stop_by_channel_test() {
        stop_by_channel();
    }

    #[tokio::test]
    async fn stop_by_cancel_token_test() {
        stop_by_cancel_token().await;
    }
}

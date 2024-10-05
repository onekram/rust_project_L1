use flume;
use tokio::signal;
use tokio::task;
use tokio::time::{self, Duration};
use tokio_util::sync::CancellationToken;

async fn infinity_write_read(num_workers: usize) {
    let token = CancellationToken::new();  // Use cancellation token to stop tasks
    let token_clone = token.clone();

    let (tx, rx) = flume::unbounded(); // Use flume to create sender and receiver

    let mut handles = vec![];
    for id in 0..num_workers {
        let rx_clone = rx.clone();  // Clone receiver for current task
        let tk_clone = token_clone.clone();
        let handle = task::spawn(async move {  // Create new task
            loop {
                tokio::select! {  // Async select to match channel cancellation
                    data = rx_clone.recv_async() => {
                        match data {
                            Ok(message) => {
                                println!("Recived: \"{}\", by worker {}", message, id);
                            }
                            Err(_) => {
                                println!("Channel closed");
                                break;
                            }
                        }
                    }
                    _ = tk_clone.cancelled() => {
                        println!("Cancellation signal received by worker {id}. Exiting...");
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    let sender_handle = task::spawn(async move {  // Create task whick will sending messages endlessly
        let mut count = 0;
        loop {
            let message = format!("Message {}", count);
            tx.send(message).expect("Failed to send message");
            count += 1;
            time::sleep(Duration::from_secs(1)).await;
        }
    });

    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");  // Handle Ctrl+C
    println!("\nCtrl+C received, all will be finished.");
    token_clone.cancel();  // If Ctrl+C push - cancel channel


    for handle in handles {
        handle.await.unwrap();
    }

    sender_handle.await.unwrap();
}

#[cfg(all(test, feature = "run-tests"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn working_test() {
        infinity_write_read(3).await;
    }
}

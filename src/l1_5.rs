use flume::{unbounded, Receiver, Sender};
use std::sync::Arc;
use tokio::signal;
use tokio::task;
use tokio::time::{self, Duration};

async fn worker(id: usize, receiver: Arc<Receiver<String>>) {
    while let Ok(message) = receiver.recv_async().await {
        println!("Worker {} received: {}", id, message);
    }
}

async fn run_workers(num_workers: usize) {
    let (sender, receiver) = unbounded();
    let receiver = Arc::new(receiver);

    let mut handles = vec![];
    for id in 0..num_workers {
        let receiver_clone = Arc::clone(&receiver);
        let handle = task::spawn(async move {
            worker(id, receiver_clone).await;
        });
        handles.push(handle);
    }

    let sender_handle = task::spawn(async move {
        let mut count = 0;
        loop {
            let message = format!("Message {}", count);
            if sender.send(message).is_err() {
                break;
            }
            count += 1;
            time::sleep(Duration::from_secs(1)).await;
        }
    });

    let _ = signal::ctrl_c().await;

    for handle in handles {
        let _ = handle.await;
    }

    let _ = sender_handle.await;

    println!("All workers have finished.");
}

use std::sync::mpsc;
use std::thread;

fn parallel_sum_of_squares(n: i32) -> i32 {
    let (sender, receiver) = mpsc::channel();  // Create sender and receiver of channel
    let mut handles = vec![];

    for i in 1..=n { // For each number create its own thread
        let thread_sender = sender.clone();  // Use many senders (multiple producer)
        let handle = thread::spawn(move || {
            if let Some(value) = i.checked_mul(i) {  // Check if overflow
                thread_sender.send(value).expect("Erorr send value")
            }
            
        });
        handles.push(handle);  // Store handle into vec
    }

    for handle in handles {
        handle.join().expect("Erorr join handle");  // Wait all handers done
    }

    let mut sum_of_squares: i32 = 0;
    while let Ok(value) = receiver.try_recv() {  // While can get value from channel
        sum_of_squares = match sum_of_squares.checked_add(value) {  // Add value to sum
            Some(value) => value,
            None => {
                println!("Overflow");
                break;
            }
        }
    }

    sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(parallel_sum_of_squares(4), 30);
    }

    #[test]
    fn second() {
        assert_eq!(parallel_sum_of_squares(14), 1015);
    }
}

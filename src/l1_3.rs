use std::sync::mpsc;
use std::thread;

fn parallel_sum_of_squares(n: u64) -> u64 {
    let (sender, receiver) = mpsc::channel();
    let mut handles = vec![];

    for i in 1..=n {
        let thread_sender = sender.clone();
        let handle = thread::spawn(move || {
            let square = i.checked_mul(i).expect("Overflow");
            thread_sender.send(square).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut sum_of_squares = 0;
    for _ in 1..=n {
        sum_of_squares += receiver.recv().unwrap();
    }
    println!("{sum_of_squares}");
    sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_squares() {
        assert_eq!(parallel_sum_of_squares(4), 30);
    }
}
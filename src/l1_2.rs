use std::thread;

fn print_squares_parallel(n: i32) {
    let mut handles = vec![];

    // Create thread for each number
    for number in 1..=n {
        let handle = thread::spawn(move || {
            number.checked_mul(number)
        });
        // Store handle into vec
        handles.push(handle);
    }

    // Wait for each thread and print the results
    for handle in handles {
        match handle.join() {
            Ok(result) => match result {
                Some(r) => println!("{r}"),
                None => println!("Overflow"),
            }
            Err(_) => println!("Error"),
        }
    }
      
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working () {
        print_squares_parallel(10);
    }
}

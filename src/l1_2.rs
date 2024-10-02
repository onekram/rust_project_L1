use std::thread;

fn print_squares_parallel<const N: usize>(arr: [i32; N]) {
    let mut handles = vec![];

    for &number in &arr {
        let handle = thread::spawn(move || {
            let square = number * number;
            square
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Ok(result) => println!("{}", result),
            Err(_) => println!("Error"),
        }
    }
      
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working () {
        let arr = [1, 2, 3, 4];
        print_squares_parallel(arr);
    }
}

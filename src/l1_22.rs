fn remove_el(){
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    
    let removed_element = v.remove(2);

    println!("Removed element: {}", removed_element);
    println!("Updated vector: {:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working_test() {
        remove_el();
    }
}

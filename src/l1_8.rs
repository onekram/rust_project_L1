use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};

fn hash_map_with_mutex(elements: Vec<(String, i32)>) -> HashMap<String, i32> {
    let map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for (id, (key, value)) in elements.iter().enumerate() {
        let map_clone = Arc::clone(&map);
        let key_clone = key.clone();
        let value_clone = *value;

        let handler = thread::spawn(move || {
            let mut map_lock = map_clone.lock().expect(&format!("Error while locking map in thread {id}"));
            map_lock.insert(key_clone, value_clone);
            println!("Added element to map from thread {id}");
        });
        handles.push(handler);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(map).expect("Failed to unwrap Arc").into_inner().expect("Failed to unlock Mutex")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn hash_map_with_mutex_test() {
        let arr = vec![("first".to_string(), 1), ("second".to_string(), 2), ("third".to_string(), 3)];
        let map = hash_map_with_mutex(arr);
        assert_eq!(map[&"first".to_string()], 1);
        assert_eq!(map[&"second".to_string()], 2);
    }
}

use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use dashmap::DashMap;

fn hash_map_with_mutex(elements: Vec<(String, i32)>) -> HashMap<String, i32> {
    let map = Arc::new(Mutex::new(HashMap::new())); //  Put map in arc mutex
    let mut handles = vec![];

    for (id, (key, value)) in elements.iter().enumerate() {  // Every element should be inserted into map via its own thread
        let map_clone = Arc::clone(&map);  // Clone map between threads
        let key_clone = key.clone();
        let value_clone = *value;

        let handler = thread::spawn(move || {
            let mut map_lock = map_clone.lock().expect(&format!("Error while locking map in thread {id}"));  // Lock map for this thread
            map_lock.insert(key_clone, value_clone);  // Put element into map
            println!("Added element to map from thread {id}");
        });
        handles.push(handler);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(map).expect("Failed to unwrap Arc").into_inner().expect("Failed to unlock Mutex")  // Return map
}

fn dash_map(elements: Vec<(String, i32)>) -> DashMap<String, i32> {
    let map = Arc::new(DashMap::new());  // Put map in arc
    let mut handles = vec![];

    for (id, (key, value)) in elements.iter().enumerate() {
        let map_clone = Arc::clone(&map);  // Clone map between threads
        let key_clone = key.clone();
        let value_clone = value.clone();

        let handler = thread::spawn(move || {
            map_clone.insert(key_clone, value_clone);  // Put element into map
            println!("Added element to map from thread {id}");
        });
        handles.push(handler);
    }

    for handle in handles {
        handle.join().unwrap();
    };

    Arc::try_unwrap(map).expect("Failed to unwrap Arc")  // Return map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn dash_map_test() {
        let arr = vec![("first".to_string(), 1), ("second".to_string(), 2), ("third".to_string(), 3)];
        let map = dash_map(arr);

        assert_eq!(*map.get(&"first".to_string()).unwrap(), 1);
        assert_eq!(*map.get(&"second".to_string()).unwrap(), 2);
    }

    #[test] 
    fn hash_map_with_mutex_test() {
        let arr = vec![("first".to_string(), 1), ("second".to_string(), 2), ("third".to_string(), 3)];
        let map = hash_map_with_mutex(arr);
        assert_eq!(map[&"first".to_string()], 1);
        assert_eq!(map[&"second".to_string()], 2);
    }
}

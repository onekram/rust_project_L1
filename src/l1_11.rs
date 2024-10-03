use std::collections::HashMap;


fn split_temperatures_into_ranges(values: &Vec<f32>) -> HashMap<String, Vec<f32>>{
    let mut map: HashMap<String, Vec<f32>> = HashMap::new();
    for  v in values {
        let lower_bound = (v / 10.0).floor() * 10.0;
        let upper_bound = lower_bound + 10.0;
        let key = format!("[{lower_bound}, {upper_bound})");
        map.entry(key).or_insert_with(Vec::new).push(*v);
    }
    map
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let map = split_temperatures_into_ranges(&vec![-49.1, -45.7, 1.0, 3.5, -11.2]);
        println!("{:?}", map);
        if let Some(values) = map.get("[-50, -40)") {
            assert!(values.contains(&-49.1), "Expected value -49.1 present in the vector for key [-50, -40)");
            assert!(values.contains(&-45.7), "Expected value -45.7 present in the vector for key [-50, -40)");
        } else {
            assert!(false, "Key not found in the map");
        }
    }
}
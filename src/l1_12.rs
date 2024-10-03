use std::collections::HashSet;
use std::hash::Hash;

fn intersection<T>(first: &HashSet<T>, second: &HashSet<T>) -> HashSet<T> 
where T: Eq + Hash + Clone,
{
    let mut res: HashSet<T> = HashSet::new();
    for v in first {
        if second.contains(v) {
            res.insert(v.clone());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let set1: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
        let set2: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();
        let res = intersection(&set1, &set2);
        assert!(!res.contains(&1));
        assert!(res.contains(&4));
    }
}
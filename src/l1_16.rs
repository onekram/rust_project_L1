use std::cmp::Ordering;

fn binary_search<T: Ord>(arr: &[T], value: &T) -> Result<usize, usize> {
    let mut l = 0;
    let mut r = arr.len();

    while l < r {
        let mid = (l + r) / 2;
        (l, r) = match arr[mid].cmp(&value) {
            Ordering::Equal => return Ok(mid),
            Ordering::Greater => (l, mid) ,
            Ordering::Less => (mid + 1, r),
        };
    }
    Err(l)
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn random_test() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let mut arr = Vec::new();
            for _ in 0..100 {
                let value = rng.gen_range(0..1000);
                arr.push(value);
            }

            arr.sort();
            let value = rng.gen_range(0..1000);
            assert_eq!(arr.binary_search(&value), binary_search(&arr, &value));
        }
    }
}

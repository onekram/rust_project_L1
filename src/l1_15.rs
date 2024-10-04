use rand::Rng;
use std::fmt::Debug;

fn partition<T: Ord + Clone, R: Rng>(arr: &mut [T], rng: &mut R) -> usize {
    let pivot = arr[rng.gen_range(0..arr.len())].clone();
    let mut l = 0usize;
    let mut r = arr.len() - 1;  // Need clone cuz mut reference in arr.swap
    loop {
        while arr[l] < pivot {
            l += 1;
        }

        while arr[r] > pivot {
            r -= 1;
        }

        if l >= r {
            return r;
        }
        arr.swap(l, r);
        l += 1;
        r -= 1;
    }
} 

fn quick_sort_impl<T: Ord + Clone, R: Rng>(arr: &mut [T],  rng: &mut R) {
    if arr.len() > 1 {
        let idx = partition(arr, rng);
        quick_sort_impl(&mut arr[0..=idx], rng);
        quick_sort_impl(&mut arr[idx+1..], rng);
    }
}

fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
    let mut rng = rand::thread_rng();
    quick_sort_impl(arr, &mut rng);
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let mut arr = [5, 4, 100, 10, 228];
        quick_sort(&mut arr);
        assert_eq!(arr, [4, 5, 10, 100, 228]);
    }

    #[test]
    fn second_test() {
        let mut arr = [0, 0, 0, -1000, -1, -1];
        quick_sort(&mut arr);
        assert_eq!(arr, [-1000, -1, -1, 0, 0, 0]);
    }

    #[test]
    fn random_test() {
        let mut arr = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let value = rng.gen_range(0..1000);
            arr.push(value);
        }
        let mut arr_copy = arr.clone();

        quick_sort(&mut arr);
        arr_copy.sort();

        assert_eq!(arr, arr_copy);
    }
}

use rand::Rng;

fn partition<T: Ord, R: Rng>(arr: &mut [T], rng: &mut R) -> usize {
    let l = 0usize;
    let r = arr.len() - 1;
    arr.swap(rng.gen_range(l..=r), r);
    let mut i: usize = l;

    for j in l..=r - 1 {
        if arr[j as usize] <= arr[r] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, r);
    i
}

fn quick_sort_impl<T: Ord, R: Rng>(arr: &mut [T], rng: &mut R) {
    if arr.len() > 1 {
        let partition_idx = partition(arr, rng);
        quick_sort_impl(&mut arr[..partition_idx],rng);
        quick_sort_impl(&mut arr[partition_idx+1..], rng);
    }
}

fn quick_sort<T: Ord>(arr: &mut [T]) {
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
        for _ in 0..100 {
            let value = rng.gen_range(0..1000);
            arr.push(value);
        }
        let mut arr_copy = arr.clone();

        quick_sort(&mut arr);
        arr_copy.sort();

        assert_eq!(arr, arr_copy);
    }
}

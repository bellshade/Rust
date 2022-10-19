use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

fn is_sorted<T: Ord>(arr: &[T], len: usize) -> bool {
    for i in 0..len - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }

    true
}

fn generate_index(range: usize, generator: &mut SmallRng) -> usize {
    generator.next_u64() as usize % range
}

fn permute_randomly<T>(arr: &mut [T], len: usize, generator: &mut SmallRng) {
    for i in (1..len).rev() {
        let j = generate_index(i + 1, generator);
        arr.swap(i, j);
    }
}

pub fn bogo_sort<T: Ord>(arr: &mut [T]) {
    let mut random_generator = SmallRng::from_entropy();

    let arr_length = arr.len();
    while !is_sorted(arr, arr_length) {
        permute_randomly(arr, arr_length, &mut random_generator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_array() {
        let mut arr = [1, 8, 3, 2, 7, 4, 6, 5];
        bogo_sort(&mut arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn sorted_array() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
        bogo_sort(&mut arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}

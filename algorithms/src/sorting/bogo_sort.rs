// bogosort adlah algoritma pengurutan yang berdasarkan
// paradigma generate dan melakukan test. fungsi akan
// menghasilkan permutasi dari inputnya sampai menemukan
// satu yang diurutkan
use crate::math::PCG32;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_VALUE: u64 = 4294967296;

fn telah_disorting<T: Ord>(arr: &[T], len: usize) -> bool {
    for i in 0..len - 1{
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}

#[cfg(target_pointer_width = "64")]
fn generate_indeks(range: usize, generator: &mut PCG32) -> usize {
    generator.get_u64() as usize % range
}

#[cfg(not(target_pointer_width = "64"))]
fn generate_indeks(range: usize, generator: &mut PCG32) -> usize {
    generator.get_u32() as usize % range
}


// menggunakan algoritma fisher-yates untuk menggenrte permutasi acak
fn pemutasi_acak<T>(arr: &mut [T], len: usize, generator: &mut PCG32) {
    for i in (1..len).rev() {
        let j = generate_indeks(i + 1,generator);
        arr.swap(i, j);
    }
}

pub fn bogo_sort<T: Ord>(arr: &mut [T]) {
    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(_) => DEFAULT_VALUE,
    };

    let mut random_generator = PCG32::new_default(seed);
    let arr_length = arr.len();
    
    while !telah_disorting(arr, arr_length) {
        permutasi_acak(arr, arr_length, &mut random_generator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_acak() {
        let mut arr = [1, 8, 3, 2, 7, 4, 6, 5];
        bogo_sort(&mut arr);
        
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}

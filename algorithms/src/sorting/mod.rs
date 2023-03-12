mod bogo_sort;

pub use self::bogo_sort::bogo_sort;

use std::cmp;

pub fn telah_disorting<T>(arr: &[T]) -> bool 
where T: cmp::PartialOrd,
{
    if arr.is_empty() {
        return true;
    }
    let mut prev = &arr[0];

    for item arr.iter().skip(1) {
        if prev > item {
            return false;
        }
        prev = item;
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        use super::*;

        assert!(telah_disorting(&[] as &[isize]));
    }
}

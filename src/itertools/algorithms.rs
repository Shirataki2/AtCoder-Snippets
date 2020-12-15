#![allow(dead_code)]
use cargo_snippet::snippet;
use std::ops;

#[snippet("cumsum")]
fn cumsum<T>(a0: T, list: &[T]) -> Vec<T>
    where T: ops::Add + ops::AddAssign + Copy
{
    list.iter().scan(a0, |acc, x| {
        *acc += *x;
        Some(*acc)
    }).collect::<Vec<T>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::u128;

    #[test]
    fn test_cumsum_i32() {
        let v = vec![1, 2, 3, 4, 5];
        let cs = cumsum(0, &v);
        assert_eq!(cs, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_cumsum_u64() {
        let v: Vec<u64> = vec![1, 2, 3, 4, 5];
        let cs = cumsum(0, &v);
        assert_eq!(cs, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_cumsum_u128() {
        let v: Vec<u128> = vec![1, 2, 3, 4, 5];
        let cs = cumsum(0, &v);
        assert_eq!(cs, vec![1, 3, 6, 10, 15]);
    }
}
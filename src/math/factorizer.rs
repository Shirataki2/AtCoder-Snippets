#![allow(dead_code)]
use cargo_snippet::snippet;
use std::collections::HashMap;

#[snippet("factorize")]
/// Naive prime factorization (O(N^(1/2)))
/// 
/// Returns a map of { prime: exponent }.
fn factorize(n: u64) -> HashMap<u64, u64> {
    let mut ret = HashMap::new();
    let mut n = n;
    while n % 2 == 0 {
        *ret.entry(2).or_insert(0) += 1;
        n /= 2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            *ret.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 2;
    }
    if n > 2 { *ret.entry(n).or_insert(0) += 1;}
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_factorize() {
        let map = hashmap! {
            2 => 3,
            3 => 1,
        };
        assert_eq!(factorize(24), map);
    }

    #[test]
    fn test_factorize_large_prime() {
        let map = hashmap! {
            1_000_000_000_039 => 1,
        };
        assert_eq!(factorize(1_000_000_000_039), map);
    }
}
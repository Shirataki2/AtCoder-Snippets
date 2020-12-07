#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_of_15_and_12_is_3 () {
        let result = gcd(15, 12);
        assert_eq!(result, 3);
    }

    #[test]
    fn gcd_of_7_and_1_is_1 () {
        let result = gcd(7, 1);
        assert_eq!(result, 1);
    }
}
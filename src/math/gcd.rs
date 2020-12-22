#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
pub fn gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b)
    }
}

#[snippet(include = "gcd")]
pub fn gcd_list(list: &[u64]) -> u64 {
    list.iter().fold(list[0], |acc, &x| gcd(x, acc))
}

#[snippet(include = "gcd")]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[snippet(include = "lcm")]
pub fn lcm_list(list: &[u64]) -> u64 {
    list.iter().fold(1, |acc, &x| lcm(x, acc))
}

#[snippet("extgcd")]
pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b > 0 {
        let (d, mut y, x) = extgcd(b, a % b);
        y -= (a / b) * x;
        (d, x, y)
    } else {
        (a, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_of_15_and_12_is_3 () {
        let result = gcd(15, 12);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_gcd_of_7_and_1_is_1 () {
        let result = gcd(7, 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_gcd_of_18_and_24_and_60_is_6 () {
        let result = gcd_list(&[18, 24, 60]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_gcd_of_3_and_5_and_7_is_1 () {
        let n = vec![3, 5, 7];
        let result = gcd_list(&n);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_lcm_of_15_and_12_is_60 () {
        let result = lcm(15, 12);
        assert_eq!(result, 60);
    }

    #[test]
    fn test_lcm_of_18_and_24_and_60_is_360 () {
        let result = lcm_list(&[18, 24, 60]);
        assert_eq!(result, 360);
    }

    #[test]
    fn test_extgcd() {
        let a = 1914;
        let b = 899;
        let (d, x, y) = extgcd(a, b);
        assert_eq!((d, x, y), (29, 8, -17));
    }
}
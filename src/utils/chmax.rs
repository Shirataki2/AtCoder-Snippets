#![allow(dead_code, unused_macros)]

macro_rules! min {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::min($a, $b) };
    ($a: expr, $($rest: expr), +) => { std::cmp::min($a, min!($($rest),+)) };
}

macro_rules! max {
    ($a: expr) => { $a };
    ($a: expr, $b: expr) => { std::cmp::max($a, $b) };
    ($a: expr, $($rest: expr), +) => { std::cmp::max($a, max!($($rest),+)) };
}

macro_rules! chmin {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_min = min!($($rest),+);
        if $a > cmp_min { $a = cmp_min; true } else { false }
    }};
}

macro_rules! chmax {
    ($a: expr, $($rest: expr),+) => {{
        let cmp_max = max!($($rest),+);
        if $a < cmp_max { $a = cmp_max; true } else { false }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_min() {
        assert_eq!(min!(4), 4);
        assert_eq!(min!(4, 9), 4);
        assert_eq!(min!(4, 9, 16, 25), 4);
    }

    #[test]
    fn test_max() {
        assert_eq!(max!(4), 4);
        assert_eq!(max!(4, 9), 9);
        assert_eq!(max!(4, 9, 16, 25), 25);
    }

    #[test]
    fn test_chmin() {
        let mut a = 16;
        assert!(!chmin!(a, 35));
        assert_eq!(a, 16);
        assert!(chmin!(a, 9));
        assert_eq!(a, 9);
        assert!(chmin!(a, 18, 6, 7, 2, 16));
        assert_eq!(a, 2);
    }

    #[test]
    fn test_chmax() {
        let mut a = 16;
        assert!(!chmax!(a, 9));
        assert_eq!(a, 16);
        assert!(chmax!(a, 25));
        assert_eq!(a, 25);
        assert!(chmax!(a, 4, 7, 35, 18, 22));
        assert_eq!(a, 35);
    }
}
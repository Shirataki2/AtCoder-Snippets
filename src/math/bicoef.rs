#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet("pascal")]
fn pascal_triangle(n: usize) -> Vec<Vec<u128>> {
    let mut c = vec![vec![0u128; n+1]; n+1];
    c[0][0] = 0;
    for n in 1..=n {
        c[n][0] = 1;
        c[n][n] = 1;
        for k in 1..n {
            c[n][k] = c[n-1][k-1] + c[n-1][k];
        }
    }
    c
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_triangle() {
        let c = pascal_triangle(100);
        assert_eq!(c[12][0], 1);
        assert_eq!(c[12][1], 12);
        assert_eq!(c[12][12], 1);
        assert_eq!(c[12][4], 495);
        assert_eq!(c[91][17], 1_149_101_010_321_489_225);
        assert_eq!(c[100][50], 100_891_344_545_564_193_334_812_497_256);
    }
}
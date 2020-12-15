#![allow(dead_code)]
use cargo_snippet::snippet;
use num::traits::Zero;
use std::ops::*;


#[snippet("bit")]
#[derive(Debug)]
struct BIT<T>
where
    T: Clone + Copy + Zero + 
       Add<Output=T> + Sub<Output=T> + 
       AddAssign + SubAssign +
       PartialEq + PartialOrd + std::fmt::Debug
{
    n: usize,
    data: Vec<T>,
}

#[snippet("bit")]
impl<T> BIT<T>
where
    T: Clone + Copy + Zero + 
       Add<Output=T> + Sub<Output=T> + 
       AddAssign + SubAssign +
       PartialEq + PartialOrd + std::fmt::Debug
{
    pub fn new(n: usize) -> Self {
        let data: Vec<T> = vec![T::zero(); n+1];
        Self { n, data }
    }

    pub fn add(&mut self, i: usize, val: T) {
        let i = i + 1;
        if i == 0 { return }
        let mut k = i as i128;
        while k <= self.n as i128 {
            self.data[k as usize] += val;
            k += k & -k;
        }
    }

    // Calc a[l]+a[l+1]+...a[r]
    pub fn sum(&self, l: isize, r: isize) -> T {
        return self.sum_subtask(r) - self.sum_subtask(l-1);
    }

    // Calc a[0] + a[1] + ... + a[i]
    fn sum_subtask(&self, i: isize) -> T {
        let i = i + 1;
        let mut s: T = T::zero();
        if i == 0 {
            s
        } else {
            let mut k = i as i128;
            while k > 0 {
                s += self.data[k as usize];
                k -= k & -k;
            }
            s
        }
    }

    // Found minimum i where a[0] + a[1] + ... + a[i] >= x (forall a[k] >= 0)
    pub fn lower_bound(&self, x: T) -> usize {
        if x <= T::zero() {
            0
        } else {
            let mut x = x;
            let mut i: usize = 0;
            let mut r: usize = 1;
            while r < self.n { r <<= 1; }
            let mut len = r;
            while len > 0 {
                if i + len < self.n && self.data[i+len] < x {
                    x -= self.data[i+len];
                    i += len;
                }
                len >>= 1;
            }
            i
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_query() {
        let mut bit = BIT::<i64>::new(8);
        bit.add(1, 4);
        bit.add(0, 1);
        bit.add(5, 4);
        bit.add(5, 1);
        bit.add(2, 7);
        bit.add(6, -1);
        assert_eq!(bit.sum(0, 7), 16);
        assert_eq!(bit.sum(4, 7), 4);
        assert_eq!(bit.sum(0, 1), 5);
        bit.add(7, -10);
        assert_eq!(bit.sum(0, 7), 6);
        assert_eq!(bit.sum(6, 7), -11);
    }

    #[test]
    fn test_small_bitsearch() {
        let mut bit = BIT::<i64>::new(8);
        // [1, 2, 3, 4, 5, 6, 7, 8]
        for i in 0..8 {
            bit.add(i, (i+1) as i64);
        }
        assert_eq!(bit.lower_bound(11), 4);
        assert_eq!(bit.lower_bound(10), 3);
        assert_eq!(bit.lower_bound(9), 3);
    }
}
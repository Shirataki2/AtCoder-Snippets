#![allow(dead_code)]
use cargo_snippet::snippet;
use crate::math::gcd::extgcd;
use std::mem::swap;

#[snippet(include = "extgcd")]
/// ## Chinese Remainder Theorem
/// ### Theorem
/// Let `p`, `q` be coprime integers. Then the system of equations
/// 
/// `x ≡ a (mod p), x ≡ b (mod q)`
/// 
/// has unique solution for x modulo pq.
/// 
/// ### Description
/// 
/// This function returns a `(r, m)` pair (where `m=lcm(mi)`)
/// satisfying `x ≡ ni (mod mi) ⇔ x ≡ r (mod m)`
/// for given integers `ns, ms`. 
/// 
/// If `ms` are coprime, there is only one pair of `(r, m)`,
/// otherwise, there may be no pair of `(r, m)`.
/// In that case, returns `(0, 0)`.
/// 
/// ### Example
/// 
/// ```rust
/// # use snippets::math::mods::crt;
/// assert_eq!(crt(&[2, 3], &[3, 5]), (8, 15))
/// ```
pub fn crt(ns: &[i64], ms: &[i64]) -> (i64, i64) {
    assert_eq!(ns.len(), ms.len());
    let (mut r, mut m) = (0, 1);
    for i in 0..ns.len() {
        let (d, p, _q) = extgcd(m, ms[i]);
        match (ns[i] - r) % d {
            0 => {
                let t = (ns[i] - r) / d * p % (ms[i] / d);
                r += m * t;
                m *= ms[i] / d;
            },
            _ => return (0, 0)
        }
    }
    ((r % m + m) % m, m)
}

#[snippet]
pub fn powmod(a: i64, n: i64, modulo: i64) -> i64 {
    let mut res = 1;
    let (mut a, mut n) = (a, n);
    while n > 0 {
        if n & 1 == 1 { res = res * a % modulo }
        a = a * a % modulo;
        n >>= 1;
    }
    res
}

#[snippet(prefix = "use std::mem::swap;")]
pub fn invmod(a: i64, modulo: i64) -> i64 {
    let (mut a, mut b, mut u, mut v) = (a, modulo, 1, 0);
    while b > 0 {
        let t = a / b;
        a -= t * b;
        swap(&mut a, &mut b);
        u -= t * v;
        swap(&mut u, &mut v);
    }
    (u % modulo + modulo) % modulo
}

#[snippet(include = "extgcd")]
pub mod modulo {
    use std::cell::RefCell;
    use super::extgcd;

    type Num = i64;
    thread_local!(
        static MOD: RefCell<Num> = RefCell::new(0)
    );

    pub fn set_mod(m: Num) {
        MOD.with(|x| x.replace(m));
    }

    pub fn modulo() -> Num {
        MOD.with(|x| *x.borrow())
    }

    pub fn signed_mod(a: Num) -> Num {
        let m = modulo();
        (a % m + m) % m
    }

    /// Modulo and `a` must be coprime integers.
    pub fn invmod(a: Num) -> Num {
        let m = modulo();
        let (_d, x, _y) = extgcd(a, m);
        signed_mod(x)
    }

    /// Modulo must be a prime number.
    pub struct Comb {
        n: usize,
        fac: Vec<Num>,
        inv: Vec<Num>,
        ifac: Vec<Num>,
    }

    impl Comb {
        pub fn new(n: usize) -> Self {
            let mut fac = vec![0; n];
            let mut inv = vec![0; n];
            let mut ifac = vec![0; n];
            fac[0] = 1;
            fac[1] = 1;
            ifac[0] = 1;
            ifac[1] = 1;
            inv[1] = 1;
            let m = modulo();
            for i in 2..n {
                let iu = i as i64;
                fac[i] = fac[i - 1] * iu % m;
                inv[i] = m - inv[m as usize % i] * (m / iu) % m;
                ifac[i] = ifac[i - 1] * inv[i] % m;
            }
            Self { n, fac, inv, ifac }
        }

        pub fn comb(&self, n: usize, r: usize) -> Num {
            let m = modulo();
            if n < r { 0 }
            else { self.fac[n] * (self.ifac[r] * self.ifac[n - r] % m) % m }
        }
    }
}

#[cfg(test)]
mod modulo_tests {
    use super::*;
    use super::modulo as m;

    #[test]
    fn test_combination_small() {
        m::set_mod(419);
        let c = m::Comb::new(20);
        assert_eq!(c.comb(12, 1), 12);
        assert_eq!(c.comb(12, 4), 76);
    }

    #[test]
    fn test_combination_large() {
        m::set_mod(1_000_000_007);
        let c = m::Comb::new(100_001);
        assert_eq!(c.comb(100_000, 50_000), 149_033_233);
        assert_eq!(c.comb(77_777, 7_777), 508_121_884);
    }

    #[test]
    fn test_invmod_module() {
        m::set_mod(7);
        assert_eq!(m::invmod(3), 5);
        m::set_mod(429);
        assert_eq!(m::invmod(2), 215);
        m::set_mod(1_000_000_007);
        assert_eq!(m::invmod(123_456_789), 18_633_540);
    }

    fn test_invmod() {
        assert_eq!(invmod(3, 7), 5);
        assert_eq!(invmod(2, 429), 215);
        assert_eq!(invmod(123_456_789, 1_000_000_007), 18_633_540);
    }

    fn test_powmod() {
        assert_eq!(powmod(1_000_000, 2, 1_000_000_007), 999993007);
    }
}
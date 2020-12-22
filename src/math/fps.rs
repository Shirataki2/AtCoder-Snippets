#![allow(dead_code)]
use cargo_snippet::snippet;

// WIP!!!!!!!!
#[snippet("mint2")]
pub mod mint {
    use std::marker::PhantomData;
    use std::ops::*;

    macro_rules! operator_impl {
        ($OpTrait: ident, $op_func: ident, $AssignTrait: ident, $assign_func: ident) => {
            impl<M: Modulo> $OpTrait for MInt<M> {
                type Output = Self;
                fn $op_func(self, rhs: Self) -> Self {
                    let mut res = self;
                    $AssignTrait::$assign_func(&mut res, rhs);
                    res
                }
            }
        }
    }
    macro_rules! define_modulo {
        ($id:ident, $v:tt) => {
            #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
            pub struct $id;
            impl Modulo for $id {
                fn modulo() -> i64 {
                    $v
                }
            }
        }
    }

    define_modulo!(Mod998244353, 998_244_353);
    define_modulo!(Mod7, 7);

    pub trait Modulo: Copy {
        fn modulo() -> i64;
    }


    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct MInt<M>(i64, PhantomData<M>);

    impl<M: Modulo> MInt<M> {
        pub fn new(x: i64) -> Self {
            Self(x, PhantomData)
        }

        pub fn value(&self) -> i64 {
            self.0
        }

        pub fn inv(&self) -> Self {
            self.pow((M::modulo() - 2) as u64)
        }

        pub fn pow(&self, mut n: u64) -> Self {
            let mut x = *self;
            let mut y = Self(1, PhantomData);
            while n > 0 {
                if n % 2 == 1 { y *= x; }
                x *= x;
                n /= 2;
            }
            y
        }
    }

    impl<M: Modulo> Neg for MInt<M> {
        type Output = Self;
        fn neg(self) -> Self {
            Self(if self.0 == 0 { 0 } else { M::modulo() - self.0 }, PhantomData)
        }
    }

    impl<M: Modulo> AddAssign for MInt<M> {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
            if self.0 >= M::modulo() {
                self.0 -= M::modulo();
            }
        }
    }

    impl<M: Modulo> SubAssign for MInt<M> {
        fn sub_assign(&mut self, rhs: Self) {
            self.0 -= rhs.0;
            if self.0 < 0 {
                self.0 += M::modulo();
            }
        }
    }

    impl<M: Modulo> MulAssign for MInt<M> {
        fn mul_assign(&mut self, rhs: Self) {
            self.0 = self.0 * rhs.0 % M::modulo();
        }
    }

    impl<M: Modulo> DivAssign for MInt<M> {
        fn div_assign(&mut self, rhs: Self) {
            if rhs.0 == 0 {
                panic!("Division by zero")
            }
            *self *= rhs.inv();
        }
    }

    operator_impl!(Add, add, AddAssign, add_assign);
    operator_impl!(Sub, sub, SubAssign, sub_assign);
    operator_impl!(Mul, mul, MulAssign, mul_assign);
    operator_impl!(Div, div, DivAssign, div_assign);

    impl<M: Modulo> std::iter::Sum for MInt<M> {
        fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
            iter.fold(MInt::new(0), |acc, x| acc + x)
        }
    }

    impl<M: Modulo> std::iter::Product for MInt<M> {
        fn product<I: Iterator<Item=Self>>(iter: I) -> Self {
            iter.fold(MInt::new(1), |acc, x| acc * x)
        }
    }
}

#[snippet("dft", include("mint2"))]
pub mod dft {
    use super::mint;

    pub trait PrimitiveRoot: mint::Modulo {
        fn primitive_root() -> i64;
    }

    impl PrimitiveRoot for mint::Mod998244353 {
        fn primitive_root() -> i64 { 3 }
    }

    pub fn dft<M: PrimitiveRoot>(a: &mut [mint::MInt<M>], inv: bool) {
        assert!(a.len().is_power_of_two());
        let n = a.len();
        let sft = n.leading_zeros() + 1;
        for i in 0..n {
            let j = i.reverse_bits().wrapping_shr(sft);
            if i < j {
                a.swap(i, j);
            }
        }
        let pr = mint::MInt::new(M::primitive_root());
        let mut w = Vec::with_capacity(n / 2);
        w.push(mint::MInt::new(1));
        for m in (1..).map(|i| 1 << i).take_while(|m| *m <= n) {
            let neg1 = M::modulo() as u64 - 1;
            let s = neg1 / m as u64;
            let w1 = if inv { pr.pow(neg1 - s) } else { pr.pow(s) };
            w.resize(m / 2, mint::MInt::new(0));
            for i in (0..m/4).rev() {
                w[2*i] = w[i];
                w[2*i+1] = w1 * w[i];
            }
            for i in (0..n).step_by(m) {
                for j in 0..m/2 {
                    let t = w[j] * a[i+j+m/2];
                    a[i+j+m/2] = a[i+j] - t;
                    a[i+j] += t;
                }
            }
        }
        if inv {
            let d = mint::MInt::new(n as i64).inv();
            for ai in a {
                *ai *= d;
            }
        }
    }

    pub fn conv<M: PrimitiveRoot>(a: &mut Vec<mint::MInt<M>>, b: &mut Vec<mint::MInt<M>>) {
        let deg = a.len() + b.len() - 1;
        let n = deg.next_power_of_two();
        a.resize(n, mint::MInt::new(0));
        b.resize(n, mint::MInt::new(0));
        dft(a, false);
        dft(b, false);
        for (ai, bi) in a.iter_mut().zip(b.iter()) {
            *ai *= *bi;
        }
        dft(a, true);
        a.truncate(deg);
    }
}

#[snippet(include = ["dft"])]
pub mod fps {
    use super::dft;
    use super::mint;
    use std::mem::swap;
    use std::ops::*;

    pub type Mod = mint::MInt<mint::Mod998244353>;

    macro_rules! impl_from_for_int {
        ($($t:ty)*) => {
            $(impl From<$t> for Mod {
                fn from(i: $t) -> Mod {
                    Mod::new(i as i64)
                }
            })*
        }
    }

    impl_from_for_int!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128);

    #[derive(Clone, Debug)]
    pub struct FormalPowerSeries(Vec<Mod>);

    impl FormalPowerSeries {
        pub fn new(v: &[Mod]) -> Self {
            Self(v.into())
        }

        /// Calculate by newton method
        /// 
        /// ```ignore
        /// x_n+1 = x_n - (1 / x_n - a) / (-1 / x_n^2)
        ///       = x_n - (-x_n + a * x_n^2)
        ///       = 2x_n - a * x_n^2
        /// ```
        pub fn inv(&self) -> Self {
            let n = self.len();
            let mut x = Self(Vec::with_capacity(n));
            x.0.push(self[0].inv());
            let mut x_tmp = Self(Vec::with_capacity(n));
            let mut b = Self(Vec::with_capacity(n));
            let mut len = 1;
            while len < n {
                len *= 2;
                x_tmp.0.clear();
                x_tmp.0.extend_from_slice(&x);
                b.0.clear();
                b.0.extend_from_slice(&self[..n.min(len)]);
                b.0.resize(2 * len, mint::MInt::new(0));
                x.0.resize(2 * len, mint::MInt::new(0));
                dft::dft(&mut b, false);
                dft::dft(&mut x, false);
                for (bi, &xi) in b.iter_mut().zip(x.iter()) {
                    *bi *= xi * xi;
                }
                dft::dft(&mut b, true);
                swap(&mut x, &mut x_tmp);
                x.0.resize(len, mint::MInt::new(0));
                for (xi, &bi) in x.iter_mut().zip(b.iter()).skip(len/2) {
                    *xi = *xi + *xi - bi;
                }
            }
            x.truncate(n);
            x
        }

        pub fn rev(&mut self) {
            self.0.reverse();
        }

        pub fn truncate(&mut self, n: usize) {
            self.0.truncate(n);
        }

        pub fn shrink(&mut self) {
            let mut i = 0;
            while self.0.len() > 0 && self.0.iter().next_back() == Some(&mint::MInt::new(0)) {
                i += 1;
            }
            self.0.truncate(self.0.len() - i);
        }

        pub fn to_vec<T>(&self) -> Vec<T>
        where
            T: From<i64>
        {
            self.0.iter().map(|&v| { v.value().into() }).collect::<Vec<_>>()
        }
    }

    type P = FormalPowerSeries;

    impl Deref for P {
        type Target = [Mod];
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for P {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    macro_rules! impl_binary_op {
        ($Trait:path, $op:tt, $op_assign:tt) => {
            impl $Trait for P {
                type Output = P;
                fn $op(mut self, rhs: Self) -> Self {
                    self.$op_assign(&rhs);
                    self
                }
            }
            impl $Trait for &P {
                type Output = P;
                fn $op(self, rhs: Self) -> Self::Output {
                    let mut res = self.clone();
                    res.$op_assign(rhs);
                    res
                }
            }
        };
    }

    impl Neg for P {
        type Output = P;
        fn neg(mut self) -> P {
            for a in &mut *self {
                *a = -*a;
            }
            self
        }
    }
    impl AddAssign<&Self> for P {
        fn add_assign(&mut self, rhs: &Self) {
            if rhs.0.len() > self.0.len() {
                self.0.resize(rhs.0.len(), mint::MInt::new(0));
            }
            for (li, &ri) in self.0.iter_mut().zip(rhs.0.iter()) {
                *li += ri;
            }
        }
    }
    impl<T> Add<T> for P
    where
        T: Into<i64>
    {
        type Output = P;
        fn add(self, rhs: T) -> Self {
            let mut res = self;
            res[0] += mint::MInt::new(rhs.into());
            res
        }
    }
    impl SubAssign<&Self> for P {
        fn sub_assign(&mut self, rhs: &Self) {
            if rhs.0.len() > self.0.len() {
                self.0.resize(rhs.0.len(), mint::MInt::new(0));
            }
            for (li, ri) in self.0.iter_mut().zip(rhs.0.iter()) {
                *li -= *ri;
            }
            self.shrink();
        }
    }
    impl<T> Sub<T> for P
    where
        T: Into<i64>
    {
        type Output = P;
        fn sub(self, rhs: T) -> Self {
            let mut res = self;
            res[0] -= mint::MInt::new(rhs.into());
            res.shrink();
            res
        }
    }
    impl MulAssign<&Self> for P {
        fn mul_assign(&mut self, rhs: &Self) {
            let mut rhs = rhs.clone();
            dft::conv(&mut self.0, &mut rhs.0);
        }
    }

    impl_binary_op!(Add, add, add_assign);
    impl_binary_op!(Sub, sub, sub_assign);
    impl_binary_op!(Mul, mul, mul_assign);

    impl Div for P {
        type Output = P;
        fn div(self, rhs: Self) -> Self::Output {
            if self.0.len() < rhs.0.len() {
                Self(vec![mint::MInt::new(0)])
            } else {
                let m = self.0.len() - rhs.0.len();
                let mut a_rev = self.clone();
                let mut b_revinv = rhs.clone();
                a_rev.rev();
                b_revinv.rev();
                b_revinv.inv();
                b_revinv.truncate(m + 1);
                let mut q = a_rev * b_revinv;
                q.truncate(m + 1);
                q.rev();
                q
            }
        }
    }

    impl Div for &P {
        type Output = P;
        fn div(self, rhs: Self) -> Self::Output {
            if self.0.len() < rhs.0.len() {
                P::new(&vec![mint::MInt::new(0)])
            } else {
                let m = self.0.len() - rhs.0.len();
                let mut a_rev = self.clone();
                let mut b_revinv = rhs.clone();
                a_rev.rev();
                b_revinv.rev();
                b_revinv.inv();
                b_revinv.truncate(m + 1);
                let mut q = a_rev * b_revinv;
                q.truncate(m + 1);
                q.rev();
                q
            }
        }
    }

    impl Rem for &P {
        type Output = P;
        fn rem(self, rhs: Self) -> Self::Output {
            let q = self / rhs;
            self - &(&q * rhs)
        }
    }

    impl P {
        pub fn divrem(&self, rhs: &Self) -> (Self, Self) {
            let q = self / rhs;
            (q, self - &(&(self / rhs) * rhs))
        }
    }

    impl RemAssign<&Self> for P {
        fn rem_assign(&mut self, rhs: &Self) {
            *self = self.rem(rhs);
        }
    }

    impl P {
        pub fn compose(&self, g: &P, n: usize) -> Self {
            let m = (self.0.len() as f64).sqrt().ceil() as usize;
            let fs = &self.0;
            let mut gs = vec![];
            let mut gm = P::new(&vec![mint::MInt::new(1)]);
            for _ in 0..m {
                let mut arr = gm.clone();
                arr.0.resize(n, mint::MInt::new(0));
                gs.append(&mut arr.0);
                gm *= &g;
                gm.truncate(n);
            }
            let mut hs = vec![mint::MInt::new(0); gs.len()];
            for (k, gr) in gs.chunks_mut(n).enumerate() {
                for (i, hr) in hs.chunks_mut(n).enumerate() {
                    if let Some(f) = fs.get(i * m + k) {
                        for (hi, gi) in hr.iter_mut().zip(gr.iter()) {
                            *hi = *f * *gi + *hi;
                        }
                    }
                }
            }
            let mut gpow = P::new(&vec![mint::MInt::new(1)]);
            let mut result = P::new(&vec![mint::MInt::new(0)]);
            for hr in hs.chunks(n) {
                let mut gh = &P::new(hr) * &gpow;
                gh.truncate(n);
                result += &gh;
                gpow *= &gm;
                gpow.truncate(n);
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::fps::FormalPowerSeries;

    #[test]
    fn test_fps_inv() {
        let v = vec![5, 4, 3, 2, 1];
        let v = v.iter().map(|&x| fps::Mod::from(x)).collect::<Vec<_>>();
        let fps = FormalPowerSeries::new(&v);
        let b = fps.inv();
        assert_eq!(
            b.to_vec::<i64>(),
            vec![598_946_612, 718_735_934, 862_483_121, 635_682_004, 163_871_793]
        );
    }

    #[test]
    fn test_fps_compose() {
        let a = vec![5, 4, 3, 2, 1];
        let b = vec![0, 1, 2, 3, 4];
        let f = a.iter().map(|&x| fps::Mod::from(x)).collect::<Vec<_>>();
        let g = b.iter().map(|&x| fps::Mod::from(x)).collect::<Vec<_>>();
        let f = FormalPowerSeries::new(&f);
        let g = FormalPowerSeries::new(&g);
        let c = f.compose(&g, 5);
        assert_eq!(
            c.to_vec::<i64>(),
            vec![5, 4, 11, 26, 59]
        );
    }

    #[test]
    fn test_mod() {
        type M = mint::MInt::<mint::Mod7>;
        let m5 = M::new(5);
        let m3 = M::new(3);
        assert_eq!(-m5, M::new(2));
        assert_eq!(m5 + m3, M::new(1));
        assert_eq!(m5 - m3, M::new(2));
        assert_eq!(m3 - m5, M::new(5));
        assert_eq!(m3 * m5, M::new(1));
        assert_eq!(m5 / m3, M::new(4));
        assert_eq!(m5.pow(2), M::new(4));
        assert_eq!(m3.inv(), M::new(5));
    }
}
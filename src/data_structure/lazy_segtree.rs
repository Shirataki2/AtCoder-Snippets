#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet("lazysegtree")]
struct LazySegTree<Monoid, Operator>
{
    size: usize,
    height: usize,
    data: Vec<Monoid>,
    lazy: Vec<Operator>,
    f: fn(&Monoid, &Monoid) -> Monoid,
    g: fn(&Monoid, &Operator) -> Monoid,
    h: fn(&Operator, &Operator) -> Operator,
    m0: Monoid,
    o0: Operator,
}

impl<Monoid, Operator> LazySegTree<Monoid, Operator> 
where
    Monoid: Copy + Clone,
    Operator: Eq + Copy + Clone,
{
    fn new(
        n: usize,
        f: fn(&Monoid, &Monoid) -> Monoid,
        g: fn(&Monoid, &Operator) -> Monoid,
        h: fn(&Operator, &Operator) -> Operator,
        m0: Monoid,
        o0: Operator,
    ) -> Self {
        let mut size = 1;
        let mut height = 0;
        while size < n {
            size <<= 1;
            height += 1;
        }
        let data = vec![m0; 2 * size];
        let lazy = vec![o0; 2 * size];
        Self { size, height, data, lazy, f, g, h, m0, o0 }
    }

    fn set(&mut self, k: usize, v: Monoid) {
        self.data[self.size + k] = v;
    }

    fn build(&mut self) {
        for k in (1..=self.size-1).rev() {
            self.data[k] = (self.f)(&self.data[(k<<1)|0], &self.data[(k<<1)|1]);
        }
    }

    fn propagate(&mut self, k: usize) {
        if self.lazy[k] == self.o0 { return }
        self.lazy[(k<<1)|0] = (self.h)(&self.lazy[(k<<1)|0], &self.lazy[k]);
        self.lazy[(k<<1)|1] = (self.h)(&self.lazy[(k<<1)|1], &self.lazy[k]);
        self.data[k] = self.reflect(k);
        self.lazy[k] = self.o0;
    }

    fn reflect(&self, k: usize) -> Monoid {
        if self.lazy[k] == self.o0 {
            self.data[k]
        } else {
            (self.g)(&self.data[k], &self.lazy[k])
        }
    }

    fn recalc(&mut self, k: usize) {
        let mut k = k;
        while {
            k >>= 1;
            k > 0
        } {
            self.data[k] = (self.f)(&self.reflect((k<<1)|0), &self.reflect((k<<1)|1));
        }
    }

    fn thrust(&mut self, k: usize) {
        for i in (1..=self.height).rev() {
            self.propagate(k >> i);
        }
    }

    fn update(&mut self, i: usize, j: usize, x: Operator) {
        if i >= j { return }
        let i = i + self.size;
        let j = j + self.size - 1;
        self.thrust(i);
        self.thrust(j);
        let (mut l, mut r) = (i, j + 1);
        while l < r {
            if l & 1 > 0 {
                self.lazy[l] = (self.h)(&self.lazy[l], &x);
                l += 1;
            }
            if r & 1 > 0 {
                r -= 1;
                self.lazy[r] = (self.h)(&self.lazy[r], &x);
            }
            l >>= 1;
            r >>= 1;
        }
        self.recalc(i);
        self.recalc(j);
    }

    fn query(&mut self, i: usize, j: usize) -> Monoid {
        if i >= j { return self.m0 }
        let i = i + self.size;
        let j = j + self.size - 1;
        self.thrust(i);
        self.thrust(j);
        let (mut l, mut r) = (i, j + 1);    
        let (mut vl, mut vr) = (self.m0, self.m0);
        while l < r {
            if l & 1 > 0 {
                vl = (self.f)(&vl, &self.reflect(l));
                l += 1;
            }
            if r & 1 > 0 {
                r -= 1;
                vr = (self.f)(&self.reflect(r), &vr);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.f)(&vl, &vr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_rsq_small() {
        let mut rsq = LazySegTree::<i64, i64>::new(
            3, |&a, &b| a + b, |&a, &b| a + b, |&a, &b| a + b, 0, 0
        );
        rsq.update(0, 0, 100);
        assert_eq!(0, rsq.query(0, 2));
        rsq.update(0, 1, 1);
        assert_eq!(1, rsq.query(0, 3));
        rsq.update(0, 2, 2);
        assert_eq!(5, rsq.query(0, 3));
        rsq.update(0, 3, 3);
        rsq.update(1, 2, 4);
        rsq.update(1, 3, 5);
        rsq.update(2, 3, 6);
        // 6 14 14
        assert_eq!(6, rsq.query(0, 1));
        assert_eq!(14, rsq.query(1, 2));
        assert_eq!(14, rsq.query(2, 3));
        assert_eq!(34, rsq.query(0, 3));
        assert_eq!(28, rsq.query(1, 3));
        rsq.update(1, 3, 5);
        // 6 19 19
        assert_eq!(44, rsq.query(0, 3));
        rsq.update(0, 2, 1);
        // 7 20 19
        assert_eq!(46, rsq.query(0, 3));
        assert_eq!(20, rsq.query(1, 2));
        assert_eq!(39, rsq.query(1, 3));
    }
}
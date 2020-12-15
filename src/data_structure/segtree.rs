#![allow(dead_code)]
use cargo_snippet::snippet;


#[snippet("segtree")]
struct SegTree<T>
where
    T: Copy,
{
    size: usize,
    seg: Vec<T>,
    f: fn(&T, &T) -> T,
    id: T,
}

#[snippet("segtree")]
impl<T> std::fmt::Debug for SegTree<T>
where
    T: Copy + std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SegTree {{ ")?;
        write!(f, "size: {}, vec: {:?} }}", self.size, self.seg)?;
        Ok(())
    }
}

#[snippet("segtree")]
impl<T> SegTree<T>
where
    T: Copy,
{
    fn new(n: usize, f: fn(&T, &T) -> T, id: T) -> SegTree<T> {
        let mut size = 1;
        while n > size { size <<= 1; }
        let seg = vec![id; 2*size];
        Self { size, seg, f, id }
    }

    fn set(&mut self, k: usize, v: T) {
        self.seg[k + self.size] = v;
    }

    fn get(&self, k: usize) -> T {
        self.seg[k + self.size]
    }

    fn build(&mut self) {
        for k in (1..self.size).rev() {
            self.seg[k] = (self.f)(&self.seg[2 * k + 0], &self.seg[2 * k + 1]);
        }
    }

    fn update(&mut self, k: usize, v: T) {
        let mut k = k + self.size;
        self.seg[k] = v;
        while k > 1 {
            self.seg[k >> 1] = (self.f)(&self.seg[k], &self.seg[k^1]);
            k >>= 1;
        }
    }

    fn query(&self, i: usize, j: usize) -> T {
        let mut s = self.id;
        let mut l = i + self.size;
        let mut r = j + self.size;
        while l < r {
            if (l & 1) > 0 {
                s = (self.f)(&s, &self.seg[l]);
                l += 1;
            }
            if (r & 1) > 0{
                s = (self.f)(&s, &self.seg[r - 1]);
            }
            l >>= 1;
            r >>= 1;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_segtree_sum() {
        let mut seg = SegTree::new(
            10, |&a, &b| a + b, 0
        );
        for i in 0..10 {
            seg.update(i, i);
        }
        println!("{:?}", seg);
        assert_eq!(seg.query(0, 9), 36);
        assert_eq!(seg.query(3, 10), 42);
        assert_eq!(seg.query(4, 6), 9);
        seg.update(6, 1);
        assert_eq!(seg.query(0, 9), 31);
        assert_eq!(seg.query(3, 10), 37);
        assert_eq!(seg.query(4, 6), 9);
    }
}

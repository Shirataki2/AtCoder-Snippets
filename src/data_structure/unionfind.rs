#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet("unionfind")]
pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

#[snippet("unionfind")]
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|n| n).collect(),
            sizes: vec![1; n],
            size: n
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
            self.parent[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        if px == py { return false }
        let (large, small) = if self.sizes[px] > self.sizes[py] {
            (px, py)
        } else {
            (py, px)
        };
        self.parent[small] = large;
        self.sizes[large] += self.sizes[small];
        self.sizes[small] = 0;
        self.size -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unionfind_test() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 1);
        uf.unite(0, 2);
        uf.unite(3, 4);
        assert!(uf.find(1) == uf.find(2));
        assert!(uf.find(1) != uf.find(3));
        assert_eq!(uf.sizes.iter().max(), Some(&3));
    }
}
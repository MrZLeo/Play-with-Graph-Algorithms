#![allow(dead_code)]

pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
        }

        Self { parent }
    }

    pub fn find(&mut self, p: usize) -> usize {
        if self.parent[p] != p {
            self.parent[p] = self.find(self.parent[p]);
        }
        self.parent[p]
    }

    pub fn is_connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let l = self.find(p);
        let r = self.find(q);

        if l == r {
            return;
        }
        self.parent[l] = r;
    }
}

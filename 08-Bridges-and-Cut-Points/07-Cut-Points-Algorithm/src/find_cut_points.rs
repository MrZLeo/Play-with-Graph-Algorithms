use std::collections::HashSet;

use crate::graph::Graph;

/// # Interface of find cur points
///
/// Must provide a special `fn bridges()` to achieve bridges of one graph
pub trait FindCutPoints {
    fn cut_points(&mut self) -> Vec<usize>;
}

pub struct FindCutPointsImpl {
    g: Graph,
    visited: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    cnt: usize,
    res: HashSet<usize>,
    calculated: bool,
}

impl FindCutPointsImpl {
    pub fn new(g: Graph) -> Self {
        let v = g.v();
        Self {
            g,
            visited: vec![false; v],
            ord: vec![0; v],
            low: vec![0; v],
            cnt: 0,
            res: HashSet::new(),
            calculated: false,
        }
    }

    fn dfs(&mut self, v: usize, parent: usize) {
        self.visited[v] = true;
        self.ord[v] = self.cnt;
        self.low[v] = self.cnt;
        self.cnt += 1;

        let mut child = 0;
        for w in self.g.adj_edge(v) {
            if !self.visited[w] {
                self.dfs(w, v);
                self.low[v] = usize::min(self.low[v], self.low[w]);
                if v != parent && self.low[w] >= self.ord[v] {
                    self.res.insert(v);
                }
                child += 1;
                if v == parent && child > 1 {
                    self.res.insert(v);
                }
            } else if w != parent {
                self.low[v] = usize::min(self.low[v], self.low[w]);
            }
        }
    }
}

impl FindCutPoints for FindCutPointsImpl {
    fn cut_points(&mut self) -> Vec<usize> {
        if !self.calculated {
            for w in 0..self.g.v() {
                if !self.visited[w] {
                    self.dfs(w, w);
                }
            }
            self.calculated = true;
        }

        self.res.clone().into_iter().collect()
    }
}

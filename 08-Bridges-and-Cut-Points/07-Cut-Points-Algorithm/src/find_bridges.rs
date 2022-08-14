use crate::{edge::Edge, graph::Graph};

/// # Interface of find bridges
///
/// Must provide a special `fn bridges()` to achieve bridges of one graph
pub trait FindBridges {
    fn bridges(&mut self) -> Vec<Edge>;
}

pub struct FindBridgesImpl {
    g: Graph,
    visited: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    cnt: usize,
    res: Vec<Edge>,
    calculated: bool,
}

impl FindBridgesImpl {
    pub fn new(g: Graph) -> Self {
        let v = g.v();
        Self {
            g,
            visited: vec![false; v],
            ord: vec![0; v],
            low: vec![0; v],
            cnt: 0,
            res: vec![],
            calculated: false,
        }
    }

    fn dfs(&mut self, v: usize, parent: usize) {
        self.visited[v] = true;
        self.ord[v] = self.cnt;
        self.low[v] = self.cnt;
        self.cnt += 1;

        for w in self.g.adj_edge(v) {
            if !self.visited[w] {
                self.dfs(w, v);
                self.low[v] = usize::min(self.low[v], self.low[w]);
                if self.low[w] > self.ord[v] {
                    // find a bridge
                    self.res.push(Edge::new(v, w));
                }
            } else if w != parent {
                self.low[v] = usize::min(self.low[v], self.low[w]);
            }
        }
    }
}

impl FindBridges for FindBridgesImpl {
    fn bridges(&mut self) -> Vec<Edge> {
        if !self.calculated {
            for w in 0..self.g.v() {
                if !self.visited[w] {
                    self.dfs(w, w);
                }
            }
            self.calculated = true;
        }

        self.res.clone()
    }
}

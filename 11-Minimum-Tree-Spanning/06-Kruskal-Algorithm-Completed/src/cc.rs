use crate::weighted_graph::WeightedGraph;

pub struct Cc {
    g: WeightedGraph,
    visited: Vec<bool>,
    count: usize,
}

impl Cc {
    pub fn count(&mut self) -> usize {
        for v in 0..self.g.v() {
            if !self.visited[v] {
                self.dfs(v);
                self.count += 1;
            }
        }
        self.count
    }

    pub fn new(g: WeightedGraph) -> Self {
        let v = g.v();
        Self {
            g,
            visited: vec![false; v],
            count: 0,
        }
    }

    fn dfs(&mut self, v: usize) {
        self.visited[v] = true;
        for w in self.g.adj_edge(v) {
            if !self.visited[w] {
                self.dfs(w);
            }
        }
    }
}

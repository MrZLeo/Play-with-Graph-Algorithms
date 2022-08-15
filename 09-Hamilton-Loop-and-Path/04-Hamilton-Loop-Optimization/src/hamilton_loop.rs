use crate::graph::Graph;

pub trait HamiltonLoop {
    fn hamilton_loop(&mut self) -> Vec<usize>;
}

pub struct HamiltonLoopImpl {
    g: Graph,
    visited: Vec<bool>,
    pre: Vec<usize>,
    end: Option<usize>,
}

impl HamiltonLoopImpl {
    pub fn new(g: Graph) -> Self {
        let v = g.v();
        let mut hl = Self {
            g,
            visited: vec![false; v],
            pre: vec![0; v],
            end: None,
        };
        hl.dfs(0, 0, v);
        hl
    }

    fn dfs(&mut self, v: usize, parent: usize, left: usize) -> bool {
        self.visited[v] = true;
        self.pre[v] = parent;
        if left == 1 && self.g.has_edge(v, 0) {
            // v is the last one to visit
            self.end = Some(v);
            return true;
        }

        for w in self.g.adj_edge(v) {
            if !self.visited[w] && self.dfs(w, v, left - 1) {
                return true;
            }
        }

        self.visited[v] = false;
        false
    }
}

impl HamiltonLoop for HamiltonLoopImpl {
    fn hamilton_loop(&mut self) -> Vec<usize> {
        if let Some(mut end) = self.end {
            // find a loop
            let mut res = Vec::with_capacity(self.g.v());
            while end != 0 {
                res.push(end);
                end = self.pre[end];
            }
            res.push(0);
            res.reverse();
            res
        } else {
            vec![]
        }
    }
}

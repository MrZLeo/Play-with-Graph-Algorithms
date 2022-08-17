use crate::graph::Graph;

pub trait EulerLoop {
    fn has_loop(&self) -> bool;
    fn euler_loop(&self) -> Vec<usize>;
}

pub struct EulerLoopImpl {
    g: Graph,
}

impl EulerLoopImpl {
    pub fn new(g: Graph) -> Self {
        Self { g }
    }

    fn dfs(&self, visited: &mut Vec<bool>, v: usize) {
        visited[v] = true;
        for w in self.g.adj_edge(v) {
            if !visited[w] {
                self.dfs(visited, w);
            }
        }
    }
}

impl EulerLoop for EulerLoopImpl {
    fn has_loop(&self) -> bool {
        // check CC
        let mut visited = vec![false; self.g.v()];
        self.dfs(&mut visited, 0);
        for v in visited {
            if !v {
                return false; // not CC
            }
        }

        for v in 0..self.g.v() {
            if self.g.degree(v) % 2 != 0 {
                return false; // odd degree
            }
        }

        true
    }

    fn euler_loop(&self) -> Vec<usize> {
        if !self.has_loop() {
            return vec![];
        }

        let mut res = Vec::with_capacity(self.g.e() + 1);
        let mut g = self.g.clone();
        let mut stack = vec![];
        let mut curv = 0;
        // important, push curv first but this curv will not push into res
        stack.push(curv);
        while !stack.is_empty() {
            if g.degree(curv) != 0 {
                stack.push(curv);
                let w = g.adj_edge(curv).next().unwrap();
                g.remove_edge(curv, w);
                curv = w;
            } else {
                res.push(curv);
                curv = stack.pop().unwrap();
            }
        }

        res
    }
}

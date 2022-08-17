use crate::graph::Graph;

trait EulerLoop {
    fn has_loop(&self) -> bool;
}

struct EulerLoopImpl {
    g: Graph,
}

impl EulerLoopImpl {
    fn new(g: Graph) -> Self {
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
}

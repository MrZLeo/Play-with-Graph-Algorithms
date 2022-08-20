use crate::graph::Graph;

pub struct DirectedCycleDetection {
    g: Graph,
    visited: Vec<bool>,
    path: Vec<bool>,
    has_cycle: bool,
}

impl DirectedCycleDetection {
    pub fn new(g: Graph) -> Self {
        let visited = vec![false; g.v()];
        let path = vec![false; g.v()];
        let has_cycle = false;
        let mut dcd = Self {
            g,
            visited,
            path,
            has_cycle,
        };

        for v in 0..dcd.g.v() {
            if !dcd.visited[v] && dcd.dfs(v) {
                break;
            }
        }

        dcd
    }

    fn dfs(&mut self, v: usize) -> bool {
        self.visited[v] = true;
        self.path[v] = true;

        for w in self.g.adj_edge(v) {
            if !self.visited[w] {
                if self.dfs(w) {
                    return true;
                }
            } else if self.path[w] {
                self.has_cycle = true;
                return true;
            }
        }
        self.path[v] = false;
        false
    }

    pub fn has_cycle(&self) -> bool {
        self.has_cycle
    }
}

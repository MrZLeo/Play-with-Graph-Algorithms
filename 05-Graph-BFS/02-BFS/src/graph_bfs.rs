use std::collections::VecDeque;

use crate::graph::Graph;

/// Interface: Bfs
/// we need to provide a `bfs` method to do bfs search and returen iterator of
/// bfs search result
pub trait Bfs {
    fn bfs(&mut self, s: usize) -> Box<dyn Iterator<Item = usize>>;
}

pub struct GraphBFS {
    graph: Graph,
    visited: Vec<bool>,
    res: Vec<usize>,
}

impl GraphBFS {
    pub fn new(graph: Graph) -> Self {
        let visited = vec![false; graph.v()];
        let res = Vec::new();
        Self {
            graph,
            visited,
            res,
        }
    }
}

impl Bfs for GraphBFS {
    fn bfs(&mut self, source: usize) -> Box<dyn Iterator<Item = usize>> {
        // inner bfs, start from s
        let mut bfs_inner = |s| {
            // check if `s` is visited before
            // MUST do this in `bfs_inner` because we have mutable access inside
            // closure, so we can't have a immutable access outside
            if self.visited[s] {
                return;
            }

            let mut q = VecDeque::new();
            q.push_back(s);
            self.visited[s] = true;

            while !q.is_empty() {
                let v = q.pop_front().unwrap();
                self.res.push(v);
                for &next in self.graph.adj_edge(v) {
                    if !self.visited[next] {
                        q.push_back(next);
                        self.visited[next] = true;
                    }
                }
            }
        };

        // we should search first from s
        // and if there's some v we can't reach, search latter
        bfs_inner(source);

        for v in 0..self.graph.v() {
            bfs_inner(v);
        }

        Box::new(self.res.clone().into_iter())
    }
}

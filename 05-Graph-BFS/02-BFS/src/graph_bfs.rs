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
    fn bfs(&mut self, s: usize) -> Box<dyn Iterator<Item = usize>> {
        let mut q = VecDeque::new();
        q.push_back(s);
        self.visited[s] = true;

        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            // println!("{v}");
            self.res.push(v);
            for next in self.graph.adj_edge(v) {
                if !self.visited[next] {
                    q.push_back(next);
                    self.visited[next] = true;
                }
            }
        }

        Box::new(self.res.clone().into_iter())
    }
}

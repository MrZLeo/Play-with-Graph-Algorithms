use std::collections::VecDeque;

use crate::{
    graph::Graph,
    unweighted_shortest_path::{SingleSourcePath, Usssp},
};

pub struct GraphBFS {
    source: usize,
    graph: Graph,
    visited: Vec<bool>,
    pre: Vec<usize>,
    dis: Vec<i32>,
    is_computed: bool,
}

impl GraphBFS {
    pub fn new(source: usize, graph: Graph) -> Self {
        let visited = vec![false; graph.v()];
        let pre = vec![source; graph.v()];
        let dis = vec![-1; graph.v()];
        Self {
            source,
            graph,
            visited,
            pre,
            dis,
            is_computed: false,
        }
    }
}

impl SingleSourcePath for GraphBFS {
    fn single_source_path(&mut self) {
        let mut q = VecDeque::new();
        let s = self.source;
        q.push_back(s);
        self.visited[s] = true;
        self.pre[s] = s;
        self.dis[s] = 0;

        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            for &next in self.graph.adj_edge(v) {
                if !self.visited[next] {
                    q.push_back(next);
                    self.visited[next] = true;
                    self.pre[next] = v;
                    self.dis[next] = self.dis[v] + 1;
                }
            }
        }
        self.is_computed = true;

        // just for debug, print `dis`
        for i in &self.dis {
            print!("{i} ");
        }
        println!();
    }

    fn is_connected(&mut self, t: usize) -> bool {
        self.graph.validate_vertex(t);
        if !self.is_computed {
            self.single_source_path();
        }
        self.visited[t]
    }

    fn path(&mut self, t: usize) -> Box<dyn Iterator<Item = usize>> {
        self.graph.validate_vertex(t);
        if !self.is_computed {
            self.single_source_path();
        }

        let mut res = Vec::new();
        if !self.is_connected(t) {
            return Box::new(res.into_iter());
        }

        let mut cur = t;
        while cur != self.source {
            res.push(cur);
            cur = self.pre[cur];
        }
        res.push(self.source);
        res.reverse();

        Box::new(res.into_iter())
    }
}

impl Usssp for GraphBFS {
    fn dis(&mut self, t: usize) -> i32 {
        if !self.is_computed {
            self.single_source_path();
        }
        self.graph.validate_vertex(t);
        self.dis[t]
    }
}

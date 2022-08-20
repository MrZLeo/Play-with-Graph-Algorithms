#![allow(dead_code)]

use crate::graph::Graph;
use std::collections::VecDeque;

pub struct TopoSort {
    g: Graph,
    res: Vec<usize>,
    indegree: Vec<usize>,
    has_cycle: bool,
}

impl TopoSort {
    pub fn new(g: Graph) -> Self {
        let mut res = Vec::with_capacity(g.v());
        let mut indegree = Vec::with_capacity(g.v());
        let mut has_cycle = false;

        for v in 0..g.v() {
            indegree.push(g.in_degree(v));
        }

        let mut q = VecDeque::new();
        indegree
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 0)
            .for_each(|(i, _)| q.push_back(i));

        while let Some(v) = q.pop_front() {
            res.push(v);
            for w in g.adj_edge(v) {
                indegree[w] -= 1;
                if indegree[w] == 0 {
                    q.push_back(w);
                }
            }
        }

        if res.len() < g.v() {
            res.clear();
            has_cycle = true;
        }

        Self {
            g,
            res,
            indegree,
            has_cycle,
        }
    }

    pub fn has_cycle(&self) -> bool {
        self.has_cycle
    }

    pub fn result(&self) -> Vec<usize> {
        self.res.clone()
    }
}

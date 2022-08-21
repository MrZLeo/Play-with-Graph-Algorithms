use std::collections::VecDeque;

use crate::weighted_graph::WeightedGraph;

pub struct MaxFlow {
    rg: WeightedGraph,
    s: usize,
    t: usize,
    maxflow: usize,
}

impl MaxFlow {
    pub fn new(g: WeightedGraph, s: usize, t: usize) -> Self {
        if !g.directed() {
            panic!("Only support directed graph");
        }
        if g.v() < 2 {
            panic!("Graph should has more than one vertex");
        }
        g.validate_vertex(s);
        g.validate_vertex(t);

        if s == t {
            panic!("start vertex can't be the same as end vertex");
        }

        let rg = WeightedGraph::new_rg(&g);
        let mut q = VecDeque::new();
        q.push_back(s);
        let maxflow = 0;
        let mut this = Self { rg, s, t, maxflow };

        while let Some(path) = this.get_augementing_path() {
            let mut f = i32::MAX;
            for i in 1..path.len() {
                let v = path[i - 1];
                let w = path[i];
                f = i32::min(f, this.rg.get_weight(v, w).unwrap());
            }
            this.maxflow += f as usize;
            for i in 1..path.len() {
                let v = path[i - 1];
                let w = path[i];
                this.rg
                    .set_weight(v, w, this.rg.get_weight(v, w).unwrap() - f as i32);
                this.rg
                    .set_weight(w, v, this.rg.get_weight(w, v).unwrap() + f as i32);
            }
        }

        this
    }

    fn get_augementing_path(&mut self) -> Option<Vec<usize>> {
        let mut pre = vec![-1; self.rg.v()];
        let mut q = VecDeque::new();
        q.push_back(self.s);
        pre[self.s] = self.s as i32;
        while let Some(v) = q.pop_front() {
            if v == self.t {
                break;
            }
            for w in self.rg.adj_edge(v) {
                if pre[w] == -1 && self.rg.get_weight(v, w).unwrap() > 0 {
                    q.push_back(w);
                    pre[w] = v as i32;
                }
            }
        }

        if pre[self.t] == -1 {
            return None;
        }

        let mut res = vec![];
        let mut cur = self.t;
        while cur as usize != self.s {
            res.push(cur);
            cur = pre[cur] as usize;
        }
        res.push(self.s);
        res.reverse();
        Some(res)
    }

    pub fn maxflow(&self) -> usize {
        self.maxflow
    }

    pub fn get_flow(&self, v: usize, w: usize) -> Option<i32> {
        if self.rg.has_edge(v, w) {
            return Some(self.rg.get_weight(w, v).unwrap());
        }
        None
    }
}

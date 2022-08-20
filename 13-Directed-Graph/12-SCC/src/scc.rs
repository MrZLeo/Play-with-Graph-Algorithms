#![allow(dead_code)]

use crate::{dfs::Dfs, graph::Graph};

#[derive(Debug)]
pub struct Scc {
    g: Graph,
    visited: Vec<i32>,
    cnt: usize,
}

impl Scc {
    pub fn new(mut g: Graph) -> Self {
        let visited = vec![-1; g.v()];
        let cnt = 0;

        let mut order = Dfs::new(g.new_reverse()).post();
        order.reverse();

        let mut this = Self { g, visited, cnt };

        for v in order {
            if this.visited[v] == -1 {
                this.dfs(v, this.cnt as i32);
                this.cnt += 1;
            }
        }

        this
    }

    fn dfs(&mut self, v: usize, cnt: i32) {
        self.visited[v] = cnt;
        for w in self.g.adj_edge(v) {
            if self.visited[w] == -1 {
                self.dfs(w, cnt);
            }
        }
    }

    pub fn strong_connected(&self, v: usize, w: usize) -> bool {
        self.visited[v] == self.visited[w]
    }

    pub fn component(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.cnt];
        for v in 0..self.g.v() {
            res[self.visited[v] as usize].push(v);
        }
        res
    }
}

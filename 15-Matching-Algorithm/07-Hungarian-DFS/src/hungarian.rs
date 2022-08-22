use std::collections::VecDeque;

use crate::{bipartition, graph::Graph};

pub struct Hungarian {
    g: Graph,
    max_matching: usize,
    matching: Vec<i32>,
}

impl Hungarian {
    pub fn new(g: Graph) -> Self {
        let (is_bd, colors) = bipartition::is_bipartition(&g);
        if !is_bd {
            panic!("Only works for bipartition graph");
        }
        let colors = colors.unwrap();
        let matching = vec![-1; g.v()];
        let mut this = Self {
            g,
            max_matching: 0,
            matching,
        };

        for v in 0..this.g.v() {
            if colors[v] == 0 && this.matching[v] == -1 {
                let mut visited = vec![false; this.g.v()];
                if this.dfs(v, &mut visited) {
                    this.max_matching += 1;
                }
            }
        }

        this
    }

    fn dfs(&mut self, v: usize, visited: &mut [bool]) -> bool {
        visited[v] = true;
        for w in self.g.adj_edge(v) {
            if visited[w] {
                continue;
            }
            visited[w] = true;
            if self.matching[w] == -1 || self.dfs(w, visited) {
                self.matching[w] = v as i32;
                self.matching[v] = w as i32;
                return true;
            }
        }

        false
    }

    fn get_augementing_path(pre: &[i32], v: usize, w: usize) -> Vec<usize> {
        let mut res = Vec::new();
        let mut cur = w;
        while cur != v {
            res.push(cur);
            cur = pre[cur] as usize;
        }
        res.push(v);
        res
    }

    pub fn max_matching(&self) -> usize {
        self.max_matching
    }

    pub fn is_perfect_matching(&self) -> bool {
        self.max_matching * 2 == self.g.v()
    }
}

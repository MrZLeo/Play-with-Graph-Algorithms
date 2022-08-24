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
            if colors[v] == 0 && this.matching[v] == -1 && this.bfs(v) {
                this.max_matching += 1;
            }
        }

        this
    }

    fn bfs(&mut self, v: usize) -> bool {
        let mut q = VecDeque::new();
        let mut pre = vec![-1; self.g.v()];
        q.push_back(v);
        pre[v] = v as i32;
        while let Some(v) = q.pop_front() {
            for next in self.g.adj_edge(v) {
                if pre[next] != -1 {
                    continue;
                }
                if self.matching[next] != -1 {
                    q.push_back(self.matching[next] as usize);
                    pre[next] = v as i32;
                    pre[self.matching[next] as usize] = next as i32;
                } else {
                    pre[next] = v as i32;
                    let path = Self::get_augementing_path(&pre, v, next);
                    for i in (0..path.len()).step_by(2) {
                        self.matching[path[i]] = path[i + 1] as i32;
                        self.matching[path[i + 1]] = path[i] as i32;
                    }
                    return true; // find a path
                }
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

#![allow(dead_code)]

use crate::weighted_graph::WeightedGraph;

pub struct BellmanFord {
    g: WeightedGraph,
    dis: Vec<i32>,
    s: usize,
    has_negative_cycle: bool,
}

impl BellmanFord {
    pub fn new(g: WeightedGraph, s: usize) -> Self {
        let mut dis = vec![i32::MAX; g.v()];
        let mut has_negative_cycle = false;
        dis[s] = 0;

        for _pass in 1..g.v() {
            for v in 0..g.v() {
                for w in g.adj_edge(v) {
                    if dis[v] < i32::MAX && dis[v] + g.get_weight(v, w).unwrap() < dis[w] {
                        dis[w] = dis[v] + g.get_weight(v, w).unwrap();
                    }
                }
            }
        }

        // detect negative cycle
        for v in 0..g.v() {
            for w in g.adj_edge(v) {
                if dis[v] < i32::MAX && dis[v] + g.get_weight(v, w).unwrap() < dis[w] {
                    has_negative_cycle = true;
                }
            }
        }

        Self {
            g,
            dis,
            s,
            has_negative_cycle,
        }
    }

    pub fn has_negative_cycle(&self) -> bool {
        self.has_negative_cycle
    }

    pub fn is_connected_to(&self, w: usize) -> bool {
        self.g.validate_vertex(w);
        self.dis[w] != i32::MAX
    }

    pub fn dis_to(&self, w: usize) -> Option<i32> {
        self.g.validate_vertex(w);
        if self.has_negative_cycle {
            // println!("Negative cycle detected");
            None
        } else {
            Some(self.dis[w])
        }
    }
}

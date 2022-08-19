#![allow(dead_code)]

use std::collections::BinaryHeap;

use crate::weighted_graph::WeightedGraph;

pub struct Dijkstra {
    g: WeightedGraph,
    s: usize,
    visited: Vec<bool>,
    dis: Vec<i32>,
}

#[derive(PartialEq, Eq)]
struct Node {
    v: usize,
    dis: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dis.cmp(&self.dis).then_with(|| self.v.cmp(&other.v))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Dijkstra {
    pub fn new(g: WeightedGraph, s: usize) -> Self {
        let mut visited = vec![false; g.v()];
        let mut dis = vec![i32::MAX; g.v()];

        // Dijkstra algorithm
        dis[s] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Node { v: s, dis: 0 });
        while let Some(Node { v, dis: d }) = heap.pop() {
            if visited[v] {
                continue;
            }
            // update others
            visited[v] = true;
            for w in g.adj_edge(v) {
                if !visited[w] && dis[w] > d + g.get_weight(v, w).unwrap() {
                    dis[w] = d + g.get_weight(v, w).unwrap();
                    heap.push(Node { v: w, dis: dis[w] })
                }
            }
        }

        Self { g, s, visited, dis }
    }

    pub fn is_connected_to(&self, w: usize) -> bool {
        self.g.validate_vertex(w);
        self.visited[w]
    }

    pub fn dis_to(&self, w: usize) -> i32 {
        self.g.validate_vertex(w);
        self.dis[w]
    }

    pub fn s(&self) -> usize {
        self.s
    }
}

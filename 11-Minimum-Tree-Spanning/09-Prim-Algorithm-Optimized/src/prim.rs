use std::collections::BinaryHeap;

use crate::cc::Cc;
use crate::weighted_edge::WeightedEdge;
use crate::weighted_graph::WeightedGraph;

pub trait Prim {
    fn mst(&self) -> Vec<WeightedEdge>;
}

pub struct PrimImpl {
    g: WeightedGraph,
}

impl PrimImpl {
    pub fn new(g: WeightedGraph) -> Self {
        Self { g }
    }
}

impl Prim for PrimImpl {
    fn mst(&self) -> Vec<WeightedEdge> {
        let mut cc = Cc::new(self.g.clone());
        if cc.count() > 1 {
            return vec![];
        }

        let mut mst = Vec::with_capacity(self.g.v() + 1);
        let mut visited = vec![false; self.g.v()];
        visited[0] = true;

        let mut heap = BinaryHeap::new();
        for w in self.g.adj_edge(0) {
            heap.push(WeightedEdge::new(0, w, self.g.get_weight(0, w).unwrap()));
        }

        while let Some(edge) = heap.pop() {
            if visited[edge.v()] && visited[edge.w()] {
                continue;
            }

            let newv = {
                if visited[edge.v()] {
                    edge.w()
                } else {
                    edge.v()
                }
            };
            visited[newv] = true;
            for w in self.g.adj_edge(newv) {
                if !visited[w] {
                    heap.push(WeightedEdge::new(
                        newv,
                        w,
                        self.g.get_weight(newv, w).unwrap(),
                    ));
                }
            }
            mst.push(edge);
        }
        mst
    }
}

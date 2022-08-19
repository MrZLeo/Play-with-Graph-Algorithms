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
        for _i in 1..self.g.v() {
            let mut min_edge = WeightedEdge::new(self.g.v(), self.g.v(), i32::MAX);
            for v in 0..self.g.v() {
                if visited[v] {
                    for w in self.g.adj_edge(v) {
                        if !visited[w] && self.g.get_weight(v, w).unwrap() < min_edge.weight() {
                            min_edge = WeightedEdge::new(v, w, self.g.get_weight(v, w).unwrap());
                        }
                    }
                }
            }
            visited[min_edge.v()] = true;
            visited[min_edge.w()] = true;
            mst.push(min_edge);
        }

        mst
    }
}

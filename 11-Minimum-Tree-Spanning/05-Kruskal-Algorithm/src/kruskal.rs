use crate::{cc::Cc, weighted_edge::WeightedEdge, weighted_graph::WeightedGraph};

pub trait Kruskal {
    fn mst(&self) -> Vec<WeightedEdge>;
}

pub struct KruskalImpl {
    g: WeightedGraph,
}

impl KruskalImpl {
    pub fn new(g: WeightedGraph) -> Self {
        Self { g }
    }
}

impl Kruskal for KruskalImpl {
    fn mst(&self) -> Vec<WeightedEdge> {
        let mut mst = Vec::with_capacity(self.g.v() + 1);

        let mut cc = Cc::new(self.g.clone());
        if cc.count() > 1 {
            return vec![];
        }

        let mut edges = Vec::with_capacity(self.g.e());
        for v in 0..self.g.v() {
            for w in self.g.adj_edge(v) {
                if v < w {
                    edges.push(WeightedEdge::new(v, w, self.g.get_weight(v, w).unwrap()));
                }
            }
        }
        edges.sort_unstable_by_key(|a| a.weight());

        mst
    }
}

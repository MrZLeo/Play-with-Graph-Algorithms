use crate::cc::Cc;
use crate::union_find::UnionFind;
use crate::weighted_edge::WeightedEdge;
use crate::weighted_graph::WeightedGraph;

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

        let mut mst = Vec::with_capacity(self.g.v() + 1);
        let mut union_find = UnionFind::new(self.g.v());
        for edge in edges {
            let v = edge.v();
            let w = edge.w();
            if !union_find.is_connected(v, w) {
                mst.push(edge);
                union_find.union(v, w);
            }
        }

        mst
    }
}

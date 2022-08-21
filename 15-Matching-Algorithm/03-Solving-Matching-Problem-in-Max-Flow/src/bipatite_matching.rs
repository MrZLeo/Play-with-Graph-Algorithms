use crate::{bipartition, graph::Graph, max_flow::MaxFlow, weighted_graph::WeightedGraph};

pub struct BipartiteMatching {
    g: Graph,
    max_matching: usize,
}

impl BipartiteMatching {
    pub fn new(g: Graph) -> Self {
        let (is_bd, colors) = bipartition::is_bipartition(&g);
        if !is_bd {
            panic!("Only works for bipartition graph");
        }
        let colors = colors.unwrap();

        let mut network = WeightedGraph::new(g.v() + 2, true);
        for v in 0..g.v() {
            if colors[v] == 0 {
                network.add_edge(g.v(), v, 1);
            } else {
                network.add_edge(v, g.v() + 1, 1);
            }
            for w in g.adj_edge(v) {
                if v < w {
                    if colors[v] == 0 {
                        network.add_edge(v, w, 1);
                    } else {
                        network.add_edge(w, v, 1);
                    }
                }
            }
        }

        let mf = MaxFlow::new(network, g.v(), g.v() + 1);
        let max_matching = mf.maxflow();

        Self { g, max_matching }
    }

    pub fn max_matching(&self) -> usize {
        self.max_matching
    }

    pub fn is_perfect_matching(&self) -> bool {
        self.max_matching * 2 == self.g.v()
    }
}

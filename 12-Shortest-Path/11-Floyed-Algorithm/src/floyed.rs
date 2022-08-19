use crate::weighted_graph::WeightedGraph;

pub struct Floyed {
    g: WeightedGraph,
    dis: Vec<Vec<i32>>,
    has_nagative_cycle: bool,
}

impl Floyed {
    pub fn new(g: WeightedGraph) -> Self {
        let mut dis = vec![vec![i32::MAX; g.v()]; g.v()];
        let mut has_nagative_cycle = false;

        for v in 0..g.v() {
            dis[v][v] = 0;
            for w in g.adj_edge(v) {
                dis[v][w] = g.get_weight(v, w).unwrap();
            }
        }

        for t in 0..g.v() {
            for v in 0..g.v() {
                for w in 0..g.v() {
                    if dis[v][t] < i32::MAX
                        && dis[t][w] < i32::MAX
                        && dis[v][t] + dis[t][w] < dis[v][w]
                    {
                        dis[v][w] = dis[v][t] + dis[t][w];
                    }
                }
            }
        }

        for v in 0..g.v() {
            if dis[v][v] < 0 {
                has_nagative_cycle = true;
            }
        }

        Self {
            g,
            dis,
            has_nagative_cycle,
        }
    }

    pub fn has_nagative_cycle(&self) -> bool {
        self.has_nagative_cycle
    }

    pub fn is_connected_to(&self, v: usize, w: usize) -> bool {
        self.g.validate_vertex(v);
        self.g.validate_vertex(w);
        self.dis[v][w] == i32::MAX
    }

    pub fn dis_to(&self, v: usize, w: usize) -> Option<i32> {
        self.g.validate_vertex(v);
        self.g.validate_vertex(w);

        if self.has_nagative_cycle {
            None
        } else {
            Some(self.dis[v][w])
        }
    }
}

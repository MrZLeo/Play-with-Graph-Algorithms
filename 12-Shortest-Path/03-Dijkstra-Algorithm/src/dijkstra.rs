use crate::weighted_graph::WeightedGraph;

pub struct Dijkstra {
    g: WeightedGraph,
    s: usize,
    visited: Vec<bool>,
    dis: Vec<i32>,
}

impl Dijkstra {
    pub fn new(g: WeightedGraph, s: usize) -> Self {
        let mut visited = vec![false; g.v()];
        let mut dis = vec![i32::MAX; g.v()];

        // Dijkstra algorithm
        dis[0] = 0;
        loop {
            // find out the minimum point
            let mut cur = -1;
            let mut min = i32::MAX;
            for v in 0..g.v() {
                if !visited[v] && dis[v] < min {
                    cur = v as i32;
                    min = dis[v];
                }
            }

            if cur == -1 {
                break;
            }

            // update others
            let cur = cur as usize;
            visited[cur] = true;
            for w in g.adj_edge(cur) {
                if !visited[w] && dis[w] > dis[cur] + g.get_weight(cur, w).unwrap() {
                    dis[w] = dis[cur] + g.get_weight(cur, w).unwrap();
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

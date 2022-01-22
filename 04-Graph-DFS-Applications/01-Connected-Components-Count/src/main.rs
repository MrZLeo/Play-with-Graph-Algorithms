mod graph;

mod cc {

    use crate::graph::Graph;

    pub fn count(g: &Graph) -> i32 {
        let mut visited = vec![false; g.v()];
        let mut cc_count = 0;
        for v in 0..g.v() {
            if visited[v] == false {
                __count(&g, v, &mut visited);
                cc_count += 1;
            }
        }
        return cc_count;
    }

    fn __count(g: &Graph, v: usize, visited: &mut Vec<bool>) {
        visited[v] = true;

        for &w in g.adj_edge(v) {
            if visited[w] == false {
                __count(g, w, visited);
            }
        }
    }
}

use crate::{cc::count, graph::Graph};

pub fn main() {}

#[test]
pub fn cc_test() {
    let g = Graph::from("g.txt");
    assert_eq!(cc::count(&g), 2);
}

mod graph;

mod cc {

    use crate::graph::Graph;

    pub fn count(g: &Graph) -> i32 {
        let mut visited = vec![-1; g.v()];
        let mut cc_count = 0;
        for v in 0..g.v() {
            if visited[v] == -1 {
                __count(&g, v, &mut visited, cc_count);
                cc_count += 1;
            }
        }

        println!("{:?}", visited);
        return cc_count;
    }

    fn __count(g: &Graph, v: usize, visited: &mut Vec<i32>, ccid: i32) {
        visited[v] = ccid;

        for &w in g.adj_edge(v) {
            if visited[w] == -1 {
                __count(g, w, visited, ccid);
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

mod graph;

mod cycle_detection {

    use crate::graph::Graph;

    pub fn has_cycle(g: &Graph) -> bool {
        let mut visited = vec![false; g.v()];
        for v in 0..g.v() {
            if visited[v] == false {
                if __dfs(&g, v, &mut visited, v) {
                    return true;
                }
            }
        }

        return false;
    }


    fn __dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, pre: usize) -> bool {
        visited[v] = true;

        for &w in g.adj_edge(v) {
            if visited[w] == false {
                __dfs(g, w, visited, v);
            } else if w != pre {
                return true;
            }
        }

        return false;
    }

}

use crate::{graph::Graph, cycle_detection::has_cycle};

pub fn main() {}

#[test]
pub fn cycle_detection_test() {
    let g = Graph::from("g.txt");
    let g2 = Graph::from("g2.txt");

    assert_eq!(has_cycle(&g), true);
    assert_eq!(has_cycle(&g2), false);
}

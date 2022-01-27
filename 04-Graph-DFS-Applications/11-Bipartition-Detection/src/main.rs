mod graph;

mod bipartition_detection {

    use crate::graph::Graph;

    /// `colors`: 表示颜色
    ///     - 0 -> blue
    ///     - 1 -> green
    pub fn is_bipartition(g: &Graph) -> bool {
        let mut visited = vec![false; g.v()];
        let mut colors = vec![-1; g.v()];
        for v in 0..g.v() {
            if visited[v] == false {
                if __dfs(&g, v, &mut visited, &mut colors, 0) == false {
                    return false;
                }
            }
        }

        return true;
    }

    fn __dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, colors: &mut Vec<i32>, color: i32) -> bool {
        visited[v] = true;
        colors[v] = color;

        for &w in g.adj_edge(v) {
            if visited[w] == false {
                if __dfs(g, w, visited, colors, 1 - color) == false {
                    return false;
                }
            } else if colors[w] == colors[v] {
                return false;
            }
        }

        return true;
    }
}

use crate::{bipartition_detection::is_bipartition, graph::Graph};

pub fn main() {}

#[test]
pub fn cycle_detection_test() {
    let g = Graph::from("g.txt");
    let g2 = Graph::from("g2.txt");
    let g3 = Graph::from("g3.txt");

    assert_eq!(is_bipartition(&g), true);
    assert_eq!(is_bipartition(&g2), false);
    assert_eq!(is_bipartition(&g3), true);
}

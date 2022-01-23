mod graph;

mod cc {

    use crate::graph::Graph;

    pub fn dfs(g: &Graph) -> (Vec<i32>, i32) {
        let mut visited = vec![-1; g.v()];
        let mut cc_count = 0;
        for v in 0..g.v() {
            if visited[v] == -1 {
                __dfs(&g, v, &mut visited, cc_count);
                cc_count += 1;
            }
        }

        return (visited, cc_count);
    }

    fn __dfs(g: &Graph, v: usize, visited: &mut Vec<i32>, ccid: i32) {
        visited[v] = ccid;

        for &w in g.adj_edge(v) {
            if visited[w] == -1 {
                __dfs(g, w, visited, ccid);
            }
        }
    }

    pub fn component(g: &Graph) -> Vec<Vec<usize>> {
        let (visited, cnt) = dfs(g);
        let mut res: Vec<Vec<usize>> = vec![vec![]; cnt as usize];

        for v in 0..g.v() {
            res[visited[v] as usize].push(v);
        }

        return res;
    }

    pub fn is_connected(g: &Graph, v: usize, w: usize) -> bool {
        let (visited, _) = dfs(g);

        visited[v] == visited[w]
    }
}

use crate::{cc::dfs, graph::Graph};

pub fn main() {}

#[test]
pub fn cc_test() {
    let g = Graph::from("g.txt");
    assert_eq!(cc::dfs(&g).1, 2);
    assert_eq!(cc::is_connected(&g, 0, 6), true);
    assert_eq!(cc::is_connected(&g, 0, 5), false);
    let gc = cc::component(&g);
    for i in 0..gc.len() {
        print!("{}: ", i);
        for v in &gc[i] {
            print!("{} ", v);
        }
        println!();
    }
}

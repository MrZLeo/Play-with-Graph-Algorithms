mod graph;

mod graph_dfs {

    use crate::graph::Graph;

    pub fn dfs(g: &Graph) -> Vec<usize> {
        let mut visited = vec![false; g.v()];
        let mut res = Vec::new();
        for v in 0..g.v() {
            if visited[v] == false {
                __dfs(&g, v, &mut visited, &mut res);
            }
        }
        return res;
    }

    pub fn post_dfs(g: &Graph) -> Vec<usize> {
        let mut visited = vec![false; g.v()];
        let mut res = Vec::new();
        for v in 0..g.v() {
            if visited[v] == false {
                __post_dfs(&g, v, &mut visited, &mut res);
            }
        }
        return res;
    }

    fn __dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, res: &mut Vec<usize>) {
        visited[v] = true;
        res.push(v);

        for &w in g.adj_edge(v) {
            if visited[w] == false {
                __dfs(g, w, visited, res);
            }
        }
    }

    fn __post_dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, res: &mut Vec<usize>) {
        visited[v] = true;

        for &w in g.adj_edge(v) {
            if visited[w] == false {
                __post_dfs(g, w, visited, res);
            }
        }

        res.push(v);
    }
}

use crate::{
    graph::Graph,
    graph_dfs::{dfs, post_dfs},
};

pub fn main() {
    let g = Graph::from("g.txt");
    let res = dfs(&g);
    println!("{:?}", res);

    let res2 = post_dfs(&g);
    println!("{:?}", res2);
}

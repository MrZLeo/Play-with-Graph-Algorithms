mod graph;

mod graph_dfs {

    use crate::graph::Graph;

    pub fn dfs(g: &Graph) -> Vec<usize> {
        let mut visited = vec![false; g.v()];
        let mut res = Vec::new();
        __dfs(&g, 0, &mut visited, &mut res);
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
}

use crate::{graph::Graph, graph_dfs::dfs};

pub fn main() {
    let g = Graph::from("g.txt");
    let res = dfs(&g);
    println!("{:?}", res);
}

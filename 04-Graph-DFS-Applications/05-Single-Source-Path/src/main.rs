mod graph;

mod single_source_path {

    use crate::graph::Graph;

    pub fn dfs(g: &Graph, s: usize) -> Vec<i32> {
        let mut visited = vec![false; g.v()];
        let mut pre = vec![-1; g.v()];

        __dfs(g, s, &mut visited, &mut pre, s);
        pre
    }

    fn __dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, pre: &mut Vec<i32>, pre_v: usize) {
        visited[v] = true;
        pre[v] = pre_v as i32;

        for &w in g.adj_edge(v) {
            if !visited[w] {
                __dfs(g, w, visited, pre, v);
            }
        }
    }

    pub fn is_connected_to(g: &Graph, s: usize, w: usize) -> bool {
        let pre = dfs(g, s);
        pre[w] != -1
    }

    pub fn path(g: &Graph, s: usize, w: usize) -> Vec<usize> {
        let pre = dfs(g, s);
        let mut res = Vec::new();

        if pre[w] == -1 {
            return res;
        }

        let mut cur = w;
        while cur != s {
            res.push(cur);
            cur = pre[cur] as usize;
        }
        res.push(s);
        res.reverse();

        res
    }
}

use crate::{
    graph::Graph,
    single_source_path::{is_connected_to, path},
};

pub fn main() {
    let g = Graph::from("g.txt");
    println!("{}", is_connected_to(&g, 0, 6));
    println!("{}", is_connected_to(&g, 0, 5));
    println!("0 -> 6: {:?}", path(&g, 0, 6));
    println!("0 -> 5: {:?}", path(&g, 0, 5));
}

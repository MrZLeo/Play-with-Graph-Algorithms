use crate::graph::Graph;

/// `colors`: 表示颜色
///     - 0 -> blue
///     - 1 -> green
pub fn is_bipartition(g: &Graph) -> (bool, Option<Vec<i32>>) {
    let mut visited = vec![false; g.v()];
    let mut colors = vec![-1; g.v()];
    for v in 0..g.v() {
        if !visited[v] && !__dfs(g, v, &mut visited, &mut colors, 0) {
            return (false, None);
        }
    }

    (true, Some(colors))
}

fn __dfs(g: &Graph, v: usize, visited: &mut Vec<bool>, colors: &mut Vec<i32>, color: i32) -> bool {
    visited[v] = true;
    colors[v] = color;

    for w in g.adj_edge(v) {
        if !visited[w] {
            if !__dfs(g, w, visited, colors, 1 - color) {
                return false;
            }
        } else if colors[w] == colors[v] {
            return false;
        }
    }

    true
}

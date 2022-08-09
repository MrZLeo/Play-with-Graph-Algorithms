pub struct Solution;
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let V = graph.len();
        let mut visited = vec![false; V];
        let mut color = vec![-1; V];

        for v in 0..V {
            if !visited[v] && !Solution::dfs(v as i32, 0, &graph, &mut visited, &mut color) {
                return false;
            }
        }

        true
    }

    fn dfs(
        v: i32,
        color: i32,
        graph: &Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
        colors: &mut Vec<i32>,
    ) -> bool {
        // chang to usize inorder to index the vec
        let v = v as usize;

        visited[v] = true;
        colors[v] = color;

        for &w in &graph[v] {
            let w = w as usize;
            if !visited[w] {
                if !Solution::dfs(w as i32, 1 - color, graph, visited, colors) {
                    return false;
                }
            } else {
                if colors[v] == colors[w] {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {}

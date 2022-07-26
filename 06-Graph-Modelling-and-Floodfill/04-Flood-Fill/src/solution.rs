use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let dir = vec![vec![-1, 0], vec![0, 1], vec![1, 0], vec![0, -1]];
        let r = grid.len();
        let c = grid[0].len();
        let mut g = vec![HashSet::new(); r * c];
        let mut visited = vec![false; g.len()];

        let in_area = |x, y| -> bool { x >= 0 && y >= 0 && x < r as i32 && y < c as i32 };

        let mut construct_graph = || {
            for v in 0..g.len() {
                let x = v / c;
                let y = v % c;
                if grid[x][y] == 1 {
                    for d in &dir {
                        let nextx = x as i32 + d[0];
                        let nexty = y as i32 + d[1];
                        if in_area(nextx, nexty) && grid[nextx as usize][nexty as usize] == 1 {
                            let next = nextx as usize * c + nexty as usize;
                            g[v].insert(next);
                            g[next].insert(v);
                        }
                    }
                }
            }
        };

        construct_graph();

        let mut res = 0;

        for v in 0..g.len() {
            let x = v / c;
            let y = v % c;
            if !visited[v] && grid[x][y] == 1 {
                res = i32::max(res, Solution::dfs(v, &g, &mut visited));
            }
        }

        res
    }

    fn dfs(v: usize, g: &Vec<HashSet<usize>>, visited: &mut Vec<bool>) -> i32 {
        visited[v] = true;
        let mut res = 1;
        for &w in &g[v] {
            if !visited[w] {
                res += Solution::dfs(w, g, visited);
            }
        }
        res
    }
}

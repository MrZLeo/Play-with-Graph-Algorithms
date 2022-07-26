use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let dir = vec![vec![-1, 0], vec![0, 1], vec![1, 0], vec![0, -1]];
        let r = grid.len();
        let c = grid[0].len();
        let mut visited = vec![vec![false; c]; r];

        let mut res = 0;
        for x in 0..r {
            for y in 0..c {
                if !visited[x][y] && grid[x][y] == 1 {
                    res = i32::max(
                        res,
                        Solution::dfs(x, y, &mut visited, &dir, r as i32, c as i32, &grid),
                    );
                }
            }
        }
        res
    }

    fn dfs(
        x: usize,
        y: usize,
        visited: &mut Vec<Vec<bool>>,
        dir: &Vec<Vec<i32>>,
        r: i32,
        c: i32,
        grid: &Vec<Vec<i32>>,
    ) -> i32 {
        visited[x][y] = true;
        let mut res = 1;
        for d in dir {
            let x = x as i32 + d[0];
            let y = y as i32 + d[1];
            if x >= 0 && y >= 0 && x < r && y < c {
                let x = x as usize;
                let y = y as usize;
                if !visited[x][y] && grid[x][y] == 1 {
                    res += Solution::dfs(x, y, visited, dir, r, c, grid);
                }
            }
        }
        res
    }
}

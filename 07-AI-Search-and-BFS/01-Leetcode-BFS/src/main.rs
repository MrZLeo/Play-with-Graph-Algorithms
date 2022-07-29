use std::collections::LinkedList;

struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let dirs = vec![
            vec![0, 1],
            vec![0, -1],
            vec![1, 0],
            vec![1, 1],
            vec![1, -1],
            vec![-1, 0],
            vec![-1, 1],
            vec![-1, -1],
        ];
        let r = grid.len();
        let c = grid[0].len();
        let mut visited = vec![vec![false; c]; r];
        let mut dis = vec![vec![1; c]; r];

        // cannot reach
        if grid[0][0] == 1 {
            return -1;
        }

        // only one point
        if r == c && r == 1 {
            return 1;
        }

        let in_area = |x, y| x >= 0 && x < r as i32 && y >= 0 && y < c as i32;

        let mut q = LinkedList::new();
        q.push_back(0);
        visited[0][0] = true;
        dis[0][0] = 1;
        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            let curx = cur / c as i32;
            let cury = cur % c as i32;
            for dir in &dirs {
                let nextx = curx + dir[0];
                let nexty = cury + dir[1];
                if in_area(nextx, nexty)
                    && !visited[nextx as usize][nexty as usize]
                    && grid[nextx as usize][nexty as usize] == 0
                {
                    q.push_back(nextx * (c as i32) + nexty);

                    let (nextx, nexty) = (nextx as usize, nexty as usize);
                    visited[nextx][nexty] = true;
                    dis[nextx][nexty] = dis[curx as usize][cury as usize] + 1;

                    if nextx == c - 1 && nexty == r - 1 {
                        return dis[nextx][nexty];
                    }
                }
            }
        }

        -1
    }
}

fn main() {}

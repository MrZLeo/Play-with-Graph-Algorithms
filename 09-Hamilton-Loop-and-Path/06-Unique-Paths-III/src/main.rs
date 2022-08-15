struct Solution;

const DIR: [[i32; 2]; 4] = [[-1, 0], [0, -1], [1, 0], [0, 1]];
static mut R: usize = 0;
static mut C: usize = 0;
static mut END: usize = 0;

impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let r;
        let c;
        unsafe {
            R = grid.len();
            C = grid[0].len();
            r = R;
            c = C;
        }
        let mut start = 0;
        let mut visited = vec![vec![false; c]; r];
        let mut left = r * c;

        for i in 0..r {
            for j in 0..c {
                if grid[i][j] == 1 {
                    start = i * c + j;
                    grid[i][j] = 0;
                } else if grid[i][j] == 2 {
                    unsafe {
                        END = i * c + j;
                    }
                    grid[i][j] = 0;
                } else if grid[i][j] == -1 {
                    left -= 1;
                }
            }
        }

        Solution::dfs(start, left, &mut visited, &mut grid)
    }

    fn dfs(start: usize, left: usize, visited: &mut [Vec<bool>], grid: &mut [Vec<i32>]) -> i32 {
        unsafe {
            let x = start / C;
            let y = start % C;
            visited[x][y] = true;
            if left == 1 && start == END {
                visited[x][y] = false;
                return 1;
            }

            let in_area = |x, y| -> bool { x >= 0 && x < R as i32 && y >= 0 && y < C as i32 };

            let mut res = 0;
            for dir in DIR {
                let nextx = x as i32 + dir[0];
                let nexty = y as i32 + dir[1];
                if in_area(nextx, nexty)
                    && !visited[nextx as usize][nexty as usize]
                    && grid[nextx as usize][nexty as usize] != 0
                {
                    res +=
                        Solution::dfs((nextx * C as i32 + nexty) as usize, left - 1, visited, grid);
                }
            }

            visited[x][y] = false;
            res
        }
    }
}

fn main() {
    println!("Hello, world!");
}

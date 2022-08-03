use std::collections::{HashMap, LinkedList};

// Leetcode 773, finished
// use 6-bit number to present state
struct Solution;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut q = LinkedList::new();
        let mut visited = HashMap::new();

        let s0 = Solution::to_state(&board);
        q.push_back(s0);
        visited.insert(s0, 0);
        if s0 == 123450 {
            return 0;
        }

        // while !q.is_empty() {
        //     let s = q.pop_front().unwrap();
        // }
        while let Some(s) = q.pop_front() {
            let nexts = Solution::get_nexts(s);
            for next in &nexts {
                if !visited.contains_key(next) {
                    visited.insert(*next, visited[&s] + 1);
                    q.push_back(*next);
                    if *next == 123450 {
                        return visited[next];
                    }
                }
            }
        }

        -1
    }

    fn to_state(board: &Vec<Vec<i32>>) -> usize {
        let mut res = 0;
        for row in board {
            for j in row {
                res = res * 10 + j;
            }
        }
        res as usize
    }

    fn to_board(mut s: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; 3]; 2];
        for i in (0..2).rev() {
            for j in (0..3).rev() {
                res[i][j] = s % 10;
                s /= 10;
            }
        }
        res
    }

    fn get_nexts(s: usize) -> Vec<usize> {
        let dirs = vec![vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1]];
        let in_area = |x, y| (0..2).contains(&x) && (0..3).contains(&y);
        let mut board = Solution::to_board(s as i32);

        // find zero
        let mut x = 0;
        let mut y = 0;
        for (i, row) in board.iter().enumerate() {
            for (j, e) in row.iter().enumerate() {
                if *e == 0 {
                    x = i;
                    y = j;
                }
            }
        }

        let mut res = Vec::new();
        for dir in &dirs {
            let nextx = x + dir[0] as usize;
            let nexty = y + dir[1] as usize;
            if in_area(nextx, nexty) {
                let t = board[x][y];
                board[x][y] = board[nextx][nexty];
                board[nextx][nexty] = t;
                res.push(Solution::to_state(&board));
                let t = board[x][y];
                board[x][y] = board[nextx][nexty];
                board[nextx][nexty] = t;
            }
        }

        res
    }
}

fn main() {
    println!("Hello, world!");
}

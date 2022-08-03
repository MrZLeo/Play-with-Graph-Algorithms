use std::collections::{HashMap, LinkedList};

// Leetcode 773, unfinished
// use 6-bit number to present state
struct Solution;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut q = LinkedList::new();
        let mut visited = HashMap::new();

        let s0 = Solution::to_state(board);
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

    fn to_state(board: Vec<Vec<i32>>) -> usize {
        let mut res = 0;
        for row in &board {
            for j in row {
                res = res * 10 + j;
            }
        }
        res as usize
    }

    fn get_nexts(s: usize) -> Vec<usize> {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}

// Leetcode 752
use std::collections::{HashMap, HashSet, LinkedList};
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let deadset: HashSet<String> = HashSet::from_iter(deadends.into_iter());

        // special situations
        if deadset.contains("0000") || deadset.contains(&target) {
            return -1;
        }
        if target == "0000" {
            return 0;
        }

        // BFS
        let mut q = LinkedList::new();
        q.push_back("0000".to_string());

        let mut visited: HashMap<String, i32> = HashMap::new();
        visited.insert("0000".to_string(), 0);

        while !q.is_empty() {
            let curs = q.pop_front().unwrap();
            let mut curarray: Vec<char> = curs.chars().collect();
            let mut nexts = Vec::new();

            // construct nexts
            for i in 0..4 {
                let o = curarray[i];
                curarray[i] = char::from_digit((o.to_digit(10).unwrap() + 1) % 10, 10).unwrap();
                nexts.push(String::from_iter(curarray.iter()));

                curarray[i] = char::from_digit((o.to_digit(10).unwrap() + 9) % 10, 10).unwrap();
                nexts.push(String::from_iter(curarray.iter()));
                curarray[i] = o;
            }

            for next in nexts.into_iter() {
                if !visited.contains_key(&next) && !deadset.contains(&next) {
                    if next == target {
                        return visited[&curs] + 1;
                    }
                    // q own next, visited need have a copy of next
                    visited.insert(next.clone(), visited[&curs] + 1);
                    // push next into q last, because we need to move ownership
                    // of next into q
                    q.push_back(next);
                }
            }
        }

        -1
    }
}

fn main() {
    println!("Hello, world!");
}

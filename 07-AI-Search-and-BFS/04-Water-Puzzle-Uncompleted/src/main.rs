use std::collections::LinkedList;

struct WaterPuzzle;

impl WaterPuzzle {
    fn new(bottle1: usize, bottle2: usize) -> Self {
        // maximum of two bottles should less than 10
        assert!(bottle1 < 10);
        assert!(bottle2 < 10);

        let mut q = LinkedList::new();
        let mut visited = vec![false; 100];

        q.push_back(0usize);
        visited[0] = true;

        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            let a = cur / 10;
            let b = cur % 10;

            let mut nexts = Vec::<usize>::new();

            for next in nexts {
                if !visited[next] {
                    q.push_back(next);
                    visited[next] = true;
                }
                if next / 10 == 4 || next % 10 == 4 {
                    return Self {};
                }
            }
        }

        Self {}
    }
}

fn main() {
    println!("Hello, world!");
}

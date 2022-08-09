use std::{collections::LinkedList, ops::Index};

struct WaterPuzzle {
    end: i32,
    pre: Vec<i32>,
}

impl WaterPuzzle {
    fn new(bottle1: usize, bottle2: usize) -> Self {
        // maximum of two bottles should less than 10
        assert!(bottle1 < 10);
        assert!(bottle2 < 10);

        let mut q = LinkedList::new();
        let mut visited = vec![false; 100];
        let mut pre = vec![-1; 100];

        q.push_back(0usize);
        visited[0] = true;

        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            let a = cur / 10;
            let b = cur % 10;

            let mut nexts = Vec::<usize>::with_capacity(6);
            nexts.push(bottle1 * 10 + b);
            nexts.push(a * 10 + bottle2);
            nexts.push(b);
            nexts.push(a * 10);

            let x = usize::min(a, bottle2 - b);
            nexts.push((a - x) * 10 + (b + x));

            let y = usize::min(b, bottle1 - a);
            nexts.push((a + y) * 10 + (b - y));

            for next in nexts {
                if !visited[next] {
                    q.push_back(next);
                    visited[next] = true;
                    pre[next] = cur as i32;
                }
                if next / 10 == 4 || next % 10 == 4 {
                    return Self {
                        end: next as i32,
                        pre,
                    };
                }
            }
        }

        Self {
            end: -1,
            pre: vec![],
        }
    }

    fn result(&self) -> Box<dyn Iterator<Item = usize>> {
        let mut res = Vec::new();
        let mut cur = self.end;

        if cur == -1 {
            return Box::new(res.into_iter());
        }

        while cur != 0 {
            res.push(cur as usize);
            cur = self.pre[cur as usize];
        }
        res.push(0);
        res.reverse();
        Box::new(res.into_iter())
    }
}

fn main() {
    let water = WaterPuzzle::new(5, 3);
    water.result().for_each(|x| print!("{:02} ", x));
    println!();
}

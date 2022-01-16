use core::panic;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Debug)]
struct AdjMatrix {
    v: i32,
    e: i32,
    adj: Vec<Vec<i32>>,
}

impl AdjMatrix {
    pub fn from(filename: &str) -> Self {
        let file = fs::read_to_string(filename).unwrap();
        let mut iter = file.split("\n").into_iter();
        let mut first_line = iter.next().unwrap().split_whitespace();

        let v = first_line.next().unwrap().parse::<i32>().unwrap();
        let e = first_line.next().unwrap().parse::<i32>().unwrap();

        if v < 0 {
            panic!("V must be non-negative");
        }
        if e < 0 {
            panic!("E must be not-negative");
        }

        let mut ret = AdjMatrix {
            v,
            e,
            adj: vec![vec![0; v as usize]; v as usize],
        };

        while let Some(line) = iter.next() {
            if line.len() == 0 {
                break;
            }
            let mut line = line.split_whitespace();
            let x = line.next().unwrap().parse::<usize>().unwrap();
            let y = line.next().unwrap().parse::<usize>().unwrap();
            ret.validate_vertex(x);
            ret.validate_vertex(y);

            if x == y {
                panic!("Self Loop is Detected!");
            }
            if ret.adj[x][y] == 1 {
                panic!("Parallel Edges are Detected!");
            }

            ret.adj[x][y] = 1;
            ret.adj[y][x] = 1;
        }

        return ret;
    }

    pub fn e(&self) -> i32 {
        self.e
    }

    pub fn v(&self) -> i32 {
        self.v
    }

    pub fn has_edge(&self, v1: usize, v2: usize) -> bool {
        self.validate_vertex(v1);
        self.validate_vertex(v2);
        return self.adj[v1][v2] == 1;
    }

    fn validate_vertex(&self, v: usize) {
        if v >= self.v as usize {
            panic!("vertex {} is not valid", v);
        }
    }

    pub fn adj_edge(&self, v: usize) -> Vec<i32> {
        self.validate_vertex(v);

        self.adj[v]
            .to_owned()
            .into_iter()
            .enumerate()
            .filter(|&(_, x)| x == 1)
            .map(|(index, _)| index as i32)
            .collect()
    }

    pub fn degree(&self, v: usize) -> i32 {
        self.validate_vertex(v);

        self.adj_edge(v).len() as i32
    }
}

impl Display for AdjMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut matrix = String::new();
        for line in &self.adj {
            for num in line {
                matrix.push_str(num.to_string().as_str());
                matrix.push_str(" ");
            }
            matrix.pop();
            matrix.push_str("\n");
        }
        write!(
            f,
            "AdjMatrix: \nV: {}, \nE: {}, \nMatrix: \n{}",
            self.v, self.e, matrix
        )
    }
}

pub fn main() {
    let adj = AdjMatrix::from("g.txt");
    println!("{}", adj);
}

#[cfg(test)]
mod test_02_03 {
    use crate::AdjMatrix;
    #[test]
    fn test_adj_edge() {
        let adj = AdjMatrix::from("g.txt");
        assert_eq!(adj.adj_edge(1), vec![0, 2, 6]);
        assert_eq!(adj.adj_edge(1), vec![0, 2, 6]); // 重复调用是否会导致move
        assert_eq!(adj.adj_edge(3), vec![0, 2, 4]);
    }
}

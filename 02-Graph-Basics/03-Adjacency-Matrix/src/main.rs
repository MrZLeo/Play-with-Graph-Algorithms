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
            ret.mut_adj()[x][y] = 1;
            ret.mut_adj()[y][x] = 1;
        }

        return ret;
    }

    pub fn mut_adj(&mut self) -> &mut Vec<Vec<i32>> {
        &mut self.adj
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

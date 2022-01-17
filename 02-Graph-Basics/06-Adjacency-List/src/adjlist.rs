use core::panic;
use std::collections::LinkedList;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Debug)]
pub struct AdjList {
    v: usize,
    e: usize,
    adj: Vec<LinkedList<usize>>,
}

impl AdjList {
    pub fn from(filename: &str) -> Self {
        let file = fs::read_to_string(filename).unwrap();
        let mut iter = file.split("\n").into_iter();
        let mut first_line = iter.next().unwrap().split_whitespace();

        let v = first_line.next().unwrap().parse::<usize>().unwrap();
        let e = first_line.next().unwrap().parse::<usize>().unwrap();

        let mut ret = AdjList {
            v,
            e,
            adj: vec![LinkedList::new(); v as usize],
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
            if ret.adj[x].contains(&y) {
                panic!("Parallel Edges are Detected!");
            }

            ret.adj[x].push_back(y);
            ret.adj[y].push_back(x);
        }

        return ret;
    }

    pub fn e(&self) -> usize {
        self.e
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn has_edge(&self, v1: usize, v2: usize) -> bool {
        self.validate_vertex(v1);
        self.validate_vertex(v2);
        return self.adj[v1].contains(&v2);
    }

    fn validate_vertex(&self, v: usize) {
        if v >= self.v as usize {
            panic!("vertex {} is not valid", v);
        }
    }

    pub fn adj_edge(&self, v: usize) -> &LinkedList<usize> {
        self.validate_vertex(v);

        &self.adj[v]
    }

    pub fn degree(&self, v: usize) -> usize {
        self.validate_vertex(v);

        self.adj_edge(v).len()
    }
}

impl Display for AdjList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut list = String::new();
        for v in 0..self.v {
            list.push_str(format!("{}: ", v).as_str());
            for w in &self.adj[v] {
                list.push_str(format!("{} ", w).as_str());
            }
            list.push('\n');
        }
        write!(
            f,
            "AdjList: \nV: {}, E: {}\nList: \n{}",
            self.v, self.e, list
        )
    }
}

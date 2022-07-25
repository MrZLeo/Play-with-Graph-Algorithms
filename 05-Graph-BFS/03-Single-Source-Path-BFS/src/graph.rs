#![allow(dead_code)]

use std::{collections::HashSet, fmt::Display, fs};

#[derive(Debug)]
pub struct Graph {
    e: usize,
    v: usize,
    adj_list: Vec<HashSet<usize>>,
}

impl Graph {
    pub fn from_file(path: &str) -> Self {
        let file = match fs::read_to_string(path) {
            Err(_) => panic!("Cannot open file {}", path),
            Ok(str) => str,
        };

        let mut lines = file.lines();
        let mut first_line = lines.next().unwrap().split_whitespace();
        let v = first_line.next().unwrap().parse::<usize>().unwrap();
        let e = first_line.next().unwrap().parse::<usize>().unwrap();

        let mut adj_list: Vec<HashSet<usize>> = vec![HashSet::new(); v];

        // we can't use `self.validate_vertex` here, so we have to define
        // a closure to do that
        let validate_vertex = |x| {
            if x >= v {
                panic!("vertex {} is not valid", x);
            }
        };

        for line in lines {
            let mut line = line.split_whitespace();
            let x = line.next().unwrap().parse::<usize>().unwrap();
            let y = line.next().unwrap().parse::<usize>().unwrap();

            validate_vertex(x);
            validate_vertex(y);

            if x == y {
                panic!("Self Loop is Detected!");
            }

            if adj_list[x].contains(&y) {
                panic!("Parallel Edges are Detected!");
            }

            adj_list[x].insert(y);
            adj_list[y].insert(x);
        }

        Graph { e, v, adj_list }
    }

    #[inline]
    pub fn validate_vertex(&self, v: usize) {
        if v >= self.v {
            panic!("vertex {} is not valid", v);
        }
    }

    #[inline]
    pub fn e(&self) -> usize {
        self.e
    }

    #[inline]
    pub fn v(&self) -> usize {
        self.v
    }

    pub fn has_edge(&self, v1: usize, v2: usize) -> bool {
        self.validate_vertex(v1);
        self.validate_vertex(v2);
        self.adj_list[v1].contains(&v2)
    }

    pub fn adj_edge(&self, v: usize) -> Box<dyn Iterator<Item = usize>> {
        self.validate_vertex(v);
        Box::new((self.adj_list[v]).clone().into_iter())
    }

    #[inline]
    pub fn degree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj_list[v].len()
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = String::new();
        for v in 0..self.v {
            list.push_str(format!("{}: ", v).as_str());
            for w in &self.adj_list[v] {
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

#[test]
fn build_graph() {
    let g = Graph::from_file("g.txt");
    assert_eq!(g.v(), 7);
    assert_eq!(g.e(), 9);

    let adj = g.adj_edge(0);
    let mut vec = Vec::new();
    for v in adj {
        vec.push(v);
    }
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 3]);

    assert!(g.has_edge(1, 6));
    assert!(g.has_edge(5, 6));
    assert!(g.has_edge(4, 5));

    assert_eq!(g.degree(2), 3);
}

#![allow(dead_code)]

use std::fmt::Debug;

#[derive(Eq)]
pub struct WeightedEdge {
    v: usize,
    w: usize,
    weight: i32,
}

impl WeightedEdge {
    pub fn new(v: usize, w: usize, weight: i32) -> Self {
        Self { v, w, weight }
    }

    pub fn weight(&self) -> i32 {
        self.weight
    }

    pub fn w(&self) -> usize {
        self.w
    }

    pub fn v(&self) -> usize {
        self.v
    }
}

impl Debug for WeightedEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}-{}: {})", self.v, self.w, self.weight).as_str())
    }
}

impl Ord for WeightedEdge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for WeightedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.weight.cmp(&self.weight))
    }
}

impl PartialEq for WeightedEdge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

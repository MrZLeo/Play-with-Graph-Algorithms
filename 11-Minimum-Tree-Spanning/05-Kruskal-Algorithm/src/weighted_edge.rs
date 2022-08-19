#![allow(dead_code)]

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

use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Edge {
    v: usize,
    w: usize,
}

impl Edge {
    pub fn new(v: usize, w: usize) -> Self {
        Self { v, w }
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.v, self.w)
    }
}

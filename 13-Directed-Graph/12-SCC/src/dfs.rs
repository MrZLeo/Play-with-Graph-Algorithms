use crate::graph::Graph;

pub struct Dfs {
    g: Graph,
    visited: Vec<bool>,
    post: Vec<usize>,
}

impl Dfs {
    pub fn new(g: Graph) -> Self {
        let visited = vec![false; g.v()];
        let post = Vec::with_capacity(g.v());

        let mut this = Self { g, visited, post };

        for v in 0..this.g.v() {
            if !this.visited[v] {
                this.dfs(v);
            }
        }

        this
    }

    fn dfs(&mut self, v: usize) {
        self.visited[v] = true;
        for w in self.g.adj_edge(v) {
            if !self.visited[w] {
                self.dfs(w);
            }
        }
        self.post.push(v);
    }

    pub fn post(&self) -> Vec<usize> {
        self.post.clone()
    }
}

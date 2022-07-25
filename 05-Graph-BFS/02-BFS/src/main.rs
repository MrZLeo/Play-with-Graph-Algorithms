use graph::Graph;
use graph_bfs::{Bfs, GraphBFS};

mod graph;
mod graph_bfs;

fn main() {
    let g = Graph::from_file("g.txt");
    let mut graph_bfs = GraphBFS::new(g);
    let res = graph_bfs.bfs(0);
    print!("bfs result: ");
    for v in res {
        print!("{v} ");
    }
    println!();
}

use graph::Graph;
use graph_bfs::GraphBFS;

use crate::graph_bfs::SingleSourcePath;

mod graph;
mod graph_bfs;

fn main() {
    let g = Graph::from_file("g1.txt");
    let mut graph_bfs = GraphBFS::new(0, g);
    graph_bfs.single_source_path();
    let res = graph_bfs.path(6);
    print!("path 0->6 result: ");
    // let vec: Vec<usize> = res.collect();
    // dbg!(vec);
    for v in res {
        print!("{v} ");
    }
    println!();
}

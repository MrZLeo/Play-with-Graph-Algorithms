use crate::hungarian::Hungarian;
use graph::Graph;

mod bipartition;
mod graph;
mod hungarian;
mod max_flow;
mod weighted_graph;

fn main() {
    let g = Graph::from_file("g.txt");
    let hg = Hungarian::new(g);
    println!("{}", hg.max_matching());

    let g = Graph::from_file("g2.txt");
    let hg = Hungarian::new(g);
    println!("{}", hg.max_matching());
}

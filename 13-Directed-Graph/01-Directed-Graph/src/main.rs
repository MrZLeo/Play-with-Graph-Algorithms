use graph::Graph;

use crate::weighted_graph::WeightedGraph;

mod graph;
mod weighted_graph;

fn main() {
    let g = Graph::from_file("ug.txt", false);
    println!("{g}");

    let g = Graph::from_file("ug.txt", true);
    println!("{g}");

    let g = WeightedGraph::from_file("wg.txt", false);
    println!("{g}");

    let g = WeightedGraph::from_file("wg.txt", true);
    println!("{g}");
}

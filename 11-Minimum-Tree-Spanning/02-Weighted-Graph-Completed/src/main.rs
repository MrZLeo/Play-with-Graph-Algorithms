use weighted_graph::WeightedGraph;

mod weighted_graph;

fn main() {
    let g = WeightedGraph::from_file("g.txt");
    println!("{g}");
}

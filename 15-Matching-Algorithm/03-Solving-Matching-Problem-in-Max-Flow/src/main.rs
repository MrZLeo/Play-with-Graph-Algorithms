use bipatite_matching::BipartiteMatching;
use graph::Graph;

mod bipartition;
mod bipatite_matching;
mod graph;
mod max_flow;
mod weighted_graph;

fn main() {
    let g = Graph::from_file("g.txt");
    let bipatite_matching = BipartiteMatching::new(g);
    println!("{}", bipatite_matching.max_matching());

    let g = Graph::from_file("g2.txt");
    let bipatite_matching = BipartiteMatching::new(g);
    println!("{}", bipatite_matching.max_matching());
}

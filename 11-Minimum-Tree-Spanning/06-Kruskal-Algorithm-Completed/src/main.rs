use kruskal::{Kruskal, KruskalImpl};
use weighted_graph::WeightedGraph;

mod cc;
mod kruskal;
mod union_find;
mod weighted_edge;
mod weighted_graph;

fn main() {
    let g = WeightedGraph::from_file("g.txt");
    let kruskal = KruskalImpl::new(g);
    println!("{:#?}", kruskal.mst())
}

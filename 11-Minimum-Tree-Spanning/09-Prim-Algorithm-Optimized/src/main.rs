use prim::{Prim, PrimImpl};
use weighted_graph::WeightedGraph;

mod cc;
mod prim;
mod union_find;
mod weighted_edge;
mod weighted_graph;

fn main() {
    let g = WeightedGraph::from_file("g.txt");
    let prim = PrimImpl::new(g);
    println!("{:#?}", prim.mst())
}

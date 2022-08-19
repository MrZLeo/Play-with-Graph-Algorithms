use dijkstra::Dijkstra;
use weighted_graph::WeightedGraph;

mod dijkstra;
mod weighted_edge;
mod weighted_graph;

fn main() {
    let g = WeightedGraph::from_file("g.txt");
    let v = g.v();
    let dij = Dijkstra::new(g, 0);
    for v in 0..v {
        print!("{} ", dij.dis_to(v));
    }
    println!();
}

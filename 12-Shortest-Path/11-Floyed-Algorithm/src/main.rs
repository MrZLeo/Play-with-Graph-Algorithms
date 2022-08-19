use weighted_graph::WeightedGraph;

use crate::floyed::Floyed;

mod bellman_ford;
mod dijkstra;
mod floyed;
mod weighted_edge;
mod weighted_graph;

fn main() {
    let g = WeightedGraph::from_file("g.txt");
    let size = g.v();
    let fy = Floyed::new(g);

    // use ∞ present we have a negative cycle
    for v in 0..size {
        for w in 0..size {
            print!(
                "{} ",
                if let Some(dis) = fy.dis_to(v, w) {
                    dis.to_string()
                } else {
                    "∞".to_string()
                }
            );
        }
        println!();
    }
}

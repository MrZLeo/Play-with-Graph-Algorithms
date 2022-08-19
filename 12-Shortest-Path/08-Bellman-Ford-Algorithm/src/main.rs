use bellman_ford::BellmanFord;
use weighted_graph::WeightedGraph;

mod bellman_ford;
mod dijkstra;
mod weighted_edge;
mod weighted_graph;

fn main() {
    let g = WeightedGraph::from_file("g.txt");
    let v = g.v();
    let bf = BellmanFord::new(g, 0);

    // use ∞ present we have a negative cycle
    for v in 0..v {
        print!(
            "{} ",
            if let Some(dis) = bf.dis_to(v) {
                dis.to_string()
            } else {
                "∞".to_string()
            }
        );
    }
    println!();
}

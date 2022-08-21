use max_flow::MaxFlow;
use weighted_graph::WeightedGraph;

mod max_flow;
mod weighted_graph;

fn main() {
    let g = WeightedGraph::from_file("network.txt", true);
    let mf = MaxFlow::new(g.clone(), 0, 3);
    println!("{}", mf.maxflow());
    for v in 0..g.v() {
        for w in g.adj_edge(v) {
            println!(
                "{}-{}: {} / {}",
                v,
                w,
                mf.get_flow(v, w).unwrap(),
                g.get_weight(v, w).unwrap()
            );
        }
    }

    let g = WeightedGraph::from_file("network2.txt", true);
    let mf = MaxFlow::new(g.clone(), 0, 5);
    println!("{}", mf.maxflow());
    for v in 0..g.v() {
        for w in g.adj_edge(v) {
            println!(
                "{}-{}: {} / {}",
                v,
                w,
                mf.get_flow(v, w).unwrap(),
                g.get_weight(v, w).unwrap()
            );
        }
    }
}

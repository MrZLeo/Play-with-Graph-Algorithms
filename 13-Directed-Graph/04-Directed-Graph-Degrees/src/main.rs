use graph::Graph;

use crate::directed_cycle_detection::DirectedCycleDetection;

mod directed_cycle_detection;
mod graph;

fn main() {
    let g = Graph::from_file("ug.txt", true);
    let dcd = DirectedCycleDetection::new(g);
    println!("{}", dcd.has_cycle());

    let g = Graph::from_file("ug2.txt", true);
    let dcd = DirectedCycleDetection::new(g);
    println!("{}", dcd.has_cycle());
}

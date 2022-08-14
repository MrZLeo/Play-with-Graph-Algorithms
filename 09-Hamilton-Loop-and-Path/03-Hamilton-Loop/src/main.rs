use graph::Graph;
use hamilton_loop::{HamiltonLoop, HamiltonLoopImpl};

mod graph;
mod hamilton_loop;

fn main() {
    let g = Graph::from_file("g.txt");
    let mut hl = HamiltonLoopImpl::new(g);

    println!("{:?}", hl.hamilton_loop());

    let g2 = Graph::from_file("g2.txt");
    let mut hl2 = HamiltonLoopImpl::new(g2);
    println!("{:?}", hl2.hamilton_loop());
}

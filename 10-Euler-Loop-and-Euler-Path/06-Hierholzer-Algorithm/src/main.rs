use euler_loop::EulerLoopImpl;
use graph::Graph;

use crate::euler_loop::EulerLoop;

mod euler_loop;
mod graph;

fn main() {
    let g = Graph::from_file("g.txt");
    let el = EulerLoopImpl::new(g);
    println!("{:?}", el.euler_loop());

    let g2 = Graph::from_file("g2.txt");
    let el2 = EulerLoopImpl::new(g2);
    println!("{:?}", el2.euler_loop());
}

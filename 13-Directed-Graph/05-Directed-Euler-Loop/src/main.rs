use euler_loop::{EulerLoop, EulerLoopImpl};
use graph::Graph;

mod euler_loop;
mod graph;

fn main() {
    let g = Graph::from_file("ug.txt", true);
    let el = EulerLoopImpl::new(g);
    println!("{:?}", el.euler_loop());

    let g = Graph::from_file("ug2.txt", true);
    let el = EulerLoopImpl::new(g);
    println!("{:?}", el.euler_loop());
}

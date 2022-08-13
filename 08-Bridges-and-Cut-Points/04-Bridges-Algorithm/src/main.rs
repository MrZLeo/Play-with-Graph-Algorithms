use find_bridges::{FindBridges, FindBridgesImpl};
use graph::Graph;

mod edge;
mod find_bridges;
mod graph;

fn main() {
    let g = Graph::from_file("g.txt");
    let g2 = Graph::from_file("g2.txt");
    let g3 = Graph::from_file("g3.txt");
    let tree = Graph::from_file("tree.txt");

    let mut fb = FindBridgesImpl::new(g);
    let mut fb2 = FindBridgesImpl::new(g2);
    let mut fb3 = FindBridgesImpl::new(g3);
    let mut fb_tree = FindBridgesImpl::new(tree);

    println!("Bridges in g: {:?}", fb.bridges());
    println!("Bridges in g1: {:?}", fb2.bridges());
    println!("Bridges in g2: {:?}", fb3.bridges());
    println!("Bridges in tree: {:?}", fb_tree.bridges());
}

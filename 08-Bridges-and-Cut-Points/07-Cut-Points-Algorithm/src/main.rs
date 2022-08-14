use find_bridges::{FindBridges, FindBridgesImpl};
use find_cut_points::{FindCutPoints, FindCutPointsImpl};
use graph::Graph;

mod edge;
mod find_bridges;
mod find_cut_points;
mod graph;

fn main() {
    let g = Graph::from_file("g.txt");
    let g2 = Graph::from_file("g2.txt");
    let g3 = Graph::from_file("g3.txt");
    let tree = Graph::from_file("tree.txt");

    let mut fc = FindCutPointsImpl::new(g);
    let mut fc2 = FindCutPointsImpl::new(g2);
    let mut fc3 = FindCutPointsImpl::new(g3);
    let mut fc_tree = FindCutPointsImpl::new(tree);

    println!("Bridges in g: {:?}", fc.cut_points());
    println!("Bridges in g1: {:?}", fc2.cut_points());
    println!("Bridges in g2: {:?}", fc3.cut_points());
    println!("Bridges in tree: {:?}", fc_tree.cut_points());
}

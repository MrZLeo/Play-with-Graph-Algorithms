use graph::Graph;
use topo_sort::TopoSort;

mod graph;
mod topo_sort;

fn main() {
    let g = Graph::from_file("ug.txt", true);
    let ts = TopoSort::new(g);
    println!("{:?}", ts.result());
}

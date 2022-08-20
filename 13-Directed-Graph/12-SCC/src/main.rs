use graph::Graph;
use scc::Scc;

mod dfs;
mod graph;
mod scc;

fn main() {
    let g = Graph::from_file("ug.txt", true);
    let scc = Scc::new(g);

    println!("{:?}", scc.component());
}

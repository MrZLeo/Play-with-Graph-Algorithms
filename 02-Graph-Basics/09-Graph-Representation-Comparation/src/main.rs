mod graph;

use crate::graph::Graph;

pub fn main() {
    let set = Graph::from("g.txt");
    println!("{}", set);

}

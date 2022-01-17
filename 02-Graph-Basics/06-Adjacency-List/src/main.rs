mod adjlist;

use crate::adjlist::AdjList;

pub fn main() {
    let list = AdjList::from("g.txt");
    println!("{}", list);
}

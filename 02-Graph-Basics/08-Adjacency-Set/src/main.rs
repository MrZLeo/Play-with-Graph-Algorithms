mod adjlist;
mod adjset;

use crate::adjlist::AdjList;
use crate::adjset::AdjSet;

pub fn main() {
    let list = AdjList::from("g.txt");
    println!("{}", list);
    let set = AdjSet::from("g.txt");
    println!("{}", set);

}

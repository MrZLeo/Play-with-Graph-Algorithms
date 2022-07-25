use crate::graph_bfs::SingleSourcePath;

pub trait Usssp: SingleSourcePath {
    fn dis(&self, t: usize) -> i32;
}

pub trait SingleSourcePath {
    fn single_source_path(&mut self);
    fn is_connected(&mut self, t: usize) -> bool;
    fn path(&mut self, t: usize) -> Box<dyn Iterator<Item = usize>>;
}

pub trait Usssp: SingleSourcePath {
    fn dis(&mut self, t: usize) -> i32;
}

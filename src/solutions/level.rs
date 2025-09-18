pub trait Level {
    fn level(&self) -> usize;
    fn solve(&self, input: &str) -> Option<String>;
}

pub trait Level {
    const LEVEL: usize;

    fn solve(&self, input: &str) -> Option<String>;
}

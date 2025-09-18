use crate::solutions::level::Level;

pub struct Level2;

impl Level for Level2 {
    fn level(&self) -> usize {
        2
    }

    fn solve(&self, input: &str) -> Option<String> {
        Some(input.to_owned())
    }
}


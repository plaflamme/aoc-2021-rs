use itertools::Itertools;
use std::str::FromStr;

pub struct Solution(Vec<u32>);

fn increases(depths: impl Iterator<Item = u32>) -> usize {
    depths
        .into_iter()
        .tuple_windows()
        .filter(|(previous, next)| next > previous)
        .count()
}

impl super::Solver for Solution {
    const SAMPLE: &'static str = "199
200
208
210
200
207
240
269
260
263";

    const LEVEL1: &'static str = "7";

    const LEVEL2: &'static str = "5";

    type Output = usize;

    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|l| u32::from_str(l.trim()).unwrap())
                .collect(),
        )
    }

    fn part1(self) -> Self::Output {
        increases(self.0.into_iter())
    }

    fn part2(self) -> Self::Output {
        increases(
            self.0
                .into_iter()
                .tuple_windows()
                .map(|(a, b, c)| a + b + c),
        )
    }
}

use itertools::Itertools;
use std::str::FromStr;

sample!(
    crate::Day1,
    "199
200
208
210
200
207
240
269
260
263",
    "7",
    "5"
);

fn increases(depths: impl Iterator<Item = u32>) -> usize {
    depths
        .into_iter()
        .tuple_windows()
        .filter(|(previous, next)| next > previous)
        .count()
}

impl super::Solver for super::Day1 {
    type Output = usize;
    type Input = Vec<u32>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| u32::from_str(l.trim()).unwrap())
            .collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        increases(input.into_iter())
    }

    fn part2(input: Self::Input) -> Self::Output {
        increases(input.into_iter().tuple_windows().map(|(a, b, c)| a + b + c))
    }
}

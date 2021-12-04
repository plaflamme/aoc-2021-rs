pub trait Solver {
    const SAMPLE: &'static str;
    const LEVEL1: &'static str;
    const LEVEL2: &'static str;

    type Output: ToString;

    fn parse(input: &str) -> Self
    where
        Self: Sized;

    fn part1(self) -> Self::Output;

    fn part2(self) -> Self::Output;
}

pub mod day1;
pub mod day2;
pub mod day3;

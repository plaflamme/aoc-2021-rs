#![feature(drain_filter)]
#![feature(const_fn_trait_bound)]

use aocf::{Aoc, Level};

#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl std::str::FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "1" | "one" => Ok(Part::One),
            "2" | "two" => Ok(Part::Two),
            _ => Err(anyhow::anyhow!("invalid puzzle part {}", s)),
        }
    }
}

pub trait Day {
    const DAY: u8;
}

macro_rules! day {
    ($d: ident, $n: literal) => {
        pub struct $d;
        impl Day for $d {
            const DAY: u8 = $n;
        }
    };
}

day!(Day1, 1);
day!(Day2, 2);
day!(Day3, 3);
day!(Day4, 4);
day!(Day5, 5);
day!(Day6, 6);

pub trait Input {
    fn load(&mut self) -> String; // TODO: how do we make the Aoc implementation require &mut only?
    fn solution(&self, part: Part) -> Option<String>;
}

pub struct Sample {
    content: &'static str,
    part1: &'static str,
    part2: Option<&'static str>,
}

impl Sample {
    const fn new(content: &'static str, part1: &'static str, part2: Option<&'static str>) -> Self {
        Sample {
            content,
            part1,
            part2,
        }
    }
}

impl Input for Sample {
    fn load(&mut self) -> String {
        self.content.to_string()
    }

    fn solution(&self, part: Part) -> Option<String> {
        match part {
            Part::One => Some(self.part1.to_string()),
            Part::Two => self.part2.map(|s| s.to_string()),
        }
    }
}

macro_rules! sample {
    ($content: expr, $part1: expr, $part2: expr) => {
        pub const SAMPLE: crate::Sample = crate::Sample::new($content, $part1, Some($part2));
    };
    ($content: expr, $part1: expr) => {
        pub const SAMPLE: crate::Sample = crate::Sample::new($content, $part1, None);
    };
}

impl Input for Aoc {
    fn load(&mut self) -> String {
        self.get_input(false).unwrap()
    }

    fn solution(&self, part: Part) -> Option<String> {
        match part {
            Part::One => self.solution.get(&Level::First).cloned(),
            Part::Two => self.solution.get(&Level::Second).cloned(),
        }
    }
}

#[derive(Debug)]
pub struct Main;

pub trait Solver<Alt = Main> {
    type Output: ToString;
    type Input: Sized;
    fn parse(input: &str) -> Self::Input;
    fn part1(input: Self::Input) -> Self::Output;
    fn part2(input: Self::Input) -> Self::Output;
}

macro_rules! timed {
    ($e: expr) => {{
        use std::time::Instant;
        let start = Instant::now();
        let solution = { $e };
        (solution, start.elapsed())
    }};
}

pub fn solve_part<D: Day, A>(input: &str, part: Part) -> (String, std::time::Duration)
where
    D: Solver<A>,
{
    let parsed = <D as Solver<A>>::parse(input);
    let (solution, duration) = match part {
        Part::One => timed!(<D as Solver<A>>::part1(parsed)),
        Part::Two => timed!(<D as Solver<A>>::part2(parsed)),
    };
    (solution.to_string(), duration)
}

pub(crate) mod tools;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

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

#[macro_export]
macro_rules! day {
    ($d: ident, $n: literal) => {
        #[derive(Clone, Copy)]
        pub struct $d;
        impl Day for $d {
            const DAY: u8 = $n;
        }
    };
}

pub trait Input {
    fn load(&mut self) -> String; // TODO: how do we make the Aoc implementation require &mut only?
    fn solution(&self, part: Part) -> Option<String>;
}

pub trait Sample {
    const CONTENT: &'static str;
    const PART1: &'static str;
    const PART2: Option<&'static str>;
}

impl<S> Input for S
where
    S: Sample,
{
    fn load(&mut self) -> String {
        <S as Sample>::CONTENT.to_string()
    }

    fn solution(&self, part: Part) -> Option<String> {
        match part {
            Part::One => Some(<S as Sample>::PART1.to_string()),
            Part::Two => <S as Sample>::PART2.map(|s| s.to_string()),
        }
    }
}

#[macro_export]
macro_rules! sample {
    ($day: path, $content: literal, $part1: literal, $part2: literal) => {
        sample!($day, $content, $part1, Some($part2));
    };
    ($day: path, $content: literal, $part1: literal) => {
        sample!($day, $content, $part1, None);
    };
    ($day: path, $content: literal, $part1: literal, $part2: expr) => {
        impl ::aoc_lib::Sample for $day {
            const CONTENT: &'static str = $content;
            const PART1: &'static str = $part1;
            const PART2: Option<&'static str> = $part2;
        }
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

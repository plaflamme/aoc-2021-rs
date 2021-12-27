#![feature(drain_filter)]
#![feature(const_fn_trait_bound)]
#![feature(box_patterns)]
#![feature(int_roundings)] // for div_floor

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
        #[derive(Clone, Copy)]
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
day!(Day7, 7);
day!(Day8, 8);
day!(Day9, 9);
day!(Day10, 10);
day!(Day11, 11);
day!(Day12, 12);
day!(Day13, 13);
day!(Day14, 14);
day!(Day15, 15);
day!(Day16, 16);
day!(Day17, 17);
day!(Day18, 18);
day!(Day19, 19);
day!(Day20, 20);
day!(Day21, 21);
day!(Day22, 22);
day!(Day23, 23);
day!(Day24, 24);
day!(Day25, 25);

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

macro_rules! sample {
    ($day: path, $content: literal, $part1: literal, $part2: literal) => {
        sample!($day, $content, $part1, Some($part2));
    };
    ($day: path, $content: literal, $part1: literal) => {
        sample!($day, $content, $part1, None);
    };
    ($day: path, $content: literal, $part1: literal, $part2: expr) => {
        impl crate::Sample for $day {
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

pub(crate) mod grid;
pub(crate) mod tools;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

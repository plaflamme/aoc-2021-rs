use aocf::{Aoc, Level};
use clap::Parser;
use std::error::Error;
use std::num::ParseIntError;
use termion::{color, style};

use aoc2021::*;

#[derive(clap::ArgEnum, Clone, Copy)]
enum Mode {
    Sample,
    Print,
    Submit,
}

#[derive(Debug, Clone, Copy)]
enum Part {
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

enum Days {
    Set(Vec<u8>),
    Range(u8, u8),
}

impl std::str::FromStr for Days {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((from, to)) = s.split_once("..") {
            let from = u8::from_str(from).unwrap_or(1);
            let to = u8::from_str(to).unwrap_or(24);
            Ok(Days::Range(from, to))
        } else if s.to_ascii_lowercase() == "all" {
            Ok(Days::Range(1, 24))
        } else {
            Ok(Days::Set(
                s.split(',')
                    .map(|c| u8::from_str(c))
                    .collect::<Result<Vec<u8>, ParseIntError>>()?,
            ))
        }
    }
}

impl IntoIterator for Days {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Days::Set(days) => days.into_iter(),
            Days::Range(from, to) => (from..=to).collect::<Vec<u8>>().into_iter(),
        }
    }
}

#[derive(clap::Parser)]
struct Opts {
    /// Day(s) to run, e.g.: -d 1 or -d 1,3 or -d 3..6 or -d ..4
    #[clap(short, name = "days", default_value = "all")]
    days: Days,

    /// Puzzle part to run, e.g.: -p 2
    #[clap(short, name = "parts")]
    part: Option<Part>,

    #[clap(arg_enum, default_value = "print")]
    mode: Mode,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    println!("AoC - 2021");

    for day in opts.days {
        println!("Day {}", day);

        let mut aoc = Aoc::new()
            .parse_cli(false)
            .year(Some(2021))
            .day(Some(day as u32))
            .init()?;

        let parts: Vec<Part> = match opts.part {
            None => vec![Part::One, Part::Two],
            Some(part) => vec![part],
        };

        for part in parts {
            print!("  - part {:?} ... ", part);
            let result = match day {
                1 => run_day::<day1::Solution>(&mut aoc, opts.mode, part),
                2 => run_day::<day2::Solution>(&mut aoc, opts.mode, part),
                3 => run_day::<day3::Solution>(&mut aoc, opts.mode, part),
                4 => run_day::<day4::Solution>(&mut aoc, opts.mode, part),
                5..=24 => Ok("not implemented".to_string()),
                _ => panic!("invalid day {}, must be [1,24]", day),
            }?;
            println!("{}", result);
        }
    }
    Ok(())
}
macro_rules! timed {
    ($e: expr) => {{
        use std::time::Instant;
        let start = Instant::now();
        let solution = { $e };
        (solution, start.elapsed())
    }};
}

fn run_day<T>(aoc: &mut Aoc, mode: Mode, part: Part) -> Result<String, Box<dyn Error>>
where
    T: Solver,
{
    let level = match part {
        Part::One => Level::First,
        Part::Two => Level::Second,
    };
    let input = match mode {
        Mode::Sample => <T as Solver>::SAMPLE.to_string(),
        _ => aoc.get_input(false)?,
    };
    let expected = match (mode, level) {
        (Mode::Sample, Level::First) => Some(<T as Solver>::LEVEL1.to_string()),
        (Mode::Sample, Level::Second) => Some(<T as Solver>::LEVEL2.to_string()),
        (_, level) => aoc.solution.get(&level).cloned(),
    };
    let solver = <T as Solver>::parse(&input);
    let (solution, duration) = match level {
        Level::First => timed!(solver.part1()),
        Level::Second => timed!(solver.part2()),
    };

    let result = match mode {
        Mode::Print | Mode::Sample => {
            let qualifier = match expected {
                Some(expected) if solution.to_string() == expected => {
                    format!("({}correct{})", color::Fg(color::Green), style::Reset)
                }
                Some(expected) => {
                    format!(
                        "({}incorrect{}, expected {})",
                        color::Fg(color::Red),
                        style::Reset,
                        expected
                    )
                }
                None => "(???)".to_string(),
            };
            format!("{:?} {} {}", duration, solution.to_string(), qualifier)
        }
        Mode::Submit => aoc.submit(&solution.to_string())?,
    };

    Ok(result)
}

use aocf::{Aoc, Level};
use clap::Parser;
use std::error::Error;

use aoc2021::day1;
use aoc2021::day2;
use aoc2021::day3;
use aoc2021::Solver;

#[derive(clap::ArgEnum, Clone, Copy)]
enum Mode {
    Sample,
    Print,
    Submit,
}

#[derive(Clone, Copy)]
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

#[derive(clap::Parser)]
struct Opts {
    #[clap(short)]
    day: u8,
    #[clap(short)]
    part: Option<Part>,
    #[clap(arg_enum, default_value = "print")]
    mode: Mode,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    let aoc = Aoc::new()
        .parse_cli(false)
        .year(Some(2021))
        .day(Some(opts.day as u32))
        .init()?;

    let level = match opts.part {
        None => aoc.level,
        Some(Part::One) => Level::First,
        Some(Part::Two) => Level::Second,
    };

    println!("AOC - 2021 - day {} - {} part", opts.day, level);
    let result = match opts.day {
        1 => run_day::<day1::Solution>(aoc, opts.mode, level),
        2 => run_day::<day2::Solution>(aoc, opts.mode, level),
        3 => run_day::<day3::Solution>(aoc, opts.mode, level),
        4..=24 => Ok("not implemented".to_string()),
        _ => panic!("invalid day {}, must be [1,24]", opts.day),
    }?;

    Ok(println!("{}", result))
}

fn run_day<T>(mut aoc: Aoc, mode: Mode, level: Level) -> Result<String, Box<dyn Error>>
where
    T: Solver,
{
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
    let solution = match level {
        Level::First => solver.part1(),
        Level::Second => solver.part2(),
    };

    let result = match mode {
        Mode::Print | Mode::Sample => {
            let is_correct = match expected.clone() {
                Some(expected) if solution.to_string() == expected => " (correct)",
                _ => "",
            };
            let expected = expected.unwrap_or("???".to_string());
            format!(
                "Solution: {} expected: {}{}",
                solution.to_string(),
                expected,
                is_correct,
            )
        }
        Mode::Submit => aoc.submit(&solution.to_string())?,
    };

    Ok(result)
}

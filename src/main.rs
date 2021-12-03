use aocf::{Aoc, Level};
use clap::Parser;
use std::error::Error;

trait Day {
    const SAMPLE: &'static str;
    const LEVEL1: &'static str;
    const LEVEL2: &'static str;

    type Output: ToString;

    fn parse(input: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    fn level1(self) -> Result<Self::Output, Box<dyn Error>>;

    fn level2(self) -> Result<Self::Output, Box<dyn Error>>;
}

mod day1;
mod day2;
mod day3;

#[derive(clap::ArgEnum, Clone, Copy)]
enum Mode {
    Sample,
    Print,
    Submit,
}

#[derive(clap::Parser)]
struct Opts {
    #[clap(short)]
    day: u8,
    #[clap(short)]
    level: Option<u8>,
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

    let level = match opts.level {
        None => aoc.level,
        Some(1) => Level::First,
        Some(2) => Level::Second,
        Some(level) => panic!("invalid level {}, must be 1 or 2", level),
    };

    println!("AOC - 2021 - day {} - {} level", opts.day, level);
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
    T: Day,
{
    let input = match mode {
        Mode::Sample => <T as Day>::SAMPLE.to_string(),
        _ => aoc.get_input(false)?,
    };
    let expected = match (mode, level) {
        (Mode::Sample, Level::First) => Some(<T as Day>::LEVEL1.to_string()),
        (Mode::Sample, Level::Second) => Some(<T as Day>::LEVEL2.to_string()),
        (_, level) => aoc.solution.get(&level).cloned(),
    };
    let solver = <T as Day>::parse(&input)?;
    let solution = match level {
        Level::First => solver.level1()?,
        Level::Second => solver.level2()?,
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

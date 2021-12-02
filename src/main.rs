use aocf::{Aoc, Level};
use clap::Parser;
use std::error::Error;

trait Day {
    const SAMPLE: &'static str;
    const LEVEL1: &'static str;
    const LEVEL2: &'static str;

    type Input;
    type Output: ToString;

    fn parse(input: &str) -> Result<Self::Input, Box<dyn Error>>;

    fn level1(input: Self::Input) -> Result<Self::Output, Box<dyn Error>>;

    fn level2(input: Self::Input) -> Result<Self::Output, Box<dyn Error>>;
}

mod day1;

#[derive(clap::ArgEnum, Clone)]
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
        _ => Ok("not implemented".to_string()),
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
    let input = <T as Day>::parse(&input)?;
    let (solution, expected) = match level {
        Level::First => (<T as Day>::level1(input)?, <T as Day>::LEVEL1),
        Level::Second => (<T as Day>::level2(input)?, <T as Day>::LEVEL2),
    };

    let result = match mode {
        Mode::Print => solution.to_string(),
        Mode::Sample => format!("Solution: {} expected: {}", solution.to_string(), expected),
        Mode::Submit => aoc.submit(&solution.to_string())?,
    };

    Ok(result)
}

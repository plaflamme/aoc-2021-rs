use aocf::Aoc;
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

enum Days {
    Set(Vec<u8>),
    Range(u8, u8),
}

impl std::str::FromStr for Days {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((from, to)) = s.split_once("..") {
            let from = u8::from_str(from).unwrap_or(1);
            let to = u8::from_str(to).unwrap_or(25);
            Ok(Days::Range(from, to))
        } else if s.to_ascii_lowercase() == "all" {
            Ok(Days::Range(1, 25))
        } else {
            Ok(Days::Set(
                s.split(',')
                    .map(u8::from_str)
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

    #[clap(short, parse(from_occurrences))]
    verbose: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    stderrlog::new()
        .timestamp(stderrlog::Timestamp::Off)
        .verbosity(opts.verbose)
        .init()?;

    println!("AoC - 2021");

    let parts: Vec<Part> = match opts.part {
        None => vec![Part::One, Part::Two],
        Some(part) => vec![part],
    };

    for day in opts.days {
        println!("Day {}", day);
        match day {
            1 => run_main(Day1, parts.clone(), opts.mode),
            2 => run_main(Day2, parts.clone(), opts.mode),
            3 => run_main(Day3, parts.clone(), opts.mode),
            4 => run_main(Day4, parts.clone(), opts.mode),
            5 => run_main(Day5, parts.clone(), opts.mode),
            6 => run_main(Day6, parts.clone(), opts.mode),
            7 => run_main(Day7, parts.clone(), opts.mode),
            8..=25 => println!("  not implemented"),
            _ => panic!("invalid day {}, must be [1,24]", day),
        };
    }
    Ok(())
}

fn run_main<D: Day>(day: D, parts: Vec<Part>, mode: Mode)
where
    D: Solver<Main> + Sample + Clone + 'static,
{
    run_alt(day, Main, parts, mode);
}

fn run_alt<D: Day, A>(day: D, alt: A, parts: Vec<Part>, mode: Mode)
where
    D: Solver<A> + Sample + Clone + 'static,
    A: core::fmt::Debug,
{
    run(day.clone(), alt, parts, mode, load_input(day, mode));
}

fn run<D: Day, A>(_day: D, alt: A, parts: Vec<Part>, mode: Mode, mut input: Box<dyn Input>)
where
    D: Solver<A>,
    A: core::fmt::Debug,
{
    let loaded = input.load();
    for part in parts {
        print!("  - part {:?} ... {:?} ... ", part, alt);
        let (solution, duration) = solve_part::<D, A>(&loaded, part);
        let qualifier = match input.solution(part) {
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
        println!("{:?} {} {}", duration, solution.to_string(), qualifier);
        match mode {
            Mode::Submit => {
                submit::<D>(part, solution.to_string());
            }
            _ => (),
        }
    }
}

fn submit<D: Day>(part: Part, solution: String) {
    let mut aoc = Aoc::new()
        .parse_cli(false)
        .year(Some(2021))
        .day(Some(<D as Day>::DAY as u32))
        .init()
        .unwrap();

    if let None = aoc.solution(part) {
        println!(
            "Submitting solution {} for day {} part {:?}",
            solution,
            D::DAY,
            part
        );

        println!("{}", aoc.submit(&solution).unwrap());
    } else {
        println!(
            "Solution for day {} part {:?} has already been accepted.",
            D::DAY,
            part
        );
    }
}

fn load_input<D: Day + Sample + 'static>(day: D, mode: Mode) -> Box<dyn Input> {
    match mode {
        Mode::Sample => Box::new(day),
        _ => Box::new(
            Aoc::new()
                .parse_cli(false)
                .year(Some(2021))
                .day(Some(<D as Day>::DAY as u32))
                .init()
                .unwrap(),
        ),
    }
}

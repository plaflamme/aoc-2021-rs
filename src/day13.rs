use std::collections::HashSet;

use itertools::Itertools;

use crate::{Day13, Solver};

sample!(
    Day13,
    "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5",
    "17"
);

type Pt = crate::grid::Pt<u32>;

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(u32),
    Y(u32),
}

pub struct Paper(HashSet<Pt>, Vec<Fold>);

fn fold_paper(dots: HashSet<Pt>, along: Fold) -> HashSet<Pt> {
    dots.into_iter()
        .map(|mut pt| match along {
            Fold::Y(y) => {
                if pt.y > y {
                    let dy = pt.y - y;
                    pt.y = y - dy;
                }
                pt
            }
            Fold::X(x) => {
                if pt.x > x {
                    let dx = pt.x - x;
                    pt.x = x - dx;
                }
                pt
            }
        })
        .collect()
}

fn print(dots: &HashSet<Pt>) -> Result<String, Box<dyn std::error::Error>> {
    use std::fmt::Write as FmtWrite;
    let max_x = dots.iter().map(|pt| pt.x).max().unwrap();
    let max_y = dots.iter().map(|pt| pt.y).max().unwrap();

    let mut out = String::new();
    writeln!(&mut out, "")?;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&crate::grid::Pt(x, y)) {
                write!(&mut out, "#")?;
            } else {
                write!(&mut out, " ")?;
            }
        }
        writeln!(&mut out, "")?;
    }
    Ok(out)
}

impl Solver for Day13 {
    type Output = String;

    type Input = Paper;

    fn parse(input: &str) -> Self::Input {
        let (dots, folds) = crate::tools::empty_line_delimited_batches(input.lines())
            .tuples()
            .exactly_one()
            .unwrap_or_else(|_| panic!("invalid input"));

        let dots = dots
            .into_iter()
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<Pt>().unwrap())
            .collect();

        let folds = folds
            .into_iter()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let (along, value) = l.split_once("=").unwrap();
                match along.chars().last().unwrap() {
                    'x' => Fold::X(value.parse::<u32>().unwrap()),
                    'y' => Fold::Y(value.parse::<u32>().unwrap()),
                    _ => panic!("invalid fold line {}", l),
                }
            })
            .collect();
        Paper(dots, folds)
    }

    fn part1(input: Self::Input) -> Self::Output {
        let first = input.1.first().unwrap();
        fold_paper(input.0, *first).len().to_string()
    }

    fn part2(input: Self::Input) -> Self::Output {
        let dots = input
            .1
            .into_iter()
            .fold(input.0, |dots, fold| fold_paper(dots, fold));

        print(&dots).unwrap()
    }
}

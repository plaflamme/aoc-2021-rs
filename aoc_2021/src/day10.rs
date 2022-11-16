use itertools::{Either, Itertools};

use aoc_lib::*;
day!(Day10, 10);

sample!(
    Day10,
    "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
    "26397",
    "288957"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Paren {
    Round,
    Square,
    Curly,
    Pointy, // I guess?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Open,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bracket(Paren, Dir);

impl Bracket {
    fn from_char(c: char) -> Self {
        match c {
            '[' => Bracket(Paren::Square, Dir::Open),
            ']' => Bracket(Paren::Square, Dir::Close),
            '(' => Bracket(Paren::Round, Dir::Open),
            ')' => Bracket(Paren::Round, Dir::Close),
            '{' => Bracket(Paren::Curly, Dir::Open),
            '}' => Bracket(Paren::Curly, Dir::Close),
            '<' => Bracket(Paren::Pointy, Dir::Open),
            '>' => Bracket(Paren::Pointy, Dir::Close),
            _ => panic!("invalid bracket {}", c),
        }
    }
}

fn solve(input: Vec<Bracket>) -> Either<Paren, Vec<Paren>> {
    let mut stack = Vec::new();

    let invalid = input
        .into_iter()
        .flat_map(|b| match b {
            Bracket(p, Dir::Open) => {
                stack.push(p);
                None
            }
            Bracket(p, Dir::Close) => match stack.pop() {
                None => Some(p),
                Some(opening) if opening != p => Some(p),
                _ => None,
            },
        })
        .at_most_one()
        .unwrap();

    if let Some(p) = invalid {
        Either::Left(p)
    } else {
        stack.reverse();
        Either::Right(stack)
    }
}

impl Solver for Day10 {
    type Output = u32;

    type Input = Vec<Vec<Bracket>>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(|s| s.chars().map(Bracket::from_char).collect())
            .collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        input
            .into_iter()
            .flat_map(|l| solve(l).left())
            .map(|c| match c {
                Paren::Round => 3,
                Paren::Square => 57,
                Paren::Curly => 1197,
                Paren::Pointy => 25137,
            })
            .sum::<u32>()
    }

    fn part2(input: Self::Input) -> Self::Output {
        let scores = input
            .into_iter()
            .flat_map(|l| solve(l).right())
            .map(|l| {
                l.into_iter()
                    .map(|c| match c {
                        Paren::Round => 1,
                        Paren::Square => 2,
                        Paren::Curly => 3,
                        Paren::Pointy => 4,
                    })
                    .fold(0_u64, |score, char_score| score * 5 + char_score)
            })
            .sorted()
            .collect_vec();

        scores[scores.len() / 2] as u32
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_something() {
        let c = '(';
        let closing = ')';

        match (c, closing) {
            ('(', ')') => (),
            _ => panic!("damn"),
        }
    }
}

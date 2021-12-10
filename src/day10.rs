use itertools::Itertools;

use crate::{Day10, Solver};

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

fn first_illegal_char(s: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '[' | '(' | '{' | '<' => stack.push(c),
            ']' | ')' | '}' | '>' => match stack.pop() {
                None => return Some(c),

                Some(opening) => match (opening, c) {
                    ('[', ']') | ('(', ')') | ('{', '}') | ('<', '>') => (),
                    _ => {
                        return Some(c);
                    }
                },
            },
            _ => panic!("illegal char {}", c),
        }
    }
    None
}

fn closing_sequence(s: &str) -> Vec<char> {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '[' | '(' | '{' | '<' => stack.push(c),
            ']' | ')' | '}' | '>' => {
                stack.pop();
            } //assumes the input was sanitized
            _ => panic!("illegal char {}", c),
        }
    }
    stack
        .into_iter()
        .map(|c| match c {
            '[' => ']',
            '(' => ')',
            '{' => '}',
            '<' => '>',
            _ => panic!("illegal opening {}", c),
        })
        .rev()
        .collect()
}

impl Solver for Day10 {
    type Output = u32;

    type Input = Vec<String>;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|s| s.to_string()).collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        input
            .into_iter()
            .flat_map(|l| first_illegal_char(&l))
            .map(|c| match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("invalid illegal char {}", c),
            })
            .sum::<u32>()
    }

    fn part2(input: Self::Input) -> Self::Output {
        let scores = input
            .into_iter()
            .filter(|l| first_illegal_char(&l).is_none())
            .map(|l| {
                let seq = closing_sequence(&l);
                log::debug!("seq: {:?}", seq);
                seq.into_iter()
                    .map(|c| match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("invalid closing char {}", c),
                    })
                    .fold(0_u64, |score, char_score| score * 5 + char_score)
            })
            .inspect(|s| log::debug!("score: {}", s))
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

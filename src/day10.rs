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
    "26397"
);

fn first_illegal_char(s: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in s.chars() {
        log::debug!("char is {}", c);
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
        todo!()
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

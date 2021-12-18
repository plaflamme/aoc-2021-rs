use crate::{Day18, Solver};

sample!(Day18, "", "");

#[derive(Debug, Clone, Eq, PartialEq)]
enum Number {
    Single(u8),
    More(Box<Fish>),
}

impl Number {
    fn single(n: u8) -> Self {
        Number::Single(n)
    }
    fn more(o: Option<Fish>) -> Self {
        match o {
            Some(f) => Number::More(Box::new(f)),
            None => Number::Single(0),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Fish(Number, Number);

fn parse_number(input: &str) -> (Number, &str) {
    let mut chars = input.chars();
    match chars.next().unwrap() {
        '[' => {
            let (more, rest) = parse_pair(input);
            (Number::More(Box::new(more)), rest)
        }
        c @ '0'..='9' => (
            Number::Single(c.to_string().parse::<u8>().unwrap()),
            chars.as_str(),
        ),
        invalid => panic!("unexpected {}", invalid),
    }
}

// 1,2
fn parse_pair(input: &str) -> (Fish, &str) {
    let (c, rest) = input.split_at(1);
    if c != "[" {
        panic!("expected [ got {}", c);
    }
    let (first, rest) = parse_number(rest);
    let (c, rest) = rest.split_at(1);
    if c != "," {
        panic!("expected , got {}", c);
    }
    let (second, rest) = parse_number(rest);
    let (c, rest) = rest.split_at(1);
    if c != "]" {
        panic!("expected ] got {}", c);
    }
    (Fish(first, second), rest)
}

fn parse(input: &str) -> Fish {
    let (fish, _) = parse_pair(input);
    fish
}

impl Fish {
    fn add_left(&mut self, add: u8) {
        match &mut self.0 {
            Number::Single(a) => *a += add,
            Number::More(f) => f.add_left(add),
        }
    }
    fn add_right(&mut self, add: u8) {
        match &mut self.1 {
            Number::Single(a) => *a += add,
            Number::More(f) => f.add_right(add),
        }
    }
}

struct ExplodeOutcome(u8, u8, Option<Fish>);

fn explode_rec(f: &Fish, depth: u8) -> Option<ExplodeOutcome> {
    match f {
        Fish(Number::Single(left), Number::Single(right)) if depth == 4 => {
            Some(ExplodeOutcome(*left, *right, None))
        }
        Fish(Number::Single(_), Number::Single(_)) => None,
        Fish(Number::More(left), Number::More(right)) => {
            if let Some(ExplodeOutcome(a, b, f)) = explode_rec(&left, depth + 1) {
                let mut new_right = right.clone();
                new_right.add_left(b);
                Some(ExplodeOutcome(
                    a,
                    0,
                    Some(Fish(Number::more(f), Number::More(new_right))),
                ))
            } else if let Some(ExplodeOutcome(a, b, f)) = explode_rec(&right, depth + 1) {
                let mut new_left = left.clone();
                new_left.add_right(a);
                Some(ExplodeOutcome(
                    a,
                    b,
                    Some(Fish(Number::More(new_left), Number::more(f))),
                ))
            } else {
                None
            }
        }

        Fish(Number::Single(left), Number::More(right)) => match explode_rec(right, depth + 1) {
            Some(ExplodeOutcome(a, b, f)) => Some(ExplodeOutcome(
                0,
                b,
                Some(Fish(Number::single(left + a), Number::more(f))),
            )),
            None => None,
        },

        Fish(Number::More(left), Number::Single(right)) => match explode_rec(left, depth + 1) {
            Some(ExplodeOutcome(a, b, f)) => Some(ExplodeOutcome(
                a,
                0,
                Some(Fish(Number::more(f), Number::single(right + b))),
            )),
            None => None,
        },
    }
}

fn explode(f: Fish) -> Option<Fish> {
    if let Some(ExplodeOutcome(_, _, Some(fish))) = explode_rec(&f, 0) {
        Some(fish)
    } else {
        None
    }
}

impl Solver for Day18 {
    type Output = usize;

    type Input = Vec<Fish>;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|l| parse(l.trim())).collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        todo!()
    }

    fn part2(input: Self::Input) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("[1,2]"), Fish(Number::Single(1), Number::Single(2)));
        assert_eq!(
            parse("[1,[2,3]]"),
            Fish(
                Number::Single(1),
                Number::More(Box::new(Fish(Number::Single(2), Number::Single(3))))
            )
        );
        assert_eq!(
            parse("[[1,2],3]"),
            Fish(
                Number::More(Box::new(Fish(Number::Single(1), Number::Single(2)))),
                Number::Single(3),
            )
        );
    }

    #[test]
    fn test_explode() {
        let fish = parse("[[[[[9,8],1],2],3],4]");
        assert_eq!(explode(fish), Some(parse("[[[[0,9],2],3],4]")));

        let fish = parse("[7,[6,[5,[4,[3,2]]]]]");
        assert_eq!(explode(fish), Some(parse("[7,[6,[5,[7,0]]]]")));

        let fish = parse("[[6,[5,[4,[3,2]]]],1]");
        assert_eq!(explode(fish), Some(parse("[[6,[5,[7,0]]],3]")));

        let fish = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        assert_eq!(
            explode(fish.clone()),
            Some(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
        );
        assert_eq!(
            explode(explode(fish).unwrap()),
            Some(parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"))
        );

        let fish = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(explode(fish), Some(parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")));
    }
}

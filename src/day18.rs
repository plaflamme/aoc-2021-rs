use std::fmt::Display;

use crate::{Day18, Solver};
use itertools::Itertools;
use num::Integer;
use text_trees::{StringTreeNode, TreeNode};

sample!(
    Day18,
    "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
    "4140"
);

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

impl Fish {
    fn new(left: Fish, right: Fish) -> Self {
        Fish(Number::More(Box::new(left)), Number::More(Box::new(right)))
    }
}

fn parse_number(input: &str) -> (Number, &str) {
    match input.chars().next().unwrap() {
        '[' => {
            let (more, rest) = parse_pair(input);
            (Number::More(Box::new(more)), rest)
        }
        '0'..='9' => {
            let n = String::from_iter(input.chars().take_while(|c| ('0'..='9').contains(c)));
            let (_, rest) = input.split_at(n.len());
            (Number::Single(n.parse::<u8>().unwrap()), rest)
        }

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
            Number::More(fish) => fish.add_left(add),
        }
    }
    fn add_right(&mut self, add: u8) {
        match &mut self.1 {
            Number::Single(a) => *a += add,
            Number::More(fish) => fish.add_right(add),
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
                    0,
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

fn split_n(n: Number) -> Option<Number> {
    match n {
        Number::Single(v) if v >= 10 => Some(Number::More(Box::new(Fish(
            Number::single(v.div_floor(&2)),
            Number::single(v.div_ceil(&2)),
        )))),
        Number::More(box Fish(left, right)) => {
            if let Some(n) = split_n(left.clone()) {
                Some(Number::More(Box::new(Fish(n, right))))
            } else if let Some(n) = split_n(right.clone()) {
                Some(Number::More(Box::new(Fish(left, n))))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn split(f: Fish) -> Option<Fish> {
    if let Some(Number::More(box f)) = split_n(Number::More(Box::new(f))) {
        Some(f)
    } else {
        None
    }
}

fn reduce(fish: Fish) -> Fish {
    let mut fish = fish;
    loop {
        if let Some(f) = explode(fish.clone()) {
            log::debug!("exp: {}", f);
            fish = f;
            continue;
        } else if let Some(f) = split(fish.clone()) {
            log::debug!("spl: {}", f);
            fish = f;
            continue;
        } else {
            break;
        }
    }
    fish
}

fn sum(left: Fish, right: Fish) -> Fish {
    reduce(Fish(
        Number::More(Box::new(left)),
        Number::More(Box::new(right)),
    ))
}

fn sum_vec(fish: Vec<Fish>) -> Fish {
    fish.into_iter().reduce(sum).unwrap()
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

fn to_str(lr: &str, n: Number) -> StringTreeNode {
    match n {
        Number::Single(s) => StringTreeNode::new(s.to_string()),
        Number::More(box Fish(l, r)) => StringTreeNode::with_child_nodes(
            lr.to_string(),
            vec![to_str("r", r), to_str("l", l)].into_iter(),
        ),
    }
}

fn print(f: Fish) -> String {
    to_str("", Number::More(Box::new(f))).to_string()
}

impl Display for Fish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", print(self.clone()))
    }
}

#[cfg(test)]
mod test {
    use crate::Sample;

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

    #[test]
    fn test_split() {
        let fish = parse("[[[[0,7],4],[15,[0,13]]],[1,1]]");
        assert_eq!(
            split(fish.clone()),
            Some(parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"))
        );
        assert_eq!(
            split(split(fish).unwrap()),
            Some(parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"))
        );
    }

    #[test]
    fn test_reduce() {
        let fish = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        assert_eq!(reduce(fish), parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_simple_sum() {
        let fish = <Day18 as Solver>::parse(
            "[1,1]
[2,2]
[3,3]
[4,4]",
        );
        assert_eq!(sum_vec(fish), parse("[[[[1,1],[2,2]],[3,3]],[4,4]]"));

        let fish = <Day18 as Solver>::parse(
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]",
        );
        assert_eq!(sum_vec(fish), parse("[[[[3,0],[5,3]],[4,4]],[5,5]]"));

        let fish = <Day18 as Solver>::parse(
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]",
        );
        assert_eq!(sum_vec(fish), parse("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    }

    #[test]
    fn test_sum() {
        let fish = <Day18 as Solver>::parse(Day18::CONTENT);
        let fish = fish.into_iter().reduce(|a, b| sum(a, b)).unwrap();
        assert_eq!(
            fish,
            parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }
}

use std::fmt::Display;

use crate::{Day18, Solver};
use itertools::Itertools;
use num::Integer;
use text_trees::StringTreeNode;

sample!(
    Day18,
    "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    "4140",
    "3993"
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
    Leaf(u8),
    Branch(Box<Node>, Box<Node>),
}

impl Node {
    fn branch(left: Node, right: Node) -> Self {
        Node::Branch(Box::new(left), Box::new(right))
    }
}

impl Node {
    fn add_left(&mut self, add: u8) {
        match self {
            Node::Leaf(a) => *a += add,
            Node::Branch(box left, _) => left.add_left(add),
        }
    }
    fn add_right(&mut self, add: u8) {
        match self {
            Node::Leaf(a) => *a += add,
            Node::Branch(_, box right) => right.add_right(add),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Node::Leaf(v) if *v >= 10 => {
                *self = Node::branch(Node::Leaf(v.div_floor(&2)), Node::Leaf(v.div_ceil(&2)));
                true
            }
            Node::Branch(box left, box right) => left.split() || right.split(),
            _ => false,
        }
    }

    fn explode_rec(&mut self, depth: u8) -> Option<(u8, u8)> {
        if depth == 4 {
            // TODO: how do I avoid the clone here?
            if let Node::Branch(box Node::Leaf(left), box Node::Leaf(right)) = self.clone() {
                *self = Node::Leaf(0);
                return Some((left, right));
            }
        }

        match self {
            Node::Leaf(_) => None,
            Node::Branch(box left, box right) => {
                if let Some((a, b)) = left.explode_rec(depth + 1) {
                    right.add_left(b);
                    Some((a, 0))
                } else if let Some((a, b)) = right.explode_rec(depth + 1) {
                    left.add_right(a);
                    Some((0, b))
                } else {
                    None
                }
            }
        }
    }
    fn explode(&mut self) -> Option<(u8, u8)> {
        self.explode_rec(0)
    }
}

impl From<&Node> for StringTreeNode {
    fn from(n: &Node) -> Self {
        match n {
            Node::Leaf(s) => StringTreeNode::new(s.to_string()),
            Node::Branch(box l, box r) => {
                StringTreeNode::with_child_nodes(String::new(), vec![r, l].into_iter().map_into())
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", StringTreeNode::from(self))
    }
}

fn parse_node(input: &str) -> (Node, &str) {
    match input.chars().next().unwrap() {
        '[' => parse_branch(input),
        '0'..='9' => {
            let n = String::from_iter(input.chars().take_while(|c| ('0'..='9').contains(c)));
            let (_, rest) = input.split_at(n.len());
            (Node::Leaf(n.parse::<u8>().unwrap()), rest)
        }

        invalid => panic!("unexpected {}", invalid),
    }
}

fn parse_branch(input: &str) -> (Node, &str) {
    let (c, rest) = input.split_at(1);
    if c != "[" {
        panic!("expected [ got {}", c);
    }
    let (first, rest) = parse_node(rest);
    let (c, rest) = rest.split_at(1);
    if c != "," {
        panic!("expected , got {}", c);
    }
    let (second, rest) = parse_node(rest);
    let (c, rest) = rest.split_at(1);
    if c != "]" {
        panic!("expected ] got {}", c);
    }
    (Node::branch(first, second), rest)
}

fn parse(input: &str) -> Node {
    let (node, _) = parse_branch(input);
    node
}

fn reduce(fish: Node) -> Node {
    let mut fish = fish;
    loop {
        if let Some(_) = fish.explode() {
            log::debug!("exp: {}", fish);
            continue;
        } else if fish.split() {
            log::debug!("spl: {}", fish);
            continue;
        } else {
            break;
        }
    }
    fish
}

fn sum(left: Node, right: Node) -> Node {
    reduce(Node::branch(left, right))
}

fn sum_vec(fish: Vec<Node>) -> Node {
    fish.into_iter().reduce(sum).unwrap()
}

fn magnitude(n: &Node) -> usize {
    match n {
        Node::Leaf(v) => *v as usize,
        Node::Branch(box left, box right) => magnitude(left) * 3 + magnitude(right) * 2,
    }
}

impl Solver for Day18 {
    type Output = usize;

    type Input = Vec<Node>;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|l| parse(l.trim())).collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        magnitude(&sum_vec(input))
    }

    fn part2(input: Self::Input) -> Self::Output {
        input
            .into_iter()
            .combinations(2)
            .flat_map(|mut c| {
                let a = magnitude(&sum_vec(c.clone()));
                c.reverse();
                let b = magnitude(&sum_vec(c));
                vec![a, b]
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::Sample;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("[1,2]"), Node::branch(Node::Leaf(1), Node::Leaf(2)));
        assert_eq!(
            parse("[1,[2,3]]"),
            Node::branch(Node::Leaf(1), Node::branch(Node::Leaf(2), Node::Leaf(3)))
        );
        assert_eq!(
            parse("[[1,2],3]"),
            Node::branch(Node::branch(Node::Leaf(1), Node::Leaf(2)), Node::Leaf(3),)
        );
    }

    #[test]
    fn test_explode() {
        let mut fish = parse("[[[[[9,8],1],2],3],4]");
        fish.explode();
        assert_eq!(fish, parse("[[[[0,9],2],3],4]"));

        let mut fish = parse("[7,[6,[5,[4,[3,2]]]]]");
        fish.explode();
        assert_eq!(fish, parse("[7,[6,[5,[7,0]]]]"));

        let mut fish = parse("[[6,[5,[4,[3,2]]]],1]");
        fish.explode();
        assert_eq!(fish, parse("[[6,[5,[7,0]]],3]"));

        let mut fish = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        fish.explode();
        assert_eq!(fish, parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
        fish.explode();
        assert_eq!(fish, parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));

        let mut fish = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        fish.explode();
        assert_eq!(fish, parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }

    #[test]
    fn test_split() {
        let mut fish = parse("[[[[0,7],4],[15,[0,13]]],[1,1]]");
        assert!(fish.split());
        assert_eq!(fish, parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));
        assert!(fish.split());
        assert_eq!(fish, parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"));
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
        assert_eq!(
            sum_vec(fish),
            parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(magnitude(&parse("[9,1]")), 29);
        assert_eq!(magnitude(&parse("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(magnitude(&parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")), 1384);
        assert_eq!(magnitude(&parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
        assert_eq!(magnitude(&parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")), 791);
        assert_eq!(magnitude(&parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")), 1137);
        assert_eq!(
            magnitude(&parse(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )),
            3488
        );
    }
}

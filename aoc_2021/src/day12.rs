use std::collections::{HashMap, HashSet};

use aoc_lib::*;
day!(Day12, 12);

sample!(
    Day12,
    "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
    "226",
    "3509"
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Cave {
    fn from_str(s: &str) -> Self {
        if s == "start" {
            Cave::Start
        } else if s == "end" {
            Cave::End
        } else if s.chars().all(|c| c.is_uppercase()) {
            Cave::Big(s.to_string())
        } else {
            Cave::Small(s.to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge(Cave, Cave);

type Edges = HashMap<Cave, Vec<Cave>>;
// The problem does not require keeping the order of the nodes
type Path = HashSet<Cave>;

fn dfs(path: &Path, candidate: &Cave, has_revisited: bool, edges: &Edges) -> usize {
    match candidate {
        Cave::End => 1,
        Cave::Small(_) if path.contains(candidate) && has_revisited => 0,
        _ => {
            let mut path = path.clone();
            let new_visit = path.insert(candidate.clone());
            let has_revisited = if let Cave::Small(_) = candidate {
                !new_visit || has_revisited
            } else {
                has_revisited
            };

            edges
                .get(candidate)
                .unwrap_or(&vec![])
                .iter()
                .map(|next| dfs(&path, next, has_revisited, edges))
                .sum::<usize>()
        }
    }
}

fn solve(paths: Vec<Edge>, allow_revisits: bool) -> usize {
    let mut edges: HashMap<Cave, Vec<Cave>> = HashMap::new();

    paths.into_iter().for_each(|p| {
        let (from, to) = (p.0, p.1);
        if to != Cave::Start {
            edges.entry(from.clone()).or_default().push(to.clone());
        }
        if from != Cave::Start {
            edges.entry(to).or_default().push(from);
        }
    });

    dfs(&HashSet::new(), &Cave::Start, !allow_revisits, &edges)
}

impl Solver for Day12 {
    type Output = usize;

    type Input = Vec<Edge>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|path| {
                let (a, b) = path
                    .split_once('-')
                    .unwrap_or_else(|| panic!("unexpected line {}", path));
                Edge(Cave::from_str(a), Cave::from_str(b))
            })
            .collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        solve(input, false)
    }

    fn part2(input: Self::Input) -> Self::Output {
        solve(input, true)
    }
}

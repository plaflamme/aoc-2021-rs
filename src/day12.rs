use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{Day12, Solver};

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
type Path = Vec<Cave>;

fn dfs(path: Path, visited: &HashSet<Cave>, has_revisited: bool, edges: &Edges) -> Vec<Path> {
    match path.last() {
        Some(Cave::End) => vec![path],
        Some(c @ Cave::Small(_)) if visited.contains(c) && has_revisited => vec![],
        Some(c) => {
            // NOTE: not sure how to avoid the clone here since we only need to do it in one of the branches
            let mut visited = visited.clone();
            let has_revisited = if let Cave::Small(_) = c {
                // we've revisited if we visit this cave for the second time or if we've already revisited
                !visited.insert(c.clone()) || has_revisited
            } else {
                has_revisited
            };

            edges
                .get(c)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .flat_map(|next| {
                    let mut path = path.clone();
                    path.push(next);
                    dfs(path, &visited, has_revisited, edges)
                })
                .collect_vec()
        }
        None => panic!("path should have at least one node"),
    }
}

fn solve(paths: Vec<Edge>, allow_revisits: bool) -> Vec<Path> {
    let mut edges: HashMap<Cave, Path> = HashMap::new();

    paths.into_iter().for_each(|p| {
        let (from, to) = (p.0, p.1);
        if to != Cave::Start {
            edges.entry(from.clone()).or_default().push(to.clone());
        }
        if from != Cave::Start {
            edges.entry(to).or_default().push(from);
        }
    });

    dfs(vec![Cave::Start], &HashSet::new(), !allow_revisits, &edges)
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
                    .split_once("-")
                    .unwrap_or_else(|| panic!("unexpected line {}", path));
                Edge(Cave::from_str(a), Cave::from_str(b))
            })
            .collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        solve(input, false).len()
    }

    fn part2(input: Self::Input) -> Self::Output {
        solve(input, true).len()
    }
}

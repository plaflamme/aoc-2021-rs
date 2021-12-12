use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{Day12, Solver};

sample!(
    Day12,
    "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
    "19"
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
pub struct Path(Cave, Cave);

type Edges = HashMap<Cave, Vec<Cave>>;

fn step_dfs(path: Vec<Cave>, smalls: &HashSet<Cave>, edges: &Edges) -> Vec<Vec<Cave>> {
    if let Some(Cave::End) = path.last() {
        vec![path]
    } else {
        if let Some(next) = edges.get(path.last().unwrap()) {
            let mut more = Vec::new();
            for n in next {
                if smalls.contains(n) || *n == Cave::Start {
                    continue;
                }
                let mut path = path.clone();
                path.push(n.clone());
                let stepped = if let Cave::Small(_) = n {
                    let mut smalls = smalls.clone();
                    smalls.insert(n.clone());

                    step_dfs(path, &smalls, edges)
                } else {
                    step_dfs(path, smalls, edges)
                };

                more.extend(stepped);
            }
            more
        } else {
            vec![]
        }
    }
}

fn step_dfs_2(
    path: Vec<Cave>,
    smalls: &HashSet<Cave>,
    special_small: &Option<Cave>,
    edges: &Edges,
) -> Vec<Vec<Cave>> {
    if let Some(Cave::End) = path.last() {
        vec![path]
    } else {
        if let Some(next) = edges.get(path.last().unwrap()) {
            let mut more = Vec::new();
            for n in next {
                if *n == Cave::Start {
                    continue;
                }

                if smalls.contains(n) && special_small.is_some() {
                    continue;
                }

                let mut path = path.clone();
                path.push(n.clone());
                let stepped = if let Cave::Small(_) = n {
                    let mut smalls = smalls.clone();
                    let special = if smalls.insert(n.clone()) {
                        special_small.clone()
                    } else {
                        if special_small.is_none() {
                            Some(n.clone())
                        } else {
                            special_small.clone()
                        }
                    };

                    step_dfs_2(path, &smalls, &special, edges)
                } else {
                    step_dfs_2(path, smalls, special_small, edges)
                };

                more.extend(stepped);
            }
            more
        } else {
            vec![]
        }
    }
}

fn solve(paths: Vec<Path>) -> Vec<Vec<Cave>> {
    let mut edges: HashMap<Cave, Vec<Cave>> = HashMap::new();

    paths.into_iter().for_each(|p| {
        let (from, to) = (p.0, p.1);
        edges.entry(from.clone()).or_default().push(to.clone());
        edges.entry(to).or_default().push(from);
    });

    let start = vec![Cave::Start];
    let result = step_dfs(start, &HashSet::new(), &edges);

    for solution in result.clone() {
        log::debug!("dfs_me: {:?}", solution);
    }
    result
}

fn solve2(paths: Vec<Path>) -> Vec<Vec<Cave>> {
    let mut edges: HashMap<Cave, Vec<Cave>> = HashMap::new();

    paths.into_iter().for_each(|p| {
        let (from, to) = (p.0, p.1);
        edges.entry(from.clone()).or_default().push(to.clone());
        edges.entry(to).or_default().push(from);
    });

    let start = vec![Cave::Start];
    let result = step_dfs_2(start, &HashSet::new(), &None, &edges);

    for solution in result.clone() {
        log::debug!("dfs_me: {:?}", solution);
    }
    result
}

impl Solver for Day12 {
    type Output = usize;

    type Input = Vec<Path>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|path| {
                let (a, b) = path
                    .split_once("-")
                    .unwrap_or_else(|| panic!("unexpected line {}", path));
                Path(Cave::from_str(a), Cave::from_str(b))
            })
            .collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        solve(input.clone()).len()
    }

    fn part2(input: Self::Input) -> Self::Output {
        solve2(input.clone()).len()
    }
}

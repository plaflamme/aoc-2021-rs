use std::{collections::HashSet, iter::once};

use itertools::Itertools;

use aoc_lib::*;
day!(Day9, 9);

sample!(
    Day9,
    "2199943210
3987894921
9856789892
8767896789
9899965678",
    "15",
    "1134"
);

type Pt = crate::grid::Pt<u32>;
type Grid = crate::grid::Grid<u32>;

impl Grid {
    fn lows(&self) -> impl Iterator<Item = Pt> + '_ {
        self.pts().filter(|pt| {
            let this_depth = self[*pt];
            pt.neighbours_checked(self.width() as u32, self.height() as u32)
                .map(|pt| self[pt])
                .all(|depth| depth > this_depth)
        })
    }

    fn rec_neighs(&self, pt: Pt, visited: &mut HashSet<Pt>) -> Vec<Pt> {
        visited.insert(pt);
        pt.neighbours_checked(self.width() as u32, self.height() as u32)
            .filter(|n| self[*n] < 9)
            .flat_map(|n| {
                if !visited.contains(&n) {
                    self.rec_neighs(n, visited)
                } else {
                    Vec::new()
                }
            })
            .chain(once(pt))
            .collect()
    }

    fn basins(&self) -> impl Iterator<Item = Vec<Pt>> + '_ {
        let mut visited = HashSet::new();
        self.lows()
            .map(move |pt| self.rec_neighs(pt, &mut visited))
            .inspect(|b| log::debug!("{:?}", b))
    }
}

impl Solver for Day9 {
    type Output = u32;

    type Input = Grid;

    fn parse(input: &str) -> Self::Input {
        Grid::from_row_iter(input.lines().filter(|l| !l.trim().is_empty()).map(|l| {
            l.trim()
                .chars()
                .map(|c| {
                    c.to_string()
                        .parse::<u32>()
                        .unwrap_or_else(|_| panic!("invalid line {}", l))
                })
                .collect_vec()
        }))
    }

    fn part1(input: Self::Input) -> Self::Output {
        input.lows().map(|low| input[low] + 1).sum::<u32>()
    }

    fn part2(input: Self::Input) -> Self::Output {
        input
            .basins()
            .map(|basin| basin.len() as u32)
            .sorted()
            .rev()
            .take(3)
            .product::<u32>()
    }
}

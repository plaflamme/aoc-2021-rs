use std::iter::once;

use itertools::Itertools;

use crate::{Day9, Solver};

sample!(
    Day9,
    "2199943210
3987894921
9856789892
8767896789
9899965678",
    "15"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pt {
    y: u32,
    x: u32,
}

impl Pt {
    fn new(x: u32, y: u32) -> Self {
        Self { y, x }
    }

    fn left(&self) -> Option<Pt> {
        if self.x > 0 {
            Some(Self::new(self.x - 1, self.y))
        } else {
            None
        }
    }

    fn right(&self, w: usize) -> Option<Pt> {
        if (self.x as usize) < w - 1 {
            Some(Self::new(self.x + 1, self.y))
        } else {
            None
        }
    }

    fn up(&self) -> Option<Pt> {
        if self.y > 0 {
            Some(Self::new(self.x, self.y - 1))
        } else {
            None
        }
    }

    fn down(&self, h: usize) -> Option<Pt> {
        if (self.y as usize) < h - 1 {
            Some(Self::new(self.x, self.y + 1))
        } else {
            None
        }
    }

    fn neigh(&self, w: usize, h: usize) -> impl Iterator<Item = Pt> {
        once(self.left())
            .chain(once(self.right(w)))
            .chain(once(self.up()))
            .chain(once(self.down(h)))
            .flatten()
    }
}
pub struct Grid(Vec<u32>, usize, usize);

impl Grid {
    fn new(rows: Vec<Vec<u32>>) -> Self {
        let wdith = rows.get(0).unwrap().len();
        let height = rows.len();
        log::debug!("w {}, h {}", wdith, height);
        Self(rows.into_iter().flatten().collect(), wdith, height)
    }

    fn width(&self) -> usize {
        self.1
    }

    fn height(&self) -> usize {
        self.2
    }

    fn depth(&self, pt: &Pt) -> Option<u32> {
        self.0
            .get(pt.y as usize * self.width() + pt.x as usize)
            .cloned()
    }

    fn pts(&self) -> impl Iterator<Item = Pt> {
        (0..self.height())
            .cartesian_product(0..self.width())
            .map(|(y, x)| Pt::new(x as u32, y as u32))
    }

    fn lows(&self) -> impl Iterator<Item = Pt> + '_ {
        self.pts().filter(|pt| {
            let this_depth = self
                .depth(pt)
                .unwrap_or_else(|| panic!("invalid pt {:?}", pt));
            pt.neigh(self.width(), self.height())
                .flat_map(|pt| self.depth(&pt))
                .all(|depth| depth > this_depth)
        })
    }
}

impl Solver for Day9 {
    type Output = u32;

    type Input = Grid;

    fn parse(input: &str) -> Self::Input {
        Grid::new(
            input
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| {
                    l.trim()
                        .chars()
                        .map(|c| {
                            c.to_string()
                                .parse::<u32>()
                                .unwrap_or_else(|_| panic!("invalid line {}", l))
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn part1(input: Self::Input) -> Self::Output {
        input
            .lows()
            .map(|low| input.depth(&low).unwrap() + 1)
            .sum::<u32>()
    }

    fn part2(input: Self::Input) -> Self::Output {
        todo!()
    }
}

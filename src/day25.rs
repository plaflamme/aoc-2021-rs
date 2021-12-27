use itertools::Itertools;

use crate::{grid::Dir, Day25, Solver};

sample!(
    Day25,
    "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>",
    "58"
);

type Pt = crate::grid::Pt<u8>;
type Grid = crate::grid::Grid<Slot>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cucumber {
    East,
    South,
}

impl Cucumber {
    fn from_char(c: char) -> Self {
        match c {
            '>' => Cucumber::East,
            'v' => Cucumber::South,
            _ => panic!("not a cucumber {}", c),
        }
    }
    fn dir(&self) -> Dir {
        match self {
            Cucumber::East => Dir::Right,
            Cucumber::South => Dir::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Empty,
    Occupied(Cucumber),
}
impl Slot {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            c => Self::Occupied(Cucumber::from_char(c)),
        }
    }
}

pub struct Ground(Grid);

impl Ground {
    fn cucumbers(&self, cucumber: Cucumber) -> impl Iterator<Item = Pt> + '_ {
        self.0.pts().filter(move |pt| match &self.0[*pt] {
            Slot::Empty => false,
            Slot::Occupied(c) => *c == cucumber,
        })
    }

    fn movable(&self, cucumber: Cucumber) -> impl Iterator<Item = (Pt, Pt)> + '_ {
        self.cucumbers(cucumber).filter_map(move |pt| {
            let w = self.0.width() as u8;
            let h = self.0.height() as u8;
            let destination = pt.to_wrapping(cucumber.dir(), w, h);
            if self.0[destination] == Slot::Empty {
                Some((pt, destination))
            } else {
                None
            }
        })
    }

    fn step_cucumber(&mut self, cucumber: Cucumber) -> usize {
        self.movable(cucumber)
            .collect_vec()
            .into_iter()
            .map(|(from, to)| {
                self.0[from] = Slot::Empty;
                self.0[to] = Slot::Occupied(cucumber);
            })
            .count()
    }

    fn step(&mut self) -> usize {
        self.step_cucumber(Cucumber::East) + self.step_cucumber(Cucumber::South)
    }
}

impl Solver for Day25 {
    type Output = usize;

    type Input = Ground;

    fn parse(input: &str) -> Self::Input {
        Ground(Grid::from_row_iter(
            input
                .lines()
                .map(|l| l.chars().map(Slot::from_char).collect()),
        ))
    }

    fn part1(mut input: Self::Input) -> Self::Output {
        1 + std::iter::repeat_with(|| input.step())
            .take_while(|s| *s > 0)
            .count()
    }

    fn part2(_input: Self::Input) -> Self::Output {
        todo!()
    }
}

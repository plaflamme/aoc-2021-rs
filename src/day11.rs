use std::collections::HashSet;

use itertools::Itertools;

use crate::{Day11, Solver};

sample!(
    Day11,
    "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
    "1656",
    "195"
);

type Pt = crate::grid::Pt<u8>;
type Grid = crate::grid::Grid<Octopus>;

pub struct Octopus {
    energy: u8,
}

impl Octopus {
    fn new(energy: u8) -> Self {
        Self { energy }
    }
}

fn step(grid: &mut Grid) -> HashSet<Pt> {
    let mut flashed = HashSet::new();
    let mut to_flash = HashSet::new();
    let mut to_bump = grid.pts::<u8>().collect_vec();

    loop {
        for pt in to_bump.drain(0..) {
            if flashed.contains(&pt) {
                continue;
            }
            let o = &mut grid[pt];
            o.energy += 1;
            if o.energy > 9 {
                to_flash.insert(pt);
            }
        }
        for pt in to_flash.drain() {
            if flashed.insert(pt) {
                grid[pt].energy = 0;
                let (w, h) = (grid.width() as u8, grid.height() as u8);
                let new = pt
                    .neighbours_checked(w, h)
                    .chain(pt.diagonals_checked(w, h));
                to_bump.extend(new)
            }
        }
        if to_bump.is_empty() {
            break flashed;
        }
    }
}

impl Solver for Day11 {
    type Output = usize;

    type Input = Grid;

    fn parse(input: &str) -> Self::Input {
        Grid::from_iter(
            10,
            input.lines().flat_map(|l| {
                l.chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .map(Octopus::new)
            }),
        )
    }

    fn part1(mut input: Self::Input) -> Self::Output {
        let mut count = 0;
        for _ in 0..100 {
            count += step(&mut input).len();
        }
        count
    }

    fn part2(mut input: Self::Input) -> Self::Output {
        let mut s = 0;
        loop {
            s += 1;
            if step(&mut input).len() == 100 {
                break;
            }
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Sample;
    #[test]
    fn test_steps() {
        let state = "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637";

        let mut grid = Day11::parse(Day11::CONTENT);
        step(&mut grid);
        let out = grid
            .rows_iter()
            .map(|row| row.iter().map(|o| o.energy.to_string()).join(""))
            .join("\n");

        assert_eq!(out, state);
        step(&mut grid);

        let state = "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848";

        let out = grid
            .rows_iter()
            .map(|row| row.iter().map(|o| o.energy.to_string()).join(""))
            .join("\n");

        assert_eq!(out, state);

        step(&mut grid);
        let state = "0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000";

        let out = grid
            .rows_iter()
            .map(|row| row.iter().map(|o| o.energy.to_string()).join(""))
            .join("\n");

        assert_eq!(out, state);
    }
}

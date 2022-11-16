use std::collections::HashSet;

use itertools::Itertools;

use aoc_lib::*;
day!(Day11, 11);

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
type Grid = crate::grid::Grid<u8>;

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
            *o += 1;
            if *o > 9 {
                to_flash.insert(pt);
            }
        }
        for pt in to_flash.drain() {
            if flashed.insert(pt) {
                grid[pt] = 0;
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
            input
                .lines()
                .flat_map(|l| l.chars().map(|c| c.to_string().parse::<u8>().unwrap())),
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
    use aoc_lib::Sample;
    #[test]
    fn test_steps() {
        let expected = vec![
            "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637",
            "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848",
            "0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000",
        ];

        let mut grid = Day11::parse(Day11::CONTENT);

        expected.into_iter().map(Day11::parse).for_each(|expected| {
            step(&mut grid);
            assert_eq!(
                grid.rows_iter().flatten().collect_vec(),
                expected.rows_iter().flatten().collect_vec()
            );
        });
    }
}

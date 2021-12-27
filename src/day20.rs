use std::{fmt::Display, ops::Index};

use itertools::Itertools;

use crate::{grid::Grid, Day20, Solver};

sample!(
    Day20,
    "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###",
    "35",
    "3351"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Light,
    Dark,
}

impl Pixel {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Pixel::Light,
            '.' => Pixel::Dark,
            _ => panic!("invalid char {}", c),
        }
    }
}

type Image = Grid<Pixel>;
type Pt = crate::grid::Pt<i32>;

impl Image {
    fn lit(&self) -> usize {
        self.rows_iter()
            .flatten()
            .filter(|p| **p == Pixel::Light)
            .count()
    }

    fn digits(&self, base: Pixel) -> impl Iterator<Item = [Pixel; 9]> + '_ {
        let w = self.width() as i32;
        let h = self.height() as i32;

        (-1..=h)
            .cartesian_product(-1..=(w))
            .map(|(y, x)| Pt::new(x, y))
            .map(move |pt| {
                let mut digit = [base; 9];
                std::iter::once(pt)
                    .chain(pt.neighbours())
                    .chain(pt.diagonals())
                    .for_each(|n| {
                        if let Some(pixel) = self.get(n) {
                            let y = (n.y - pt.y + 1) as usize;
                            let x = (n.x - pt.x + 1) as usize;
                            digit[y * 3 + x] = *pixel;
                        }
                    });
                digit
            })
    }
}

struct Algorithm(Vec<Pixel>);

impl Index<&[Pixel]> for Algorithm {
    type Output = Pixel;

    fn index(&self, index: &[Pixel]) -> &Self::Output {
        assert!(index.len() == 9);
        let mut alg_idx = 0_usize;
        index.iter().enumerate().for_each(|(idx, pixel)| {
            if *pixel == Pixel::Light {
                alg_idx |= 1 << (8 - idx);
            }
        });
        &self.0[alg_idx]
    }
}

pub struct Scan(Algorithm, Image);

fn enhance(alg: &Algorithm, i: Image, base: Pixel) -> Image {
    Image::from_iter(i.width() as usize + 2, i.digits(base).map(|d| alg[&d]))
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows_iter() {
            for s in row {
                let c = match s {
                    Pixel::Light => '#',
                    Pixel::Dark => '.',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Solver for Day20 {
    type Output = usize;

    type Input = Scan;

    fn parse(input: &str) -> Self::Input {
        let (alg, image) = crate::tools::empty_line_delimited_batches(input.lines())
            .tuples()
            .exactly_one()
            .ok()
            .unwrap();

        Scan(
            Algorithm(
                alg.into_iter()
                    .join("")
                    .chars()
                    .map(Pixel::from_char)
                    .collect(),
            ),
            Image::from_row_iter(
                image
                    .into_iter()
                    .map(|s| s.chars().map(Pixel::from_char).collect()),
            ),
        )
    }

    fn part1(input: Self::Input) -> Self::Output {
        let dark_digit = input.0[&[Pixel::Dark; 9]];
        enhance(
            &input.0,
            enhance(&input.0, input.1, Pixel::Dark),
            dbg!(dark_digit),
        )
        .lit()
    }

    fn part2(input: Self::Input) -> Self::Output {
        let mut img = input.1;
        let mut inf_pixel = Pixel::Dark;
        for _ in 0..50 {
            img = enhance(&input.0, img, inf_pixel);
            inf_pixel = input.0[&[inf_pixel; 9]];
        }
        img.lit()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Sample;

    #[test]
    fn test_index_algorithm() {
        let Scan(alg, _) = Day20::parse(Day20::CONTENT);

        let n = "...#...#.".chars().map(Pixel::from_char).collect_vec();

        assert_eq!(alg[&n], Pixel::Light);
    }

    #[test]
    fn test_step() {
        let Scan(alg, img) = Day20::parse(Day20::CONTENT);
        let expected = "#

.##.##.
#..#.#.
##.#..#
####..#
.#..##.
..##..#
...#.#.";
        let Scan(_, expected) = Day20::parse(expected);
        let img = enhance(&alg, img, Pixel::Dark);
        assert_eq!(
            img.rows_iter().collect_vec(),
            expected.rows_iter().collect_vec()
        );

        let expected = "#

.......#.
.#..#.#..
#.#...###
#...##.#.
#.....#.#
.#.#####.
..#.#####
...##.##.
....###..";

        let Scan(_, expected) = Day20::parse(expected);
        let img = enhance(&alg, img, Pixel::Dark);
        assert_eq!(
            img.rows_iter().collect_vec(),
            expected.rows_iter().collect_vec()
        );
    }
}

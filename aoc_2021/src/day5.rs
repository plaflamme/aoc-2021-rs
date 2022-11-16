use itertools::Itertools;

use aoc_lib::*;
day!(Day5, 5);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pt {
    x: i32,
    y: i32,
}

impl Pt {
    fn new(x: u32, y: u32) -> Self {
        Pt {
            x: x as i32,
            y: y as i32,
        }
    }

    // assumes 45 degree diagonals only
    fn approach(&self, other: &Pt) -> Pt {
        let dx = (other.x - self.x).signum();
        let dy = (other.y - self.y).signum();
        Pt {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line(Pt, Pt);

impl Line {
    fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    fn is_horiz(&self) -> bool {
        self.0.y == self.1.y
    }

    fn pts(&self) -> impl Iterator<Item = Pt> + '_ {
        itertools::iterate(self.0, |pt| pt.approach(&self.1))
            .take_while(|pt| *pt != self.1)
            .chain(std::iter::once(self.1))
    }
}

fn dangerous_pts<T>(lines: T) -> usize
where
    T: IntoIterator<Item = Line> + Clone,
{
    let (max_x, max_y) = lines
        .clone()
        .into_iter()
        .map(|l| (l.0.x.max(l.1.x), l.0.y.max(l.1.y)))
        .fold((0, 0), |(x, y), (xx, yy)| (x.max(xx), y.max(yy)));

    let mut freq_table = vec![0_u8; (max_x * (max_y + 1)) as usize];
    let mut count = 0;
    for line in lines {
        for pt in line.pts() {
            let freq = &mut freq_table[(pt.x + (pt.y * max_y)) as usize];
            match *freq {
                0 => *freq = 1,
                1 => {
                    // any pt with >= 2 intersecting lines is dangerous
                    *freq = 2;
                    count += 1;
                }
                _ => (),
            }
        }
    }
    count
}

sample!(
    Day5,
    "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
",
    "5",
    "12"
);

impl Solver for Day5 {
    type Output = usize;
    type Input = Vec<Line>;

    fn parse(input: &str) -> Self::Input
    where
        Self: Sized,
    {
        input
            .lines()
            .map(|l| {
                let (from, to) = l
                    .split_once(" -> ")
                    .unwrap_or_else(|| panic!("invalid line {}", l));
                fn pt(s: &str) -> Pt {
                    let (x, y) = s
                        .split_once(',')
                        .unwrap_or_else(|| panic!("invalid pt {}", s));
                    Pt::new(x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
                }
                Line(pt(from), pt(to))
            })
            .collect_vec()
    }

    fn part1(input: Self::Input) -> Self::Output {
        dangerous_pts(
            input
                .into_iter()
                .filter(|l| l.is_horiz() || l.is_vertical()),
        )
    }

    fn part2(input: Self::Input) -> Self::Output {
        dangerous_pts(input)
    }
}

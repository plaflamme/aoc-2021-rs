use itertools::Itertools;
use log::debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pt {
    x: u32,
    y: u32,
}

impl Pt {
    fn new(x: u32, y: u32) -> Self {
        Pt { x, y }
    }

    // assumes 45 degree diagonals only
    fn approach(&self, other: &Pt) -> Pt {
        let x = match self.x.cmp(&other.x) {
            std::cmp::Ordering::Less => self.x + 1,
            std::cmp::Ordering::Greater => self.x - 1,
            _ => self.x,
        };
        let y = match self.y.cmp(&other.y) {
            std::cmp::Ordering::Less => self.y + 1,
            std::cmp::Ordering::Greater => self.y - 1,
            _ => self.y,
        };
        Pt::new(x, y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line(Pt, Pt);

impl Line {
    fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    fn is_horiz(&self) -> bool {
        self.0.y == self.1.y
    }

    fn pts(&self) -> Vec<Pt> {
        let mut pt = self.0;
        let mut pts = Vec::new();
        while pt != self.1 {
            pts.push(pt);
            pt = pt.approach(&self.1);
        }
        pts.push(pt);
        pts
    }
}

fn dangerous_pts(lines: impl Iterator<Item = Line>) -> usize {
    lines
        .flat_map(|line| line.pts().into_iter())
        .sorted() // dedup_with_count dedups sequences, not the whole iterator, so we sort first
        .dedup_with_count()
        .inspect(|d| debug!("{:?}", d))
        .filter(|(count, _)| *count >= 2) // any pt with >= 2 intersecting lines is dangerous
        .count()
}

pub struct Solution(Vec<Line>);

impl super::Solver for Solution {
    const SAMPLE: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    const LEVEL1: &'static str = "5";

    const LEVEL2: &'static str = "12";

    type Output = usize;

    fn parse(input: &str) -> Self
    where
        Self: Sized,
    {
        let lines = input
            .lines()
            .map(|l| {
                let (from, to) = l
                    .split_once(" -> ")
                    .expect(format!("invalid line {}", l).as_str());
                fn pt(s: &str) -> Pt {
                    let (x, y) = s
                        .split_once(',')
                        .expect(format!("invalid pt {}", s).as_str());
                    Pt::new(x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
                }
                Line(pt(from), pt(to))
            })
            .inspect(|line| debug!("{:?} -> {:?}", line, line.pts()))
            .collect_vec();

        debug!("{:?}", lines);

        Solution(lines)
    }

    fn part1(self) -> Self::Output {
        dangerous_pts(
            self.0
                .into_iter()
                .filter(|l| l.is_horiz() || l.is_vertical()),
        )
    }

    fn part2(self) -> Self::Output {
        dangerous_pts(self.0.into_iter())
    }
}

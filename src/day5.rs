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

    fn down(&self) -> Pt {
        Pt::new(self.x, self.y + 1)
    }

    fn up(&self) -> Pt {
        Pt::new(self.x, self.y - 1)
    }

    fn left(&self) -> Pt {
        Pt::new(self.x - 1, self.y)
    }

    fn right(&self) -> Pt {
        Pt::new(self.x + 1, self.y)
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

    fn y_ext(&self) -> (u32, u32) {
        (self.0.y.min(self.1.y), self.0.y.max(self.1.y))
    }
    fn x_ext(&self) -> (u32, u32) {
        (self.0.x.min(self.1.x), self.0.x.max(self.1.x))
    }

    fn pts(&self) -> Vec<Pt> {
        if self.is_vertical() {
            let (min, max) = self.y_ext();
            (min..=max)
                .into_iter()
                .map(|y| Pt::new(self.0.x, y))
                .collect_vec()
        } else {
            let (min, max) = self.x_ext();
            (min..=max)
                .into_iter()
                .map(|x| Pt::new(x, self.0.y))
                .collect_vec()
        }
    }
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

    const LEVEL2: &'static str = "???";

    type Output = String;

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
        self.0
            .into_iter()
            .filter(|l| l.is_horiz() || l.is_vertical())
            .flat_map(|line| line.pts().into_iter())
            .sorted()
            .dedup_with_count()
            .inspect(|d| debug!("{:?}", d))
            .filter(|(count, pt)| *count >= 2)
            .count()
            .to_string()
    }

    fn part2(self) -> Self::Output {
        todo!()
    }
}

use std::{fmt::Debug, ops::RangeInclusive};

use itertools::Itertools;

use crate::{Day22, Solver};

sample!(
    Day22,
    "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
    "474140",
    "2758514936282235"
);

#[derive(Debug, Clone)]
pub struct Instr(
    bool,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    from: i32,
    to: i32, // exclusive
}

impl Range {
    fn new(from: i32, to: i32) -> Self {
        assert!(from <= to, "{} is not <= {}", from, to);
        Self { from, to }
    }

    fn is_empty(&self) -> bool {
        self.from == self.to
    }

    fn contains(&self, idx: i32) -> bool {
        self.from <= idx && self.to > idx
    }

    fn intersects(&self, rhs: &Range) -> bool {
        self.contains(rhs.from)
            || self.contains(rhs.to)
            || rhs.contains(self.from)
            || rhs.contains(self.to)
    }

    fn split(&self, rhs: &Range) -> (Option<Range>, Range, Option<Range>) {
        assert!(self.intersects(&rhs) || rhs.is_empty());

        if self.from >= rhs.from && self.to <= rhs.to {
            // lhs:  |-|
            // rhs: |---|
            (None, self.clone(), None)
        } else if self.from <= rhs.from {
            if self.to <= rhs.to {
                // lhs: |---|
                // rhs:   |---|
                (
                    Some(Range::new(self.from, rhs.from)).filter(|r| !r.is_empty()),
                    Range::new(rhs.from, self.to),
                    None,
                )
            } else if self.to > rhs.to {
                // lhs: |---|
                // rhs:  |-|
                (
                    Some(Range::new(self.from, rhs.from)).filter(|r| !r.is_empty()),
                    Range::new(rhs.from, rhs.to),
                    Some(Range::new(rhs.to, self.to)),
                )
            } else {
                unreachable!()
            }
        } else {
            if self.to >= rhs.to {
                // lhs:   |---|
                // rhs: |---|
                (
                    None,
                    Range::new(self.from, rhs.to),
                    Some(Range::new(rhs.to, self.to)).filter(|r| !r.is_empty()),
                )
            } else {
                unreachable!()
            }
        }
    }
}

impl From<std::ops::Range<i32>> for Range {
    fn from(r: std::ops::Range<i32>) -> Self {
        Range {
            from: r.start,
            to: r.end,
        }
    }
}

impl From<RangeInclusive<i32>> for Range {
    fn from(r: RangeInclusive<i32>) -> Self {
        let (from, to) = r.into_inner();
        Range { from, to: to + 1 }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Entry {
    Bulb(bool),
    Dim(Box<RangeTree>),
}

impl Entry {
    fn merge(&mut self, rhs: Self) {
        match (self, rhs) {
            (s @ Entry::Bulb(_), rhs @ Entry::Bulb(_)) => *s = rhs,
            (Entry::Dim(box ref mut lhs), Entry::Dim(box rhs)) => lhs.merge(rhs),
            _ => panic!("cannot merge different entries"),
        }
    }
    fn count(&self) -> usize {
        match self {
            Entry::Bulb(true) => 1,
            Entry::Bulb(false) => 0,
            Entry::Dim(box tree) => tree.count(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct RangeTree {
    range: Range,
    value: Entry,
    left: Option<Box<RangeTree>>,
    right: Option<Box<RangeTree>>,
}

impl RangeTree {
    fn new<I>(range: I, value: Entry) -> Self
    where
        I: Into<Range>,
    {
        Self {
            range: range.into(),
            value,
            left: None,
            right: None,
        }
    }

    fn split(&mut self, range: &Range) -> (Option<RangeTree>, Range, Option<RangeTree>) {
        let (left, this, right) = self.range.split(range);

        let left = if let Some(left) = left {
            Some(RangeTree {
                range: left,
                value: self.value.clone(),
                left: self.left.take(),
                right: None,
            })
        } else {
            self.left.take().map(|b| *b)
        };
        let right = if let Some(right) = right {
            Some(RangeTree {
                range: right,
                value: self.value.clone(),
                left: None,
                right: self.right.take(),
            })
        } else {
            self.right.take().map(|b| *b)
        };

        (left, this, right)
    }

    fn merge(&mut self, mut rhs: RangeTree) {
        if self.range.intersects(&rhs.range) {
            let (left, range, right) = self.split(&rhs.range);
            let (rhs_left, rhs_range, rhs_right) = rhs.split(&range);

            assert_eq!(range, rhs_range);

            let left = match (left, rhs_left) {
                (Some(mut l), Some(r)) => {
                    l.merge(r);
                    Some(l)
                }
                (None, r @ Some(_)) => r,
                (l @ Some(_), None) => l,
                (None, None) => None,
            };
            let right = match (right, rhs_right) {
                (Some(mut l), Some(r)) => {
                    l.merge(r);
                    Some(l)
                }
                (None, r @ Some(_)) => r,
                (l @ Some(_), None) => l,
                (None, None) => None,
            };

            // TODO: there's probably a way to avoid this clone
            let mut value = self.value.clone();
            value.merge(rhs.value);

            *self = RangeTree {
                range,
                value,
                left: left.map(|t| Box::new(t)),
                right: right.map(|t| Box::new(t)),
            };
        } else if self.range.from < rhs.range.from {
            match self.right.as_mut() {
                Some(box tree) => tree.merge(rhs),
                None => self.right = Some(Box::new(rhs)), // TODO: we could avoid adding empty branches here
            }
        } else {
            match self.left.as_mut() {
                Some(box tree) => tree.merge(rhs),
                None => self.left = Some(Box::new(rhs)), // TODO: we could avoid adding empty branches here
            }
        }
    }

    fn count(&self) -> usize {
        let size = (self.range.to - self.range.from).abs() as usize;
        self.value.count() * size
            + self.left.as_ref().map(|t| t.count()).unwrap_or(0)
            + self.right.as_ref().map(|t| t.count()).unwrap_or(0)
    }
}

impl From<Instr> for RangeTree {
    fn from(instr: Instr) -> Self {
        RangeTree::new(
            instr.1,
            Entry::Dim(Box::new(RangeTree::new(
                instr.2,
                Entry::Dim(Box::new(RangeTree::new(instr.3, Entry::Bulb(instr.0)))),
            ))),
        )
    }
}

fn solve(instrs: impl Iterator<Item = Instr>) -> RangeTree {
    instrs
        .map_into::<RangeTree>()
        .fold1(|mut l, r| {
            l.merge(r);
            l
        })
        .unwrap()
}

impl Solver for Day22 {
    type Output = usize;

    type Input = Vec<Instr>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let (on_off, ranges) = l.split_once(" ").unwrap();
                let turn_on = match on_off {
                    "on" => true,
                    "off" => false,
                    _ => panic!("unexpected instruction {}", on_off),
                };

                fn parse_range(s: &str) -> RangeInclusive<i32> {
                    let (_, s) = s.split_once("=").unwrap();
                    let (from, to) = s.split_once("..").unwrap();
                    let from = from.parse::<i32>().unwrap();
                    let to = to.parse::<i32>().unwrap();
                    from..=to
                }
                let (x, y, z) = ranges
                    .split_terminator(',')
                    .tuples()
                    .exactly_one()
                    .ok()
                    .unwrap();

                Instr(turn_on, parse_range(x), parse_range(y), parse_range(z))
            })
            .collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        fn included(r: &RangeInclusive<i32>) -> bool {
            let (from, to) = r.clone().into_inner();
            (-50..=50).contains(&from) && (-50..=50).contains(&to)
        }

        solve(
            input
                .into_iter()
                .filter(|r| included(&r.1) && included(&r.2) && included(&r.3)),
        )
        .count()
    }

    fn part2(input: Self::Input) -> Self::Output {
        solve(input.into_iter()).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
        let input = Day22::parse(input);
        let tree = solve(input.clone().into_iter().map_into().take(1));
        assert_eq!(tree.count(), 27);

        let tree = solve(input.clone().into_iter().map_into().skip(1).take(1));
        assert_eq!(tree.count(), 27);

        let tree = solve(input.clone().into_iter().map_into().take(2));
        assert_eq!(tree.count(), 27 + 19);

        let tree = solve(input.clone().into_iter().map_into().skip(2).take(1));
        assert_eq!(tree.count(), 0);

        let tree = solve(input.clone().into_iter().map_into().take(3));
        assert_eq!(tree.count(), 27 + 19 - 8);

        let tree = solve(input.clone().into_iter().map_into());
        assert_eq!(tree.count(), 39);
    }

    macro_rules! bulbs {
        ($range: expr, $lit: literal) => {
            RangeTree::new($range, Entry::Bulb($lit))
        };
        ($x: expr, $y: expr, $lit: literal) => {
            RangeTree::new($x, Entry::Dim(Box::new(bulbs!($y, $lit))))
        };
        ($x: expr, $y: expr, $z: expr, $lit: literal) => {
            RangeTree::new($x, Entry::Dim(Box::new(bulbs!($y, $z, $lit))))
        };
    }

    #[test]
    fn test_range_tree_split() {
        let mut tree = bulbs!(0..10, true);
        let (left, range, right) = tree.split(&(4..6).into());
        assert_eq!(range, (4..6).into());
        assert_eq!(left, Some(RangeTree::new(0..4, Entry::Bulb(true))));
        assert_eq!(right, Some(RangeTree::new(6..10, Entry::Bulb(true))));

        tree.merge(left.unwrap());
        tree.merge(right.unwrap());

        assert_eq!(tree.count(), 10);
    }

    #[test]
    fn test_range_tree_split_2d() {
        let inner = Entry::Dim(Box::new(RangeTree::new(0..10, Entry::Bulb(true))));
        let mut tree = bulbs!(0..10, 0..10, true);
        let (left, range, right) = tree.split(&(4..6).into());
        assert_eq!(range, (4..6).into());
        assert_eq!(left, Some(RangeTree::new(0..4, inner.clone())));
        assert_eq!(right, Some(RangeTree::new(6..10, inner.clone())));

        tree.merge(left.unwrap());
        tree.merge(right.unwrap());

        assert_eq!(tree.count(), 100);
    }

    #[test]
    fn test_range_tree_cut() {
        let mut tree = bulbs!(0..10, 0..10, true);
        assert_eq!(tree.count(), 100);

        tree.merge(bulbs!(9..11, 9..11, false));
        assert_eq!(tree.count(), 99);
    }

    #[test]
    fn test_range_tree_hole() {
        let mut tree = bulbs!(0..10, true);
        tree.merge(bulbs!(5..6, false));
        assert_eq!(tree.count(), 9);

        let mut tree = bulbs!(0..10, 0..10, true);
        let hole = bulbs!(5..6, 5..6, false);
        tree.merge(hole);
        assert_eq!(tree.count(), 99);

        let mut tree = bulbs!(0..10, 0..10, 0..10, true);
        let hole = bulbs!(5..6, 5..6, 5..6, false);
        tree.merge(hole);
        assert_eq!(tree.count(), 999);
    }

    #[test]
    fn test_range_tree_count() {
        let mut tree = bulbs!(0..4, 0..4, 0..4, true);
        assert_eq!(tree.count(), 64);
        tree.merge(bulbs!(2..6, 2..6, 2..6, true));
        assert_eq!(tree.count(), 64 + 64 - 8);
        tree.merge(bulbs!(2..4, 2..4, 2..4, false));
        assert_eq!(tree.count(), 64 + 64 - 8 - 8);
    }
}

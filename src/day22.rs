use std::ops::RangeInclusive;

use itertools::Itertools;
use ndarray::{s, Array3};

use crate::{Day22, Solver};

sample!(
    Day22,
    "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682",
    "590784"
);

pub struct Instr(
    bool,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
);

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
        let mut bools: Array3<bool> = Array3::default((101, 101, 101));

        fn shift_in_range(r: RangeInclusive<i32>) -> Option<RangeInclusive<usize>> {
            let (start, end) = r.into_inner();
            if (-50..=50).contains(&start) && (-50..=50).contains(&end) {
                let start = (start + 50) as usize;
                let end = (end + 50) as usize;
                Some(start..=end)
            } else {
                None
            }
        }

        for Instr(on, x, y, z) in input {
            if let Some(x) = shift_in_range(x) {
                if let Some(y) = shift_in_range(y) {
                    if let Some(z) = shift_in_range(z) {
                        let slice = s![x, y, z];
                        bools.slice_mut(slice).fill(on);
                    }
                }
            }
        }

        bools.into_iter().filter(|b| *b).count()
    }

    fn part2(input: Self::Input) -> Self::Output {
        todo!()
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
        assert_eq!(Day22::part1(input), 39);
    }
}

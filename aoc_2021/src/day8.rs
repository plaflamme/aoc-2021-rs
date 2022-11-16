use aoc_lib::*;
day!(Day8, 8);

use bitflags::bitflags;
use itertools::Itertools;

sample!(
    Day8,
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    "26",
    "61229"
);

//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg

bitflags! {
    struct Signal: u32 {
        const A = 0b0000001;
        const B = 0b0000010;
        const C = 0b0000100;
        const D = 0b0001000;
        const E = 0b0010000;
        const F = 0b0100000;
        const G = 0b1000000;

        const _0 = Self::A.bits | Self::B.bits | Self::C.bits | Self::E.bits | Self::F.bits | Self::G.bits;
        const _1 = Self::C.bits | Self::F.bits;
        const _2 = Self::A.bits | Self::C.bits | Self::D.bits | Self::E.bits | Self::G.bits;
        const _3 = Self::A.bits | Self::C.bits | Self::D.bits | Self::F.bits | Self::G.bits;
        const _4 = Self::B.bits | Self::C.bits | Self::D.bits | Self::F.bits;
        const _5 = Self::A.bits | Self::B.bits | Self::D.bits | Self::F.bits | Self::G.bits;
        const _6 = Self::A.bits | Self::B.bits | Self::D.bits | Self::E.bits | Self::F.bits | Self::G.bits;
        const _7 = Self::A.bits | Self::C.bits | Self::F.bits;
        const _8 = Self::A.bits | Self::B.bits | Self::C.bits | Self::D.bits | Self::E.bits | Self::F.bits | Self::G.bits;
        const _9 = Self::A.bits | Self::B.bits | Self::C.bits | Self::D.bits | Self::F.bits | Self::G.bits;
    }
}

impl Signal {
    fn from_str(s: &str) -> Self {
        s.chars()
            .map(Signal::from_char)
            .fold(Signal::empty(), |s, c| s | c)
    }

    fn from_char(ch: char) -> Self {
        match ch {
            'a' => Signal::A,
            'b' => Signal::B,
            'c' => Signal::C,
            'd' => Signal::D,
            'e' => Signal::E,
            'f' => Signal::F,
            'g' => Signal::G,
            _ => panic!("invalid segment"),
        }
    }

    fn n_bits(&self) -> u32 {
        self.bits().count_ones()
    }
}

#[derive(Debug, Clone)]
struct Panel(Vec<Signal>);

impl Panel {
    fn signals_by_num(&self, num: u32) -> impl Iterator<Item = Signal> + '_ {
        match num {
            0 => self.signals_with_nbits(Signal::_0),
            1 => self.signals_with_nbits(Signal::_1),
            2 => self.signals_with_nbits(Signal::_2),
            3 => self.signals_with_nbits(Signal::_3),
            4 => self.signals_with_nbits(Signal::_4),
            5 => self.signals_with_nbits(Signal::_5),
            6 => self.signals_with_nbits(Signal::_6),
            7 => self.signals_with_nbits(Signal::_7),
            8 => self.signals_with_nbits(Signal::_8),
            9 => self.signals_with_nbits(Signal::_9),
            _ => panic!("invalid number {}", num),
        }
    }

    // find signals that have the same number of bits (segments)
    fn signals_with_nbits(&self, other: Signal) -> impl Iterator<Item = Signal> + '_ {
        self.0
            .iter()
            .cloned()
            .filter(move |s| other.n_bits() == s.n_bits())
    }

    // filters the signals that could match the specified number
    //   the predicate is expected to match a single value which is returned
    fn filter_from_num(&self, num: u32, f: impl Fn(&Signal) -> bool) -> Signal {
        self.signals_by_num(num)
            .filter(f)
            .exactly_one()
            .unwrap_or_else(|_| panic!("cannot find {}", num))
    }
}

pub struct Reading(Panel, Vec<Signal>);

#[derive(Debug)]
struct Decoder([Signal; 10]);

impl Decoder {
    fn new(panel: Panel) -> Self {
        // extract the unique numbers
        let n1 = panel.filter_from_num(1, |_| true);
        let n4 = panel.filter_from_num(4, |_| true);
        let n7 = panel.filter_from_num(7, |_| true);
        let n8 = panel.filter_from_num(8, |_| true);

        // deal with the [6,9,0] group

        // adding 1 to 6 is the only one that gives 8 in this group
        // 6 | 1 == 8
        let n6 = panel.filter_from_num(6, |s| *s | n1 == n8);

        // similarly, 7 | 4 provides a unique pattern to identify 9 in the group
        // 7 | 4 = 9 & (7 | 4)
        let n7_4 = n7 | n4;
        let n9 = panel.filter_from_num(9, |s| (*s & n7_4) == n7_4);

        // 0 is now !n9 && !n6
        let n0 = panel.filter_from_num(0, |s| *s != n9 && *s != n6);

        // deal with the [2,3,5] group
        let a = n7 - n1;
        let d = n8 - n0;
        let g = n9 - n7_4;

        // build 3 from previous answers
        // 3 = a | d | g | 1
        let n3 = a | d | g | n1;

        // 2 | 4 is the only one in the group that results in 8
        // 2 | 4 == 8
        let n2 = panel.filter_from_num(2, |s| *s | n4 == n8);

        // 5 is now !n2 && !n3
        let n5 = panel.filter_from_num(3, |s| *s != n2 && *s != n3);

        log::debug!("n0 = {:?}", n0);
        log::debug!("n1 = {:?}", n1);
        log::debug!("n2 = {:?}", n2);
        log::debug!("n3 = {:?}", n3);
        log::debug!("n4 = {:?}", n4);
        log::debug!("n5 = {:?}", n5);
        log::debug!("n6 = {:?}", n6);
        log::debug!("n7 = {:?}", n7);
        log::debug!("n8 = {:?}", n8);
        log::debug!("n9 = {:?}", n9);

        Self([n0, n1, n2, n3, n4, n5, n6, n7, n8, n9])
    }

    fn decode(&self, input: Signal) -> u8 {
        self.0.iter().position(|s| *s == input).unwrap() as u8
    }
}

impl Solver for Day8 {
    type Output = u32;
    type Input = Vec<Reading>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let (s, out) = l.split_once(" | ").unwrap();

                let signals = s.split_ascii_whitespace().map(Signal::from_str).collect();

                let outputs = out.split_ascii_whitespace().map(Signal::from_str).collect();

                Reading(Panel(signals), outputs)
            })
            .collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        let uniques = [2, 3, 4, 7];
        input
            .into_iter()
            .map(|reading| {
                reading
                    .1
                    .into_iter()
                    .map(|s| s.n_bits())
                    .filter(|n| uniques.contains(n))
                    .count() as u32
            })
            .sum::<u32>()
    }

    fn part2(input: Self::Input) -> Self::Output {
        input
            .into_iter()
            .map(|reading| {
                let decoder = Decoder::new(reading.0);
                reading
                    .1
                    .into_iter()
                    .map(|s| decoder.decode(s))
                    .join("")
                    .parse::<u32>()
                    .unwrap()
            })
            .sum::<u32>()
    }
}

#[cfg(test)]
mod test {
    use super::{Decoder, Panel, Signal};

    #[test]
    fn test_decoder_program() {
        let s = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
        let signals = s.split_ascii_whitespace().map(Signal::from_str).collect();

        let decoder = Decoder::new(Panel(signals));
        assert_eq!(decoder.decode(Signal::from_str("cdfeb")), 5);
        assert_eq!(decoder.decode(Signal::from_str("fcadb")), 3);
        assert_eq!(decoder.decode(Signal::from_str("cdfeb")), 5);
        assert_eq!(decoder.decode(Signal::from_str("cdbaf")), 3);
    }
}

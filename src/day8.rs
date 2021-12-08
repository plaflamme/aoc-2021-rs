use crate::{Day8, Solver};

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
    "26"
);

pub struct Test(Vec<String>, Vec<String>);

const NUM_BY_SIGNAL: [Option<u8>; 8] = [None, None, Some(1), Some(7), Some(4), None, None, Some(8)];

impl Solver for Day8 {
    type Output = u32;
    type Input = Vec<Test>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let (s, out) = l.split_once(" | ").unwrap();
                Test(
                    s.split_ascii_whitespace().map(|s| s.to_string()).collect(),
                    out.split_ascii_whitespace()
                        .map(|s| s.to_string())
                        .collect(),
                )
            })
            .collect()
    }

    fn part1(mut input: Self::Input) -> Self::Output {
        input
            .into_iter()
            .map(|test| {
                let count = test
                    .1
                    .clone()
                    .into_iter()
                    .flat_map(|s| NUM_BY_SIGNAL[s.len() as usize])
                    .count() as u32;
                log::debug!("{:?} -> {}", test.1, count);
                count
            })
            .sum::<u32>()
    }

    fn part2(mut input: Self::Input) -> Self::Output {
        todo!()
    }
}

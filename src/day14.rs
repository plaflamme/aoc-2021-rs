use std::{collections::HashMap, convert::identity};

use itertools::Itertools;

use crate::{Day14, Solver};

sample!(
    Day14,
    "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
    "1588",
    "2188189693529"
);

type Element = char;
type Pair = [Element; 2];

pub struct Rule(Pair, [Pair; 2]);

#[derive(Debug)]
struct Template {
    frequencies: HashMap<Pair, usize>,
    last: Element,
}

impl Template {
    fn new(els: Vec<Element>) -> Self {
        let last = *els.last().unwrap();
        Template {
            frequencies: els
                .into_iter()
                .tuple_windows()
                .map(|(first, second)| [first, second])
                .counts_by(identity),
            last,
        }
    }

    fn step(&mut self, rules: &HashMap<Pair, [Pair; 2]>) {
        let mut new_frequencies = self.frequencies.clone();

        rules.iter().for_each(|(pair, [a, b])| {
            if let Some(freq) = self.frequencies.get(pair) {
                new_frequencies.entry(*pair).and_modify(|f| *f -= freq);
                new_frequencies
                    .entry(*a)
                    .and_modify(|f| *f += freq)
                    .or_insert(*freq);
                new_frequencies
                    .entry(*b)
                    .and_modify(|f| *f += freq)
                    .or_insert(*freq);
            }
        });
        self.frequencies = new_frequencies;
    }

    fn solve(&self) -> usize {
        let mut el_freqs = self
            .frequencies
            .iter()
            .map(|([el, _], freq)| (el, *freq))
            .into_grouping_map()
            .sum();

        el_freqs.entry(&self.last).and_modify(|f| *f += 1);

        if let itertools::MinMaxResult::MinMax(min, max) = el_freqs.values().minmax() {
            *max - *min
        } else {
            unreachable!("oops");
        }
    }
}

impl Solver for Day14 {
    type Output = usize;

    type Input = (Vec<Element>, HashMap<Pair, [Pair; 2]>);

    fn parse(input: &str) -> Self::Input {
        let (template, rules) = crate::tools::empty_line_delimited_batches(input.lines())
            .tuples()
            .exactly_one()
            .ok()
            .unwrap();

        let template = template
            .into_iter()
            .exactly_one()
            .ok()
            .unwrap()
            .chars()
            .collect();

        let rules = rules
            .into_iter()
            .map(|l| {
                let (tpl, el) = l.split_once(" -> ").unwrap();

                let (a, b) = tpl.chars().tuples().exactly_one().ok().unwrap();
                let c = el.chars().exactly_one().ok().unwrap();

                ([a, b], [[a, c], [c, b]])
            })
            .collect();

        (template, rules)
    }

    fn part1(input: Self::Input) -> Self::Output {
        let mut tpl = Template::new(input.0);
        for _ in 0..10 {
            log::debug!("{:?}", tpl);
            tpl.step(&input.1);
        }
        log::debug!("{:?}", tpl);
        tpl.solve()
    }

    fn part2(input: Self::Input) -> Self::Output {
        let mut tpl = Template::new(input.0);
        for _ in 0..40 {
            tpl.step(&input.1);
        }
        tpl.solve()
    }
}

#[cfg(test)]
mod test {
    use crate::Sample;

    use super::*;

    #[test]
    fn test_sample() {
        let (els, rules) = Day14::parse(Day14::CONTENT);
        let mut tpl = Template::new(els);

        tpl.step(&rules);
        let sample = Template::new("NCNBCHB".chars().collect());
        assert_eq!(tpl.frequencies, sample.frequencies);

        tpl.step(&rules);
        let sample = Template::new("NBCCNBBBCBHCB".chars().collect());
        assert_eq!(tpl.frequencies, sample.frequencies);

        tpl.step(&rules);
        let sample = Template::new("NBBBCNCCNBBNBNBBCHBHHBCHB".chars().collect());
        assert_eq!(tpl.frequencies, sample.frequencies);

        tpl.step(&rules);
        let sample = Template::new(
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
                .chars()
                .collect(),
        );
        assert_eq!(tpl.frequencies, sample.frequencies);
    }
}

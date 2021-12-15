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

pub struct Rule([Element; 2], Element);

#[derive(Debug)]
struct Template {
    frequencies: HashMap<[Element; 2], usize>,
    first_last: [Element; 2],
}

impl Template {
    fn new(els: Vec<Element>) -> Self {
        let first_last = [*els.first().unwrap(), *els.last().unwrap()];
        Template {
            frequencies: els
                .into_iter()
                .tuple_windows()
                .map(|(first, second)| [first, second])
                .counts_by(identity),
            first_last,
        }
    }

    fn step(&mut self, rules: &HashMap<[Element; 2], Element>) {
        let mut new_frequencies = self.frequencies.clone();

        rules.into_iter().for_each(|rule| {
            let pair = rule.0;
            let insert = rule.1;
            if let Some(freq) = self.frequencies.get(pair) {
                let current = new_frequencies.get_mut(rule.0).unwrap();
                if *current == *freq {
                    new_frequencies.remove(pair);
                } else {
                    *current -= freq;
                }

                *new_frequencies.entry([pair[0], *insert]).or_insert(0) += freq;
                *new_frequencies.entry([*insert, pair[1]]).or_insert(0) += freq;
            }
        });
        self.frequencies = new_frequencies;
    }

    fn solve(&self) -> usize {
        let mut el_freqs = HashMap::new();
        self.frequencies
            .iter()
            .filter(|(_, f)| **f > 0)
            .flat_map(|(pair, freq)| vec![(pair[0], freq), (pair[1], freq)])
            .for_each(|(el, freq)| *el_freqs.entry(el).or_insert(0_usize) += freq);

        el_freqs.iter_mut().for_each(|(el, freq)| {
            if self.first_last.contains(el) {
                *freq += 1;
            }
            *freq /= 2;
        });

        log::debug!("{:?}", el_freqs);

        let el_freqs = el_freqs.values().cloned().sorted().collect_vec();

        el_freqs.last().unwrap() - el_freqs.first().unwrap()
    }
}

impl Solver for Day14 {
    type Output = usize;

    type Input = (Vec<Element>, HashMap<[Element; 2], Element>);

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

                ([a, b], c)
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

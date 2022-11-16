use std::convert::identity;

use itertools::Itertools;
use log::debug;

use aoc_lib::*;
day!(Day6, 6);

#[derive(Clone)]
pub struct Colony([usize; 9], usize);

impl Colony {
    fn step(&mut self) {
        let gen_0 = self.0[0];
        for gen in 1..9 {
            let gen_freq = self.0[gen];
            self.0[gen - 1] = gen_freq;
        }
        self.0[6] += gen_0;
        self.0[8] = gen_0;
        self.1 += 1;
    }

    fn simulate(&mut self, to_gen: usize) -> usize {
        for _ in self.1..to_gen {
            self.step();
        }
        self.size()
    }

    fn size(&self) -> usize {
        self.0.iter().sum::<usize>()
    }
}

sample!(Day6, "3,4,3,1,2", "5934", "26984457539");

impl Solver for Day6 {
    type Output = usize;
    type Input = Colony;

    fn parse(input: &str) -> Self::Input
    where
        Self: Sized,
    {
        let counts = input
            .trim()
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .counts_by(identity);
        let mut freq_table = [0_usize; 9];
        for (age, count) in counts {
            freq_table[age as usize] = count;
        }
        debug!("freq: {:?}", freq_table);
        Colony(freq_table, 0)
    }

    fn part1(mut input: Self::Input) -> Self::Output {
        input.simulate(80)
    }

    fn part2(mut input: Self::Input) -> Self::Output {
        input.simulate(256)
    }
}

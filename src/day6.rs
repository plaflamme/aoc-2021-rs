use std::convert::identity;

use itertools::Itertools;
use log::debug;

#[derive(Clone)]
struct Colony([usize; 9], usize);

impl Colony {
    fn step(&mut self) -> usize {
        let gen_0 = self.0[0];
        for gen in 1..9 {
            let gen_freq = self.0[gen];
            self.0[gen - 1] = gen_freq;
        }
        self.0[6] += gen_0;
        self.0[8] = gen_0;
        self.1 += 1;
        self.1
    }

    fn size(&self) -> usize {
        self.0.iter().sum::<usize>()
    }
}

pub struct Solution(Colony);

impl super::Solver for Solution {
    const DAY: u8 = 6;
    const SAMPLE: &'static str = "3,4,3,1,2";
    const LEVEL1: &'static str = "5934";
    const LEVEL2: &'static str = "???";

    type Output = usize;

    fn parse(input: &str) -> Self
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
        Solution(Colony(freq_table, 0))
    }

    fn part1(mut self) -> Self::Output {
        let mut day = self.0.step();
        while day != 80 {
            debug!("freq: {:?}", self.0 .0);
            day = self.0.step();
        }
        self.0.size()
    }

    fn part2(self) -> Self::Output {
        todo!()
    }
}

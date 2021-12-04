#[derive(Clone)]
pub struct Solution(Vec<u32>, usize);

enum Criteria {
    Oxygen,
    Co2,
}

impl Solution {
    fn bit_freq(&self, bit: usize) -> (u32, u32) {
        let mut zeros = 0;
        let mut ones = 0;
        let mask = 1 << (self.1 - 1 - bit);
        for n in self.0.clone() {
            if n & mask == 0 {
                zeros += 1
            } else {
                ones += 1
            }
        }
        (zeros, ones)
    }

    fn filter_by_bit_criteria(&mut self, bit: usize, criteria: Criteria) {
        let (zeros, ones) = self.bit_freq(bit);
        let mask = 1 << (self.1 - 1 - bit);
        match criteria {
            Criteria::Oxygen => {
                // Keep the numbers that have the MOST COMMON bit or the ones that have `1` when they have the same frequency
                // AKA: keep only the numbers that have 0 in the bit position if it's more frequent.
                if zeros > ones {
                    self.0.retain(|n| n & mask == 0);
                } else {
                    self.0.retain(|n| n & mask > 0);
                }
            }
            Criteria::Co2 => {
                // Keep the numbers that have the LEAST COMMON bit or the ones that have `0` when they have the same frequency
                // AKA: keep only the numbers that have 0 in the bit position if its frequency is less or equal to that of the 1s.
                if zeros <= ones {
                    self.0.retain(|n| n & mask == 0);
                } else {
                    self.0.retain(|n| n & mask > 0);
                }
            }
        }
    }
}

impl super::Solver for Solution {
    const SAMPLE: &'static str = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    const LEVEL1: &'static str = "198";

    const LEVEL2: &'static str = "230";

    type Output = u32;

    fn parse(input: &str) -> Self
    where
        Self: Sized,
    {
        let bits = input
            .lines()
            .filter(|l| l.len() > 0)
            .map(|line| u32::from_str_radix(line, 2).unwrap())
            .collect();
        let width = input.lines().filter(|l| l.len() > 0).next().unwrap().len();
        Solution(bits, width)
    }

    fn part1(self) -> Self::Output {
        let mut zeros = vec![0 as u32; self.1];
        let mut ones = vec![0 as u32; self.1];
        for bit in 0..self.1 {
            let (zero_freq, one_freq) = self.bit_freq(bit);
            zeros[bit] = zero_freq;
            ones[bit] = one_freq;
        }
        let mut gamma = 0;
        let mut epsilon = 0;
        for bit in 0..self.1 {
            let mask = 1 << (self.1 - 1 - bit);
            if ones[bit] > zeros[bit] {
                gamma |= mask;
            } else {
                epsilon |= mask;
            }
        }

        epsilon * gamma
    }

    fn part2(self) -> Self::Output {
        let mut oxygen = self.clone();
        for bit in 0..self.1 {
            oxygen.filter_by_bit_criteria(bit, Criteria::Oxygen);
            if oxygen.0.len() == 1 {
                break;
            }
        }
        let mut co2 = self.clone();
        for bit in 0..self.1 {
            co2.filter_by_bit_criteria(bit, Criteria::Co2);
            if co2.0.len() == 1 {
                break;
            }
        }

        assert!(oxygen.0.len() == 1);
        assert!(co2.0.len() == 1);

        oxygen.0.get(0).unwrap() * co2.0.get(0).unwrap()
    }
}

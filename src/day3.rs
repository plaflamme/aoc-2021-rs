pub struct Solution(Vec<u32>, usize);

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
}

impl super::Day for Solution {
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

    fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let bits = input
            .lines()
            .filter(|l| l.len() > 0)
            .map(|line| u32::from_str_radix(line, 2).unwrap())
            .collect();
        let width = input.lines().filter(|l| l.len() > 0).next().unwrap().len();
        Ok(Solution(bits, width))
    }

    fn level1(self) -> Result<Self::Output, Box<dyn std::error::Error>> {
        let mut zeros = vec![0 as u32; self.1];
        let mut ones = vec![0 as u32; self.1];
        for bit in 0..self.1 {
            let (zero_freq, one_freq) = self.bit_freq(bit);
            zeros[bit] = zero_freq;
            ones[bit] = one_freq;
        }
        for n in self.0 {
            for bit in 0..self.1 {
                let mask = 1 << (self.1 - 1 - bit);
                if n & mask == 0 {
                    zeros[bit] += 1
                } else {
                    ones[bit] += 1
                }
            }
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

        Ok(epsilon * gamma)
    }

    fn level2(self) -> Result<Self::Output, Box<dyn std::error::Error>> {
        todo!()
    }
}

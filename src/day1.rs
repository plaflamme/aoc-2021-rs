use itertools::izip;
use std::str::FromStr;

pub struct Solution(Vec<u32>);

fn increases(depths: Vec<u32>) -> usize {
    depths
        .clone()
        .into_iter()
        .skip(1)
        .zip(depths.into_iter())
        .filter(|(next, previous)| next > previous)
        .count()
}

impl super::Day for Solution {
    const SAMPLE: &'static str = "199
200
208
210
200
207
240
269
260
263";

    const LEVEL1: &'static str = "7";

    const LEVEL2: &'static str = "5";

    type Output = usize;

    fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(
            input
                .lines()
                .map(|l| u32::from_str(l.trim()).unwrap())
                .collect(),
        ))
    }

    fn level1(self) -> Result<Self::Output, Box<dyn std::error::Error>> {
        Ok(increases(self.0))
    }

    fn level2(self) -> Result<Self::Output, Box<dyn std::error::Error>> {
        let grouped = izip![
            self.0.clone(),
            self.0.clone().into_iter().skip(1),
            self.0.clone().into_iter().skip(2)
        ];

        Ok(increases(grouped.map(|(a, b, c)| a + b + c).collect()))
    }
}

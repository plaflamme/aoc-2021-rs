use itertools::izip;
use std::str::FromStr;

pub struct Solution;

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

    type Input = Vec<u32>;

    type Output = usize;

    fn parse(input: &str) -> Result<Self::Input, Box<dyn std::error::Error>> {
        Ok(input
            .lines()
            .map(|l| u32::from_str(l.trim()).unwrap())
            .collect())
    }

    fn level1(input: Self::Input) -> Result<Self::Output, Box<dyn std::error::Error>> {
        Ok(increases(input))
    }

    fn level2(input: Self::Input) -> Result<Self::Output, Box<dyn std::error::Error>> {
        let grouped = izip![
            input.clone(),
            input.clone().into_iter().skip(1),
            input.clone().into_iter().skip(2)
        ];

        Ok(increases(grouped.map(|(a, b, c)| a + b + c).collect()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

    #[test]
    fn test_level1() {
        assert_eq!("7", level1(SAMPLE).to_string());
    }

    #[test]
    fn test_level2() {
        assert_eq!("5", level2(SAMPLE).to_string());
    }
}

use aocf::{Aoc, Level};
use std::error::Error;

mod day1 {
    use std::str::FromStr;

    fn parse(input: &str) -> Vec<u32> {
        input
            .lines()
            .map(|l| u32::from_str(l.trim()).unwrap())
            .collect()
    }

    fn increases(depths: Vec<u32>) -> usize {
        depths
            .clone()
            .into_iter()
            .skip(1)
            .zip(depths.into_iter())
            .filter(|(next, previous)| next > previous)
            .count()
    }

    pub fn level1(input: &str) -> impl ToString {
        increases(parse(input))
    }

    pub fn level2(input: &str) -> impl ToString {
        let depths = parse(input);

        let third = depths.clone().into_iter().skip(2);
        let second = depths.clone().into_iter().skip(1);

        let grouped = depths
            .into_iter()
            .zip(second)
            .zip(third)
            .map(|((first, second), third)| first + second + third)
            .collect();

        increases(grouped)
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
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut aoc = Aoc::new().year(Some(2021)).day(Some(1)).init().unwrap();
    let input = aoc.get_input(false)?;
    match aoc.level {
        Level::First => aoc.submit(&day1::level1(&input.clone()).to_string())?,
        Level::Second => aoc.submit(&day1::level2(&input.clone()).to_string())?,
    };
    Ok(())
}

use itertools::Itertools;

pub struct Solution(Vec<u32>);

fn fuel_cost(max: u32) -> Vec<u32> {
    (0..=max)
        .into_iter()
        .scan(0_u32, |cost, distance| {
            *cost += distance as u32;
            Some(*cost)
        })
        .collect()
}

fn solve(crabs: Vec<u32>, fuel_cost: impl Fn(u32) -> u32) -> u32 {
    crabs
        .iter()
        .counts_by(std::convert::identity)
        .into_iter()
        .sorted_by(|(_, freq1), (_, freq2)| freq1.cmp(freq2).reverse())
        .map(|(pos, _)| *pos as usize)
        // compute the fuel cost for each position, remembering the minimum we've seen
        //   starting from the most frequent position, since that's the most likely minimum cost
        //   short-circuiting when the fuel cost sum goes above the known minimum one
        .fold(u32::MAX, |current_min, pos| {
            let mut total_fuel_cost = 0;
            for c in crabs.iter() {
                total_fuel_cost += fuel_cost((*c as i32 - pos as i32).abs() as u32);
                if total_fuel_cost >= current_min {
                    return current_min;
                }
            }
            total_fuel_cost
        })
}

impl super::Solver for Solution {
    const DAY: u8 = 7;

    const SAMPLE: &'static str = "16,1,2,0,4,2,7,1,2,14";

    const LEVEL1: &'static str = "37";

    const LEVEL2: &'static str = "168";

    type Output = u32;

    fn parse(input: &str) -> Self
    where
        Self: Sized,
    {
        Solution(
            input
                .trim()
                .split(',')
                .filter(|l| !l.is_empty())
                .map(|l| l.parse::<u32>().unwrap())
                .collect(),
        )
    }

    fn part1(self) -> Self::Output {
        solve(self.0, std::convert::identity)
    }

    fn part2(self) -> Self::Output {
        let max = self.0.clone().into_iter().max().unwrap() as usize;
        let fuel_cost = fuel_cost(max as u32);
        solve(self.0, |dist| fuel_cost[dist as usize])
    }
}

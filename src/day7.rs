use itertools::Itertools;

use crate::Day7;

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

sample!(Day7, "16,1,2,0,4,2,7,1,2,14", "37", "168");

impl super::Solver for Day7 {
    type Output = u32;
    type Input = Vec<u32>;

    fn parse(input: &str) -> Self::Input
    where
        Self: Sized,
    {
        input
            .trim()
            .split(',')
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<u32>().unwrap())
            .collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        solve(input, std::convert::identity)
    }

    fn part2(input: Self::Input) -> Self::Output {
        let max = input.clone().into_iter().max().unwrap() as usize;
        let fuel_cost = fuel_cost(max as u32);
        solve(input, |dist| fuel_cost[dist as usize])
    }
}

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
    let max = crabs.clone().into_iter().max().unwrap() as usize;
    let mut fuel_use = vec![0_u32; max + 1];
    crabs.into_iter().for_each(|c| {
        for (pos, cost) in fuel_use.iter_mut().enumerate() {
            *cost += fuel_cost((c as i32 - pos as i32).abs() as u32)
        }
    });
    fuel_use.into_iter().min().unwrap()
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

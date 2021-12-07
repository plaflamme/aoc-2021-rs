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
        let max = self.0.clone().into_iter().max().unwrap() as usize;
        let mut fuel_use = vec![0_u32; max];
        self.0.into_iter().for_each(|c| {
            for pos in 0..max {
                fuel_use[pos] += (c as i32 - pos as i32).abs() as u32
            }
        });
        fuel_use.into_iter().min().unwrap()
    }

    fn part2(self) -> Self::Output {
        let max = self.0.clone().into_iter().max().unwrap() as usize;
        let cost = fuel_cost(max as u32);
        let mut fuel_use = vec![0_u32; max];
        self.0.into_iter().for_each(|c| {
            for pos in 0..max {
                fuel_use[pos] += cost[(c as i32 - pos as i32).abs() as usize]
            }
        });
        fuel_use.into_iter().min().unwrap()
    }
}

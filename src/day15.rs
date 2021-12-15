use itertools::Itertools;
use num::integer::div_rem;

use crate::{grid::Pt, Day15, Solver};

sample!(
    Day15,
    "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
    "40",
    "315"
);

type Cavern = crate::grid::Grid<u8>;

fn lowest_cost(cavern: Cavern) -> usize {
    let w = cavern.width() as u32;
    let h = cavern.height() as u32;
    let end = Pt(w - 1, h - 1);
    let path = pathfinding::directed::astar::astar(
        &Pt(0_u32, 0),
        |pt| {
            pt.neighbours_checked(w, h)
                .map(|pt| (pt, cavern[pt] as usize))
                .collect_vec()
        },
        |pt| pt.manhattan_unsigned(&end) as usize, // this must be <= actual cost so distance works
        |pt| *pt == end,
    );
    path.unwrap().1
}

fn extend(cavern: Cavern) -> Cavern {
    let width = cavern.width();
    let height = cavern.height();

    let mut extended = Cavern::from_iter(width * 5, vec![0; width * height * 25].into_iter());

    extended.pts::<u32>().for_each(|pt| {
        // div_rem returns both the quotient and remainder in a single operation
        let (inc_x, orig_x) = div_rem(pt.x, width as u32);
        let (inc_y, orig_y) = div_rem(pt.y, height as u32);
        let orig_value = cavern[Pt(orig_x, orig_y)] as u32;
        let mut new_value = orig_value + inc_x + inc_y;
        if new_value > 9 {
            new_value -= 9
        }
        extended[pt] = new_value as u8;
    });
    extended
}

impl Solver for Day15 {
    type Output = usize;

    type Input = Cavern;

    fn parse(input: &str) -> Self::Input {
        Cavern::from_row_iter(input.lines().map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        }))
    }

    fn part1(input: Self::Input) -> Self::Output {
        lowest_cost(input)
    }

    fn part2(input: Self::Input) -> Self::Output {
        lowest_cost(extend(input))
    }
}

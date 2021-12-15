use itertools::Itertools;

use crate::{Day15, Solver};

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
type Pt = crate::grid::Pt<u32>;

fn lowest_cost(cavern: Cavern) -> usize {
    let end = cavern.pts().sorted().last().unwrap();
    let w = cavern.width() as u32;
    let h = cavern.height() as u32;
    let path = pathfinding::directed::astar::astar(
        &crate::grid::Pt(0_u32, 0),
        |pt| {
            pt.neighbours_checked(w, h)
                .map(|pt| (pt, cavern[pt] as usize))
                .collect_vec()
        },
        |_| 0,
        |pt| *pt == end,
    );
    path.unwrap().1
}

fn extend(cavern: Cavern) -> Cavern {
    let width = cavern.width();
    let height = cavern.height();

    let mut extended = Cavern::from_iter(width * 5, vec![0; width * height * 25].into_iter());

    extended.pts::<u32>().for_each(|pt| {
        let orig_x = pt.x % width as u32;
        let orig_y = pt.y % height as u32;

        let inc_x = pt.x / width as u32;
        let inc_y = pt.y / height as u32;
        let orig_value = cavern[crate::grid::Pt(orig_x, orig_y)] as u32;
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

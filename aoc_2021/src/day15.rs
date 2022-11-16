use itertools::Itertools;
use num::integer::div_rem;
use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_lib::*;
day!(Day15, 15);

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

fn pathfinding_astar(cavern: Cavern) -> usize {
    let w = cavern.width() as u32;
    let h = cavern.height() as u32;
    let end = Pt::new(w - 1, h - 1);
    let path = pathfinding::directed::astar::astar(
        &Pt::new(0, 0),
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

fn pathfinding_dijkstra(cavern: Cavern) -> usize {
    let w = cavern.width() as u32;
    let h = cavern.height() as u32;
    let end = Pt::new(w - 1, h - 1);
    let path = pathfinding::directed::dijkstra::dijkstra(
        &Pt::new(0, 0),
        |pt| {
            pt.neighbours_checked(w, h)
                .map(|pt| (pt, cavern[pt] as usize))
                .collect_vec()
        },
        |pt| *pt == end,
    );
    path.unwrap().1
}

// Priority queue "nodes".
// Keeps track of the cost from `start` to `Pt`
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct State(Reverse<usize>, Pt);

fn dijkstra<N>(
    start: Pt,
    target: Pt,
    (width, height): (usize, usize),
    mut neighbours: N,
) -> Option<usize>
where
    N: FnMut(&Pt) -> Vec<(Pt, usize)>,
{
    // keeps track of the lowest cost we've seen from start to every other node
    let mut lowest_costs = crate::grid::Grid::from_iter(width, vec![usize::MAX; width * height]);

    // a priority queue of (node, current_cost) to visit
    let mut to_visit = BinaryHeap::new();
    to_visit.push(State(Reverse(0), start));

    while let Some(State(Reverse(cost), pos)) = to_visit.pop() {
        if pos == target {
            return Some(cost);
        }

        // perhaps we've seen a better path already
        if cost > lowest_costs[pos] {
            continue;
        }

        for (n, move_cost) in neighbours(&pos) {
            let new_cost = cost + move_cost;
            if new_cost < lowest_costs[n] {
                lowest_costs[n] = new_cost;
                to_visit.push(State(Reverse(new_cost), n));
            }
        }
    }
    None
}

fn manual_dijkstra(cavern: Cavern) -> usize {
    let width = cavern.width();
    let height = cavern.height();
    dijkstra(
        Pt::new(0, 0),
        Pt::new(width as u32 - 1, height as u32 - 1),
        (width, height),
        |pt| {
            pt.neighbours_checked(width as u32, height as u32)
                .map(|pt| (pt, cavern[pt] as usize))
                .collect()
        },
    )
    .unwrap()
}

fn extend(cavern: Cavern) -> Cavern {
    let width = cavern.width();
    let height = cavern.height();

    let mut extended = Cavern::from_iter(width * 5, vec![0; width * height * 25].into_iter());

    extended.pts::<u32>().for_each(|pt| {
        // div_rem returns both the quotient and remainder in a single operation
        let (inc_x, orig_x) = div_rem(pt.x, width as u32);
        let (inc_y, orig_y) = div_rem(pt.y, height as u32);
        let orig_value = cavern[Pt::new(orig_x, orig_y)] as u32;
        let mut ext_value = orig_value + inc_x + inc_y;
        if ext_value > 9 {
            ext_value -= 9
        }
        extended[pt] = ext_value as u8;
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
        manual_dijkstra(input)
    }

    fn part2(input: Self::Input) -> Self::Output {
        manual_dijkstra(extend(input))
    }
}

#[derive(Debug)]
pub struct AStar;
impl Solver<AStar> for Day15 {
    type Output = usize;

    type Input = Cavern;

    fn parse(input: &str) -> Self::Input {
        <Day15 as Solver>::parse(input)
    }

    fn part1(input: Self::Input) -> Self::Output {
        pathfinding_astar(input)
    }

    fn part2(input: Self::Input) -> Self::Output {
        pathfinding_astar(extend(input))
    }
}

#[derive(Debug)]
pub struct Dijkstra;
impl Solver<Dijkstra> for Day15 {
    type Output = usize;

    type Input = Cavern;

    fn parse(input: &str) -> Self::Input {
        <Day15 as Solver>::parse(input)
    }

    fn part1(input: Self::Input) -> Self::Output {
        pathfinding_dijkstra(input)
    }

    fn part2(input: Self::Input) -> Self::Output {
        pathfinding_dijkstra(extend(input))
    }
}

use itertools::Itertools;

use crate::grid::Pt;
use crate::{Day17, Solver};

sample!(Day17, "target area: x=20..30, y=-10..-5", "45", "112");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Velocity(i32, i32);
#[derive(Debug)]
struct Probe {
    pos: Pt,
    vel: Velocity,
}

impl Probe {
    fn step(&mut self) {
        self.pos.x += self.vel.0;
        self.pos.y += self.vel.1;

        self.vel.0 -= self.vel.0.signum();
        self.vel.1 -= 1;
    }
}
#[derive(Debug)]
pub struct Area(Pt, Pt);

impl Area {
    fn contains(&self, pt: &Pt) -> bool {
        (self.0.x..=self.1.x).contains(&pt.x) && (self.0.y..=self.1.y).contains(&pt.y)
    }
}

fn is_solution(init: Velocity, area: &Area) -> Option<i32> {
    if let Velocity(0, 0) = init {
        return None;
    }
    let mut probe = Probe {
        pos: Pt::new(0, 0),
        vel: init,
    };

    let mut height = 0;
    loop {
        if area.contains(&probe.pos) {
            break Some(height);
        } else if probe.pos.y < area.0.y || probe.pos.x > area.1.x {
            break None;
        }
        probe.step();
        height = height.max(probe.pos.y);
    }
}

impl Solver for Day17 {
    type Output = usize;

    type Input = Area;

    fn parse(input: &str) -> Self::Input {
        let (_, input) = input.split_once(':').unwrap();
        let (x, y) = input.trim().split_once(", ").unwrap();
        let (_, x) = x.split_once('=').unwrap();
        let (_, y) = y.split_once('=').unwrap();
        let (xmin, xmax) = x.split_once("..").unwrap();
        let (ymin, ymax) = y.split_once("..").unwrap();

        Area(
            Pt::new(xmin.parse().unwrap(), ymin.parse().unwrap()),
            Pt::new(xmax.parse().unwrap(), ymax.parse().unwrap()),
        )
    }

    fn part1(area: Self::Input) -> Self::Output {
        // whatever vertical velocity we start with (Vi), when the probe comes back down,
        //   it will have velocity -(Vi + 1) at y=0
        // So the maximum vertical velocity we can get is the distance from 0 to the lower part of the area - 1.

        // this only works if the area is below the 0 line
        assert!(area.0.y < 0);

        let dist = area.0.y.abs();
        let vi = dist - 1;
        let max_x = area.1.x + 1;
        (0..max_x)
            .flat_map(|x| is_solution(Velocity(x, vi), &area))
            .max()
            .unwrap() as usize
    }

    fn part2(area: Self::Input) -> Self::Output {
        // we know this is the maximum velocity we can start with
        let max_y = area.0.y.abs();
        // at this velocity, after a single step, we've overshot the target
        let max_x = area.1.x + 1;

        (0..max_x)
            .cartesian_product(-max_y..=max_y)
            .filter(|(x, y)| is_solution(Velocity(*x, *y), &area).is_some())
            .unique()
            .count()
    }
}

#[cfg(test)]
mod test {
    use crate::Sample;

    use super::*;
    #[test]
    fn test_sample() {
        let area = Day17::parse(<Day17 as Sample>::CONTENT);

        assert_eq!(is_solution(Velocity(7, 2), &area), Some(3));
        assert_eq!(is_solution(Velocity(6, 3), &area), Some(6));
        assert_eq!(is_solution(Velocity(9, 0), &area), Some(0));
        assert_eq!(is_solution(Velocity(6, 9), &area), Some(45));
        assert_eq!(is_solution(Velocity(17, -4), &area), None);
        assert_eq!(is_solution(Velocity(0, 10), &area), None);
        assert_eq!(is_solution(Velocity(100, 0), &area), None);

        assert_eq!(Day17::part1(area), 45);
    }
}

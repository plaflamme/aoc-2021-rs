use std::ops::{Index, IndexMut};

use itertools::Itertools;

use crate::{Day23, Solver};

type Pt = crate::grid::Pt<u8>;

sample!(
    Day23,
    "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
    "12521"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Amphipod::A,
            'B' => Amphipod::B,
            'C' => Amphipod::C,
            'D' => Amphipod::D,
            c => panic!("not an amphipod: {}", c),
        }
    }
    fn cost(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Slot {
    Empty,
    Occupied(Amphipod),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Room([Slot; 2]);

impl Room {
    fn empty() -> Self {
        Self([Slot::Empty, Slot::Empty])
    }
}

impl Index<usize> for Room {
    type Output = Slot;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Room {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cave {
    hallway: [Slot; 11],
    rooms: [Room; 4],
}

impl Cave {
    fn new(rooms: [Room; 4]) -> Self {
        Self {
            hallway: [Slot::Empty; 11],
            rooms,
        }
    }

    fn pts() -> impl Iterator<Item = Pt> {
        (0..11).map(|x| Pt::new(x, 0)).chain(
            [2, 4, 6, 8]
                .into_iter()
                .flat_map(|x| [Pt::new(x, 1), Pt::new(x, 2)]),
        )
    }

    fn all_landing_pts() -> impl Iterator<Item = Pt> {
        Cave::pts().filter(|pt| {
            if pt.y == 0 {
                pt.x < 2 || pt.x > 8 || pt.x % 2 == 1
            } else {
                true
            }
        })
    }

    fn can_travel(&self, from: &Pt, to: &Pt, a: Amphipod) -> bool {
        // to the hallway or within the same room is always legal
        if to.y == 0 || from.x == to.x {
            true
        } else {
            // otherwise, check that the destination room is empty or has the same kind of Amphipod
            let other = if to.y == 1 {
                Pt::new(to.x, 2)
            } else {
                Pt::new(to.x, 1)
            };
            let other = self[other];
            self[to] == Slot::Empty && (other == Slot::Empty || other == Slot::Occupied(a))
        }
    }

    fn available_landing_pts<'a>(&'a self, from: Pt, a: Amphipod) -> impl Iterator<Item = Pt> + 'a {
        Cave::all_landing_pts()
            .filter(move |pt| *pt != from)
            .filter(|pt| self[pt] == Slot::Empty)
            .filter(move |pt| self.can_travel(&from, pt, a))
    }

    fn slots(&self) -> impl Iterator<Item = (Pt, &Slot)> + '_ {
        Cave::pts().map(|pt| (pt, &self[pt]))
    }

    fn amphipods(&self) -> impl Iterator<Item = (Pt, Amphipod)> + '_ {
        self.slots()
            .filter_map(|(pt, slot)| {
                if let Slot::Occupied(a) = slot {
                    Some((pt, *a))
                } else {
                    None
                }
            })
            .sorted_by(|(_, a), (_, b)| a.cmp(b)) // prioritize the least costly
    }

    fn distance(&self, from: &Pt, to: &Pt) -> Option<usize> {
        let mut current = from.clone();
        let dx = (to.x as i8 - from.x as i8).signum();
        let mut distance = 0;
        while current != *to && (current == *from || self[current] == Slot::Empty) {
            distance += 1;
            if current.y == 0 && current.x != to.x {
                current.x = (current.x as i8 + dx) as u8;
            } else if current.x != to.x {
                current.y -= 1
            } else {
                let dy = (to.y as i8 - current.y as i8).signum();
                current.y = (current.y as i8 + dy) as u8;
            }
        }
        if current == *to {
            Some(distance)
        } else {
            None
        }
    }

    fn neighbour_caves(&self) -> impl Iterator<Item = (Cave, usize)> + '_ {
        self.amphipods()
            .flat_map(|(from, a)| {
                self.available_landing_pts(from, a)
                    .map(move |to| (from, to, a))
            })
            .filter_map(|(from, to, a)| {
                self.distance(&from, &to)
                    .map(|d| (from, to, a, d * a.cost()))
            })
            .map(|(from, to, a, cost)| {
                let mut cave = self.clone();
                cave[from] = Slot::Empty;
                cave[to] = Slot::Occupied(a);
                (cave, cost)
            })
    }

    fn is_solved(&self) -> bool {
        self.rooms
            == [
                Room([Slot::Occupied(Amphipod::A); 2]),
                Room([Slot::Occupied(Amphipod::B); 2]),
                Room([Slot::Occupied(Amphipod::C); 2]),
                Room([Slot::Occupied(Amphipod::D); 2]),
            ]
    }
}

impl Index<&Pt> for Cave {
    type Output = Slot;

    fn index(&self, index: &Pt) -> &Self::Output {
        if index.y == 0 {
            &self.hallway[index.x as usize]
        } else {
            // 2 => 0
            // 4 => 1
            // 6 => 2
            // 8 => 3
            let room_idx = index.x / 2 - 1;
            &self.rooms[room_idx as usize][index.y as usize - 1]
        }
    }
}

impl IndexMut<&Pt> for Cave {
    fn index_mut(&mut self, index: &Pt) -> &mut Self::Output {
        if index.y == 0 {
            &mut self.hallway[index.x as usize]
        } else {
            // 2 => 0
            // 4 => 1
            // 6 => 2
            // 8 => 3
            let room_idx = index.x / 2 - 1;
            &mut self.rooms[room_idx as usize][index.y as usize - 1]
        }
    }
}

impl Index<Pt> for Cave {
    type Output = Slot;

    fn index(&self, index: Pt) -> &Self::Output {
        &self[&index]
    }
}

impl IndexMut<Pt> for Cave {
    fn index_mut(&mut self, index: Pt) -> &mut Self::Output {
        &mut self[&index]
    }
}

impl Solver for Day23 {
    type Output = usize;

    type Input = [Room; 4];

    fn parse(input: &str) -> Self::Input {
        let mut rooms = [Room::empty(); 4];
        input
            .lines()
            .skip(2)
            .take(2)
            .enumerate()
            .for_each(|(row, l)| {
                l[3..=10]
                    .split_terminator('#')
                    .take(4)
                    .map(|s| Amphipod::from_char(s.chars().nth(0).unwrap()))
                    .enumerate()
                    .for_each(|(idx, a)| rooms[idx][row] = Slot::Occupied(a))
            });
        rooms
    }

    fn part1(input: Self::Input) -> Self::Output {
        pathfinding::directed::dijkstra::dijkstra(
            &Cave::new(input),
            |cave| cave.neighbour_caves().collect_vec(), // damn
            |cave| cave.is_solved(),
        )
        .unwrap()
        .1
    }

    fn part2(input: Self::Input) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::Sample;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Day23::parse(Day23::CONTENT),
            [
                Room([Slot::Occupied(Amphipod::B), Slot::Occupied(Amphipod::A)]),
                Room([Slot::Occupied(Amphipod::C), Slot::Occupied(Amphipod::D)]),
                Room([Slot::Occupied(Amphipod::B), Slot::Occupied(Amphipod::C)]),
                Room([Slot::Occupied(Amphipod::D), Slot::Occupied(Amphipod::A)])
            ],
        );
    }
    #[test]
    fn test_one() {
        let rooms = Day23::parse(Day23::CONTENT);
        let cave = Cave::new(rooms);
        dbg!(cave.neighbour_caves().collect_vec());
    }
}

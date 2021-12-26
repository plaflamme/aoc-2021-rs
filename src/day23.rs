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
    "12521",
    "44169"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Amphipod {
    A = 2,
    B = 4,
    C = 6,
    D = 8,
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
pub struct Room<const DEPTH: usize>([Slot; DEPTH]);

impl<const D: usize> Room<D> {
    fn empty() -> Self {
        Self([Slot::Empty; D])
    }
}

impl<const D: usize> Index<usize> for Room<D> {
    type Output = Slot;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const D: usize> IndexMut<usize> for Room<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cave<const D: usize> {
    hallway: [Slot; 11], // A better solution would have been to use actual stacks.
    rooms: [Room<D>; 4],
}

impl<const D: usize> Cave<D> {
    const SOLUTION: [Room<D>; 4] = [
        Room([Slot::Occupied(Amphipod::A); D]),
        Room([Slot::Occupied(Amphipod::B); D]),
        Room([Slot::Occupied(Amphipod::C); D]),
        Room([Slot::Occupied(Amphipod::D); D]),
    ];
    const HALLWAY: [Pt; 7] = [
        Pt::new(0, 0),
        Pt::new(1, 0),
        Pt::new(3, 0),
        Pt::new(5, 0),
        Pt::new(7, 0),
        Pt::new(9, 0),
        Pt::new(10, 0),
    ];

    fn new(rooms: [Room<D>; 4]) -> Self {
        Self {
            hallway: [Slot::Empty; 11],
            rooms,
        }
    }

    fn room_pts(a: Amphipod) -> impl Iterator<Item = Pt> {
        (1..=D).map(move |d| Pt::new(a as u8, d as u8))
    }

    // returns the deepest point in an Amphipod room if an amphipod can enter it
    fn bottom_room_stack(&self, a: Amphipod) -> Option<Pt> {
        let mut room_pts = Cave::<D>::room_pts(a);
        let first = room_pts.next().unwrap();
        if let Slot::Empty = self[first] {
            room_pts.fold(Some(first), |top, b| {
                if let Some(_) = top {
                    let other = self[b];
                    match other {
                        Slot::Empty => Some(b),
                        Slot::Occupied(o) if o == a => top,
                        Slot::Occupied(_) => None,
                    }
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    // an iterator over the valid pts an amphipod can move to
    fn available_landing_pts<'a>(&'a self, from: Pt, a: Amphipod) -> impl Iterator<Item = Pt> + 'a {
        Cave::<D>::HALLWAY
            .into_iter()
            .filter(move |_| from.y != 0) // an amphipod will stay in the same spot in the hallway once it's there
            .filter(|pt| self[pt] == Slot::Empty)
            .chain(self.bottom_room_stack(a).into_iter())
            .filter(move |pt| *pt != from)
    }

    // an iterator over the first Amphipod in each room
    fn top_room_stacks(&self) -> impl Iterator<Item = (Pt, Amphipod)> + '_ {
        [Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D]
            .into_iter()
            .flat_map(|a| {
                Cave::<D>::room_pts(a).find_map(|pt| {
                    if let Slot::Occupied(a) = self[pt] {
                        Some((pt, a))
                    } else {
                        None
                    }
                })
            })
    }

    // an iterator over all the Amphipods that could move
    fn movable_amphipods(&self) -> impl Iterator<Item = (Pt, Amphipod)> + '_ {
        Cave::<D>::HALLWAY
            .into_iter()
            .filter_map(|pt| {
                if let Slot::Occupied(a) = self[pt] {
                    Some((pt, a))
                } else {
                    None
                }
            })
            .chain(self.top_room_stacks())
    }

    // computes the distance between 2 points, if all slots in between are empty
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

    // an iterator over all caves that are one move away from self (with the cost of making that move)
    fn neighbour_caves(&self) -> impl Iterator<Item = (Cave<D>, usize)> + '_ {
        self.movable_amphipods()
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
        self.rooms == Cave::<D>::SOLUTION
    }
}

impl<const D: usize> Index<&Pt> for Cave<D> {
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

impl<const D: usize> IndexMut<&Pt> for Cave<D> {
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

impl<const D: usize> Index<Pt> for Cave<D> {
    type Output = Slot;

    fn index(&self, index: Pt) -> &Self::Output {
        &self[&index]
    }
}

impl<const D: usize> IndexMut<Pt> for Cave<D> {
    fn index_mut(&mut self, index: Pt) -> &mut Self::Output {
        &mut self[&index]
    }
}

fn solve<const D: usize>(input: [Room<D>; 4]) -> usize {
    let (_, cost) = pathfinding::directed::dijkstra::dijkstra(
        &Cave::<D>::new(input),
        |cave| cave.neighbour_caves().collect_vec(), // damn
        |cave| cave.is_solved(),
    )
    .unwrap();

    cost
}

fn expand(input: [Room<2>; 4]) -> [Room<4>; 4] {
    let mut expanded = [
        Room([Slot::Empty; 4]),
        Room([Slot::Empty; 4]),
        Room([Slot::Empty; 4]),
        Room([Slot::Empty; 4]),
    ];
    let expand_with = [
        Room([Slot::Occupied(Amphipod::D); 2]),
        Room([Slot::Occupied(Amphipod::C), Slot::Occupied(Amphipod::B)]),
        Room([Slot::Occupied(Amphipod::B), Slot::Occupied(Amphipod::A)]),
        Room([Slot::Occupied(Amphipod::A), Slot::Occupied(Amphipod::C)]),
    ];
    for r in 0..4 {
        expanded[r][0] = input[r][0];
        expanded[r][1] = expand_with[r][0];
        expanded[r][2] = expand_with[r][1];
        expanded[r][3] = input[r][1];
    }
    expanded
}

impl Solver for Day23 {
    type Output = usize;

    type Input = [Room<2>; 4];

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
        solve(input)
    }

    fn part2(input: Self::Input) -> Self::Output {
        solve(expand(input))
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
    fn test_expand() {
        assert_eq!(
            expand(Day23::parse(Day23::CONTENT)),
            [
                Room([
                    Slot::Occupied(Amphipod::B),
                    Slot::Occupied(Amphipod::D),
                    Slot::Occupied(Amphipod::D),
                    Slot::Occupied(Amphipod::A)
                ]),
                Room([
                    Slot::Occupied(Amphipod::C),
                    Slot::Occupied(Amphipod::C),
                    Slot::Occupied(Amphipod::B),
                    Slot::Occupied(Amphipod::D)
                ]),
                Room([
                    Slot::Occupied(Amphipod::B),
                    Slot::Occupied(Amphipod::B),
                    Slot::Occupied(Amphipod::A),
                    Slot::Occupied(Amphipod::C)
                ]),
                Room([
                    Slot::Occupied(Amphipod::D),
                    Slot::Occupied(Amphipod::A),
                    Slot::Occupied(Amphipod::C),
                    Slot::Occupied(Amphipod::A)
                ])
            ],
        );
    }
}

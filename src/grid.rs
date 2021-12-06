use std::ops::{Index, IndexMut};

use itertools::Itertools;
use num::{FromPrimitive, Integer, ToPrimitive};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn iter() -> impl Iterator<Item = Dir> {
        [Dir::Up, Dir::Down, Dir::Left, Dir::Right].into_iter()
    }

    // Up => Left, Down => Right, Left => Down, Right => Up
    pub fn rot_left(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    // Up => Right, Down => Left, Left => Up, Right => Down
    pub fn rot_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    // Up <=> Left, Down <=> Right
    pub fn ul_dr(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    // Up <=> Right, Down <=> Left
    pub fn ur_dl(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord<N = i32> {
    pub y: N, // comes before x for Ord
    pub x: N,
}

#[allow(non_snake_case)]
pub const fn Coord<N>(x: N, y: N) -> Coord<N> {
    Coord { x, y }
}

impl<N: Integer + Copy> Coord<N> {
    pub fn up(self) -> Self {
        Coord(self.x, self.y - N::one())
    }
    pub fn down(self) -> Self {
        Coord(self.x, self.y + N::one())
    }
    pub fn left(self) -> Self {
        Coord(self.x - N::one(), self.y)
    }
    pub fn right(self) -> Self {
        Coord(self.x + N::one(), self.y)
    }
    pub fn to(self, d: Dir) -> Self {
        match d {
            Dir::Up => self.up(),
            Dir::Down => self.down(),
            Dir::Left => self.left(),
            Dir::Right => self.right(),
        }
    }
    pub fn to_checked(&self, d: Dir, w: N, h: N) -> Option<Self> {
        match d {
            Dir::Up => {
                if self.y > N::zero() {
                    Some(self.up())
                } else {
                    None
                }
            }
            Dir::Down => {
                if self.y < (h - N::one()) {
                    Some(self.down())
                } else {
                    None
                }
            }
            Dir::Left => {
                if self.x > N::zero() {
                    Some(self.left())
                } else {
                    None
                }
            }
            Dir::Right => {
                if self.x < (w - N::one()) {
                    Some(self.right())
                } else {
                    None
                }
            }
        }
    }
    pub fn go_up(&mut self) {
        *self = self.up();
    }
    pub fn go_down(&mut self) {
        *self = self.down();
    }
    pub fn go_left(&mut self) {
        *self = self.left();
    }
    pub fn go_right(&mut self) {
        *self = self.right();
    }
    pub fn go(&mut self, d: Dir) -> &mut Self {
        *self = self.to(d);
        self
    }
    pub fn neighbours(&self) -> impl Iterator<Item = Self> + '_ {
        Dir::iter().map(|d| self.to(d))
    }
    pub fn neighbours_checked(&self, w: N, h: N) -> impl Iterator<Item = Self> + '_ {
        Dir::iter().flat_map(move |d| self.to_checked(d, w, h))
    }
}

impl<N: ToPrimitive> Coord<N> {
    fn to_usize(&self) -> (usize, usize) {
        let y = self.y.to_usize().expect("invalid Y coordinate");
        let x = self.x.to_usize().expect("invalid X coordinate");
        (x, y)
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    w: usize,
    values: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(it: impl Iterator<Item = Vec<T>>) -> Self {
        let mut w = None;
        let mut values = Vec::with_capacity(it.size_hint().0);
        for v in it {
            match w {
                None => w = Some(v.len()),
                Some(w) => assert!(
                    w == v.len(),
                    "row width {} does not match previous row width {}",
                    v.len(),
                    w
                ),
            };
            values.extend(v)
        }
        Self {
            w: w.unwrap_or(0),
            values,
        }
    }

    pub fn from_iter(w: usize, it: impl Iterator<Item = T>) -> Self {
        Self {
            w,
            values: it.collect_vec(),
        }
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.values.len() / self.w
    }

    /// An iterator over the coordinates
    pub fn coords<N>(&self) -> impl Iterator<Item = Coord<N>>
    where
        N: Integer + Copy + FromPrimitive,
    {
        (0..self.height())
            .cartesian_product(0..self.w)
            .map(|(y, x)| {
                let x = N::from_usize(x).expect("invalid width");
                let y = N::from_usize(y).expect("invalid height");
                Coord(x, y)
            })
    }

    /// An iterator over the rows
    pub fn rows_iter(&self) -> impl Iterator<Item = &[T]> {
        self.values.chunks(self.w)
    }

    pub fn neighbours_mut<N>(&mut self, coord: Coord<N>, mut f: impl FnMut(&mut T))
    where
        N: Integer + Copy + FromPrimitive + ToPrimitive,
    {
        let w = N::from_usize(self.w).expect("invalid width");
        let h = N::from_usize(self.height()).expect("invalid height");
        coord
            .neighbours_checked(w, h)
            .for_each(|coord| f(&mut self[coord]));
    }
}

impl<T, N> Index<Coord<N>> for Grid<T>
where
    N: ToPrimitive,
{
    type Output = T;

    fn index(&self, index: Coord<N>) -> &Self::Output {
        let (x, y) = index.to_usize();
        &self.values[y * self.w + x]
    }
}

impl<T, N> IndexMut<Coord<N>> for Grid<T>
where
    N: ToPrimitive,
{
    fn index_mut(&mut self, index: Coord<N>) -> &mut Self::Output {
        let (x, y) = index.to_usize();
        &mut self.values[y * self.w + x]
    }
}

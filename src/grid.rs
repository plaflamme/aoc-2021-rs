use std::{
    iter::once,
    ops::{Index, IndexMut},
    str::FromStr,
};

use itertools::{Either, FoldWhile, Itertools};
use num::{FromPrimitive, Integer, Signed, ToPrimitive, Unsigned};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn all() -> impl Iterator<Item = Dir> + Clone {
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
pub struct Pt<N = i32> {
    pub y: N, // comes before x for Ord
    pub x: N,
}

impl<N: Integer + Copy> Pt<N> {
    pub const fn new(x: N, y: N) -> Self {
        Self { x, y }
    }
    pub fn up(self) -> Self {
        Pt::new(self.x, self.y - N::one())
    }
    pub fn down(self) -> Self {
        Pt::new(self.x, self.y + N::one())
    }
    pub fn left(self) -> Self {
        Pt::new(self.x - N::one(), self.y)
    }
    pub fn right(self) -> Self {
        Pt::new(self.x + N::one(), self.y)
    }
    pub fn to(self, d: Dir) -> Self {
        match d {
            Dir::Up => self.up(),
            Dir::Down => self.down(),
            Dir::Left => self.left(),
            Dir::Right => self.right(),
        }
    }
    pub fn to_checked(self, d: Dir, w: N, h: N) -> Option<Self> {
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
    pub fn to_wrapping(self, d: Dir, w: N, h: N) -> Self {
        match self.to_checked(d, w, h) {
            Some(to) => to,
            None => match d {
                Dir::Up => Self::new(self.x, h - N::one()),
                Dir::Down => Self::new(self.x, N::zero()),
                Dir::Left => Self::new(w - N::one(), self.y),
                Dir::Right => Self::new(N::zero(), self.y),
            },
        }
    }
    // Tries to travel the specified directions. Returns `Right` if it travelled all the way, `Left` otherwise.
    pub fn travel_checked(
        &self,
        dirs: impl IntoIterator<Item = Dir>,
        w: N,
        h: N,
    ) -> Either<Self, Self> {
        let traveled =
            dirs.into_iter()
                .fold_while(*self, |pt, dir| match pt.to_checked(dir, w, h) {
                    None => FoldWhile::Done(pt),
                    Some(pt) => FoldWhile::Continue(pt),
                });

        match traveled {
            FoldWhile::Done(pt) => Either::Left(pt),
            FoldWhile::Continue(pt) => Either::Right(pt),
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
        Dir::all().map(|d| self.to(d))
    }
    pub fn diagonals(&self) -> impl Iterator<Item = Self> + '_ {
        once(Dir::Up).chain(once(Dir::Down)).flat_map(|d| {
            let up_down = self.to(d);
            once(up_down.to(Dir::Left)).chain(once(up_down.to(Dir::Right)))
        })
    }
    pub fn neighbours_checked(&self, w: N, h: N) -> impl Iterator<Item = Self> + '_ {
        Dir::all().flat_map(move |d| self.to_checked(d, w, h))
    }
    pub fn diagonals_checked(&self, w: N, h: N) -> impl Iterator<Item = Self> + '_ {
        once(Dir::Up)
            .chain(once(Dir::Down))
            .flat_map(move |d| self.to_checked(d, w, h))
            .flat_map(move |up_down| {
                up_down
                    .to_checked(Dir::Left, w, h)
                    .into_iter()
                    .chain(up_down.to_checked(Dir::Right, w, h))
            })
    }
}

impl<N> Pt<N>
where
    N: Integer + Unsigned + Copy,
{
    // unfortunately, we have to use different names for usigned vs signed
    pub fn manhattan_unsigned(&self, other: &Pt<N>) -> N {
        // can't use abs_diff

        let x = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let y = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        x + y
    }
}

impl<N> Pt<N>
where
    N: Integer + Signed + Copy,
{
    // unfortunately, we have to use different names for usigned vs signed
    pub fn manhattan_signed(&self, other: &Pt<N>) -> N {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<N: ToPrimitive> Pt<N>
where
    N: Unsigned + core::fmt::Debug,
{
    fn to_usize(&self) -> (usize, usize) {
        let y = self
            .y
            .to_usize()
            .unwrap_or_else(|| panic!("invalid Y coordinate {:?}", self.y));
        let x = self
            .x
            .to_usize()
            .unwrap_or_else(|| panic!("invalid X coordinate {:?}", self.x));
        (x, y)
    }
}

impl<N> FromStr for Pt<N>
where
    N: Integer + FromStr + Copy,
    <N as FromStr>::Err: std::error::Error + 'static,
{
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("not a Pt: {}", s))?;

        Ok(Pt::new(x.trim().parse::<N>()?, y.trim().parse::<N>()?))
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    w: usize,
    values: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_row_iter(it: impl Iterator<Item = Vec<T>>) -> Self {
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

    pub fn from_iter(w: usize, it: impl IntoIterator<Item = T>) -> Self {
        Self {
            w,
            values: it.into_iter().collect_vec(),
        }
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.values.len() / self.w
    }

    pub fn get<N>(&self, pt: Pt<N>) -> Option<&T>
    where
        N: Integer + Copy + FromPrimitive + ToPrimitive,
    {
        if pt.x < N::zero() || pt.x >= N::from_usize(self.width()).unwrap() {
            return None;
        }
        if pt.y < N::zero() || pt.y >= N::from_usize(self.height()).unwrap() {
            return None;
        }
        let x = N::to_usize(&pt.x).unwrap();
        let y = N::to_usize(&pt.y).unwrap();
        let v = &self.values[y * self.width() + x];
        Some(v)
    }

    /// An iterator over the coordinates
    pub fn pts<N>(&self) -> impl Iterator<Item = Pt<N>>
    where
        N: Integer + Copy + FromPrimitive,
    {
        (0..self.height())
            .cartesian_product(0..self.w)
            .map(|(y, x)| {
                let x = N::from_usize(x).expect("invalid width");
                let y = N::from_usize(y).expect("invalid height");
                Pt::new(x, y)
            })
    }

    /// An iterator over the rows
    pub fn rows_iter(&self) -> impl Iterator<Item = &[T]> {
        self.values.chunks(self.w)
    }

    pub fn neighbours_mut<N>(&mut self, coord: Pt<N>, mut f: impl FnMut(&mut T))
    where
        N: Integer + Copy + FromPrimitive + ToPrimitive + Unsigned + core::fmt::Debug,
    {
        let w = N::from_usize(self.w).expect("invalid width");
        let h = N::from_usize(self.height()).expect("invalid height");
        coord
            .neighbours_checked(w, h)
            .for_each(|coord| f(&mut self[coord]));
    }
}

impl<T, N> Index<Pt<N>> for Grid<T>
where
    N: ToPrimitive + Unsigned + core::fmt::Debug,
{
    type Output = T;

    fn index(&self, index: Pt<N>) -> &Self::Output {
        let (x, y) = index.to_usize();
        &self.values[y * self.w + x]
    }
}

impl<T, N> IndexMut<Pt<N>> for Grid<T>
where
    N: ToPrimitive + Unsigned + core::fmt::Debug,
{
    fn index_mut(&mut self, index: Pt<N>) -> &mut Self::Output {
        let (x, y) = index.to_usize();
        &mut self.values[y * self.w + x]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dir_all() {
        let all = Dir::all().collect_vec();
        assert!(all.contains(&Dir::Up));
        assert!(all.contains(&Dir::Down));
        assert!(all.contains(&Dir::Left));
        assert!(all.contains(&Dir::Right));
    }

    #[test]
    fn test_dir_flip() {
        assert_eq!(Dir::Up.flip(), Dir::Down);
        assert_eq!(Dir::Right.flip(), Dir::Left);
        assert_eq!(Dir::Down.flip(), Dir::Up);
        assert_eq!(Dir::Left.flip(), Dir::Right);
    }
    #[test]
    fn test_dir_ul_dr() {
        assert_eq!(Dir::Up.ul_dr(), Dir::Left);
        assert_eq!(Dir::Right.ul_dr(), Dir::Down);
        assert_eq!(Dir::Down.ul_dr(), Dir::Right);
        assert_eq!(Dir::Left.ul_dr(), Dir::Up);
    }
    #[test]
    fn test_dir_ur_dl() {
        assert_eq!(Dir::Up.ur_dl(), Dir::Right);
        assert_eq!(Dir::Right.ur_dl(), Dir::Up);
        assert_eq!(Dir::Down.ur_dl(), Dir::Left);
        assert_eq!(Dir::Left.ur_dl(), Dir::Down);
    }
    #[test]
    fn test_dir_rot_left() {
        assert_eq!(Dir::Up.rot_left(), Dir::Left);
        assert_eq!(Dir::Right.rot_left(), Dir::Up);
        assert_eq!(Dir::Down.rot_left(), Dir::Right);
        assert_eq!(Dir::Left.rot_left(), Dir::Down);
    }

    #[test]
    fn test_dir_rot_right() {
        assert_eq!(Dir::Up.rot_right(), Dir::Right);
        assert_eq!(Dir::Right.rot_right(), Dir::Down);
        assert_eq!(Dir::Down.rot_right(), Dir::Left);
        assert_eq!(Dir::Left.rot_right(), Dir::Up);
    }

    #[test]
    fn test_pt_to() {
        let start = Pt::new(0, 0);
        assert_eq!(start.up(), Pt::new(0, -1));
        assert_eq!(start.up(), start.to(Dir::Up));

        assert_eq!(start.right(), Pt::new(1, 0));
        assert_eq!(start.right(), start.to(Dir::Right));

        assert_eq!(start.down(), Pt::new(0, 1));
        assert_eq!(start.down(), start.to(Dir::Down));

        assert_eq!(start.left(), Pt::new(-1, 0));
        assert_eq!(start.left(), start.to(Dir::Left));
    }
    #[test]
    fn test_pt_to_checked() {
        assert_eq!(Pt::new(0, 0).to_checked(Dir::Up, 2, 2), None);
        assert_eq!(Pt::new(0, 1).to_checked(Dir::Up, 2, 2), Some(Pt::new(0, 0)));

        assert_eq!(Pt::new(1, 0).to_checked(Dir::Right, 2, 2), None);
        assert_eq!(
            Pt::new(0, 0).to_checked(Dir::Right, 2, 2),
            Some(Pt::new(1, 0))
        );

        assert_eq!(Pt::new(0, 1).to_checked(Dir::Down, 2, 2), None);
        assert_eq!(
            Pt::new(0, 0).to_checked(Dir::Down, 2, 2),
            Some(Pt::new(0, 1))
        );

        assert_eq!(Pt::new(0, 0).to_checked(Dir::Left, 2, 2), None);
        assert_eq!(
            Pt::new(1, 0).to_checked(Dir::Left, 2, 2),
            Some(Pt::new(0, 0))
        );
    }
    #[test]
    fn test_pt_go() {
        let mut start = Pt::new(0, 0);

        start.go_up();
        assert_eq!(start, Pt::new(0, -1));
        start.go(Dir::Up);
        assert_eq!(start, Pt::new(0, -2));

        let mut start = Pt::new(0, 0);
        start.go_right();
        assert_eq!(start, Pt::new(1, 0));
        start.go(Dir::Right);
        assert_eq!(start, Pt::new(2, 0));

        let mut start = Pt::new(0, 0);
        start.go_down();
        assert_eq!(start, Pt::new(0, 1));
        start.go(Dir::Down);
        assert_eq!(start, Pt::new(0, 2));

        let mut start = Pt::new(0, 0);
        start.go_left();
        assert_eq!(start, Pt::new(-1, 0));
        start.go(Dir::Left);
        assert_eq!(start, Pt::new(-2, 0));
    }
    #[test]
    fn test_pt_neighbours() {
        let n = Pt::new(0, 0).neighbours().collect_vec();
        assert!(n.contains(&Pt::new(0, -1)));
        assert!(n.contains(&Pt::new(1, 0)));
        assert!(n.contains(&Pt::new(0, 1)));
        assert!(n.contains(&Pt::new(-1, 0)));
    }
    #[test]
    fn test_pt_neighbours_checked() {
        let n = Pt::new(0, 0).neighbours_checked(2, 2).collect_vec();
        assert!(n.contains(&Pt::new(1, 0)));
        assert!(n.contains(&Pt::new(0, 1)));
        let n = Pt::new(0, 0).neighbours_checked(1, 1).collect_vec();
        assert!(n.is_empty());
    }
    #[test]
    fn test_pt_diagonals_checked() {
        let n = Pt::new(1, 1).diagonals_checked(3, 3).collect_vec();
        assert!(n.contains(&Pt::new(0, 0)));
        assert!(n.contains(&Pt::new(2, 0)));
        assert!(n.contains(&Pt::new(2, 2)));
        assert!(n.contains(&Pt::new(0, 2)));

        let n = Pt::new(0, 0).diagonals_checked(2, 2).collect_vec();
        assert!(n.contains(&Pt::new(1, 1)));

        let n = Pt::new(0, 0).diagonals_checked(1, 1).collect_vec();
        assert!(n.is_empty());
    }
    #[test]
    fn test_pt_travel_checked() {
        let n =
            Pt::new(1_u8, 1).travel_checked(vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right], 3, 3);
        assert_eq!(n, Either::Right(Pt::new(1, 1)));
        let n = Pt::new(1_u8, 1).travel_checked(vec![Dir::Up, Dir::Up], 3, 3);
        assert_eq!(n, Either::Left(Pt::new(1, 0)));
    }
    #[test]
    fn test_pt_manhattan_distance() {
        assert_eq!(Pt::new(1, 1).manhattan_signed(&Pt::new(-1, -1)), 4);
        assert_eq!(Pt::new(-1, -1).manhattan_signed(&Pt::new(1, 1)), 4);
        assert_eq!(Pt::new(0_u32, 0).manhattan_unsigned(&Pt::new(2, 2)), 4);
        assert_eq!(Pt::new(2_u32, 2).manhattan_unsigned(&Pt::new(0, 0)), 4);
    }

    #[test]
    fn test_from_str() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!("1,1".parse::<Pt<u8>>()?, Pt::new(1, 1));
        assert_eq!("01,1".parse::<Pt<u8>>()?, Pt::new(1, 1));
        assert_eq!(" 1,10".parse::<Pt<u8>>()?, Pt::new(1, 10));
        assert_eq!("1, 0".parse::<Pt<u8>>()?, Pt::new(1, 0));
        Ok(())
    }

    #[test]
    fn test_grid_from_row_iter() {
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let grid = Grid::from_row_iter(rows.clone().into_iter());
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.rows_iter().collect_vec(), rows);
    }

    #[test]
    #[should_panic]
    fn test_grid_from_row_iter_will_panic() {
        let rows = vec![vec![1, 2, 3], vec![4, 5]];
        Grid::from_row_iter(rows.into_iter());
    }
    #[test]
    fn test_grid_from_iter() {
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let grid = Grid::from_iter(3, rows.clone().into_iter().flatten());
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.rows_iter().collect_vec(), rows);
    }
    #[test]
    fn test_grid_pts() {
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let grid = Grid::from_iter(3, rows.into_iter().flatten());
        let pts = grid.pts().collect_vec();
        assert_eq!(
            pts,
            vec![
                Pt::new(0, 0),
                Pt::new(1, 0),
                Pt::new(2, 0),
                Pt::new(0, 1),
                Pt::new(1, 1),
                Pt::new(2, 1)
            ]
        )
    }
}

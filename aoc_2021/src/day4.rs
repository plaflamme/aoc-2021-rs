use itertools::Itertools;

use crate::{tools::empty_line_delimited_batches, Day4};

#[derive(Clone, Debug)]
struct Board {
    cells: Vec<u32>,
    cols: usize,
    calls: Vec<bool>,
}

impl Board {
    fn new(cells: Vec<Vec<u32>>) -> Self {
        let flat_cells: Vec<u32> = cells.clone().into_iter().flatten().collect();
        let calls = vec![false; flat_cells.len()];
        Board {
            cells: flat_cells,
            cols: cells.get(0).unwrap().len(),
            calls,
        }
    }

    fn call(&mut self, value: u32) -> bool {
        if let Some((idx, _)) = self.cells.iter().find_position(|n| **n == value) {
            *self.calls.get_mut(idx).unwrap() = true;

            return self.wins();
        }
        false
    }

    fn wins(&self) -> bool {
        if self.rows().into_iter().any(|mut r| r.all(|v| *v)) {
            return true;
        }
        if self.cols().any(|mut c| c.all(|v| *v)) {
            return true;
        }
        false
    }

    fn rows(&self) -> itertools::IntoChunks<impl Iterator<Item = &bool>> {
        self.calls.iter().chunks(self.cols)
    }

    fn cols(&self) -> impl Iterator<Item = impl Iterator<Item = &bool>> {
        (0..self.cols)
            .into_iter()
            .map(|c| self.calls.iter().skip(c).step_by(self.cols))
    }

    fn unmarked(&self) -> impl Iterator<Item = &u32> {
        self.calls
            .iter()
            .zip(self.cells.iter())
            .filter_map(|(call, value)| if !*call { Some(value) } else { None })
    }
}

struct SolvedBoard(u32, u8);

struct SolutionsIter<N> {
    next_num: N,
    next_solved: std::vec::IntoIter<SolvedBoard>, // TODO: how do we abstract over this?
    remaining_boards: Vec<Board>,
}

impl<N> Iterator for SolutionsIter<N>
where
    N: Iterator<Item = (usize, u32)>,
{
    type Item = SolvedBoard;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_solved.next() {
            None => loop {
                if let Some((count, n)) = self.next_num.next() {
                    let solved = self
                        .remaining_boards
                        .drain_filter(|b| b.call(n))
                        .map(|b| SolvedBoard(b.unmarked().sum::<u32>() * n, (count + 1) as u8))
                        .collect_vec();

                    if !solved.is_empty() {
                        self.next_solved = solved.into_iter();
                        break self.next_solved.next();
                    }
                } else {
                    break None;
                }
            },
            s => s,
        }
    }
}

pub struct Solution(Vec<u32>, Vec<Board>);

impl Solution {
    fn solved_boards(self) -> impl Iterator<Item = SolvedBoard> {
        SolutionsIter {
            next_num: self.0.into_iter().enumerate(),
            next_solved: Vec::new().into_iter(),
            remaining_boards: self.1,
        }
    }
}

sample!(
    crate::Day4,
    "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
",
    "4512",
    "1924"
);

impl super::Solver for Day4 {
    type Output = u32;
    type Input = Solution;

    fn parse(input: &str) -> Self::Input
    where
        Self: Sized,
    {
        let mut lines = input.lines();
        let nums = lines.next().unwrap();
        let nums = nums.split(',').map(|n| n.parse::<u32>().unwrap()).collect();

        let boards = empty_line_delimited_batches(lines)
            .map(|board| {
                let cells = board
                    .into_iter()
                    .map(|row| {
                        row.split(' ')
                            .filter(|n| !n.is_empty())
                            .map(|n| n.parse::<u32>().unwrap())
                            .collect_vec()
                    })
                    .collect_vec();
                Board::new(cells)
            })
            .collect_vec();

        Solution(nums, boards)
    }

    fn part1(input: Self::Input) -> Self::Output {
        input
            .solved_boards()
            .next()
            .expect("didn't find any solution :(")
            .0
    }

    fn part2(input: Self::Input) -> Self::Output {
        input
            .solved_boards()
            .max_by(|SolvedBoard(_, count), SolvedBoard(_, count2)| count.cmp(count2))
            .unwrap()
            .0
    }
}

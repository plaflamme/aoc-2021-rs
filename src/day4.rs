use itertools::Itertools;

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

struct SolvedBoard(u32, u32);

pub struct Solution(Vec<u32>, Vec<Board>);

impl Solution {
    fn solved_boards(&self) -> impl Iterator<Item = SolvedBoard> + '_ {
        self.1.iter().cloned().flat_map(|mut board| {
            let mut count = 0;
            for n in self.0.iter() {
                count += 1;
                if board.call(*n) {
                    return Some(SolvedBoard(board.unmarked().sum::<u32>() * n, count));
                }
            }
            None
        })
    }
}

impl super::Solver for Solution {
    const SAMPLE: &'static str =
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
";

    const LEVEL1: &'static str = "4512";

    const LEVEL2: &'static str = "1924";

    type Output = u32;

    fn parse(input: &str) -> Self
    where
        Self: Sized,
    {
        let mut lines = input.lines();
        let nums = lines.next().unwrap();
        let nums = nums.split(',').map(|n| n.parse::<u32>().unwrap()).collect();

        lines.next();
        let (mut boards, last_board) = lines.fold(
            (Vec::new(), Vec::new()),
            |(mut boards, mut cells), line| match line {
                "" => {
                    boards.push(Board::new(cells));
                    (boards, Vec::new())
                }
                nums => {
                    let row = nums
                        .split(' ')
                        .filter(|n| n.len() > 0)
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect();
                    cells.push(row);
                    (boards, cells)
                }
            },
        );

        boards.push(Board::new(last_board));

        Solution(nums, boards)
    }

    fn part1(self) -> Self::Output {
        // not the most efficient, we could iterate on numbers first so that we can do .next().0
        self.solved_boards()
            .min_by(|SolvedBoard(_, count), SolvedBoard(_, count2)| count.cmp(count2))
            .unwrap()
            .0
    }

    fn part2(self) -> Self::Output {
        self.solved_boards()
            .max_by(|SolvedBoard(_, count), SolvedBoard(_, count2)| count.cmp(count2))
            .unwrap()
            .0
    }
}

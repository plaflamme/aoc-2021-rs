use std::collections::HashMap;

use itertools::Itertools;

use crate::{Day21, Solver};

sample!(
    Day21,
    "Player 1 starting position: 4
Player 2 starting position: 8",
    "739785",
    "444356092776315"
);

struct Player<T: Iterator<Item = u8>>(u8, usize, T);

fn solve1(p1: u8, p2: u8) -> usize {
    let player1 = Player(p1, 0, (1..=10).cycle().skip(p1 as usize));
    let player2 = Player(p2, 0, (1..=10).cycle().skip(p2 as usize));
    let mut dice = (1..=100).cycle();
    let mut players = vec![player1, player2];

    let mut turn = 0;
    loop {
        let mut player = &mut players[turn % 2];
        turn += 1;
        let adv = (&mut dice).take(3).collect_vec();
        // let advance = (&mut dice).take(3).sum();

        let advance = adv.into_iter().sum();
        let score = { (&mut player.2).take(advance).last().unwrap() };
        player.1 += score as usize;

        if player.1 >= 1000 {
            break;
        }
    }

    let losing_score = players.iter().map(|p| p.1).min().unwrap();
    losing_score * turn * 3
}

fn dice_tabulation() -> impl Iterator<Item = (u8, usize)> {
    let mut tab = [0_usize; 10];
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                tab[a + b + c] += 1;
            }
        }
    }
    tab.into_iter()
        .enumerate()
        .filter(|(_, freq)| *freq > 0)
        .map(|(sum, freq)| (sum as u8, freq))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct QuantumState {
    pos: u8,
    score: u8,
}

impl QuantumState {
    fn new(pos: u8) -> Self {
        Self {
            pos: pos - 1,
            score: 0,
        }
    }
    fn step(&self, rolled: u8) -> Self {
        let pos = (self.pos + rolled) % 10;
        let score = self.score + pos + 1;
        Self { pos, score }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GameState([QuantumState; 2]);

#[derive(Debug)]
struct QuantumUniverse {
    games: HashMap<GameState, usize>, // number of copies of a particular game state
    wins: [usize; 2],
}

impl QuantumUniverse {
    fn new(p1: u8, p2: u8) -> Self {
        Self {
            games: HashMap::from([(GameState([QuantumState::new(p1), QuantumState::new(p2)]), 1)]),
            wins: [0; 2],
        }
    }

    fn step_and_collapse(&mut self, player: usize) {
        let new_states = self
            .games
            .iter()
            .flat_map(|(game, copies)| {
                dice_tabulation()
                    .into_iter()
                    .flat_map(|(rolled, times)| {
                        let player_state = game.0[player].step(rolled);
                        if player_state.score >= 21 {
                            self.wins[player] += copies * times;
                            None
                        } else {
                            let mut game = game.clone();
                            game.0[player] = player_state;
                            Some((game, copies * times))
                        }
                    })
                    .collect_vec() // this is necessary because of self.wins reference; probably a limitation of the borrow checker
            })
            .into_grouping_map()
            .sum();

        self.games = new_states;
    }

    fn solve(&mut self) {
        while self.games.len() > 0 {
            self.step_and_collapse(0);
            self.step_and_collapse(1);
        }
    }
}

impl Solver for Day21 {
    type Output = usize;

    type Input = (u8, u8);

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| l.chars().last().unwrap().to_string().parse::<u8>().unwrap())
            .tuples()
            .exactly_one()
            .ok()
            .unwrap()
    }

    fn part1(input: Self::Input) -> Self::Output {
        solve1(input.0, input.1)
    }

    fn part2(input: Self::Input) -> Self::Output {
        let mut q = QuantumUniverse::new(input.0, input.1);
        q.solve();
        q.wins[0].max(q.wins[1])
    }
}

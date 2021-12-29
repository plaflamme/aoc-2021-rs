use std::{
    collections::{HashMap, HashSet},
    ops::{Index, IndexMut},
};

use crate::{Day24, Solver};
use itertools::Itertools;

sample!(Day24, "", "");

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
enum Reg {
    w,
    x,
    y,
    z,
}

impl Reg {
    fn from_str(s: &str) -> Self {
        match s {
            "w" => Reg::w,
            "x" => Reg::x,
            "y" => Reg::y,
            "z" => Reg::z,
            _ => panic!("invalid register {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(Reg),
    Int(isize),
}
impl Operand {
    fn from_str(s: &str) -> Self {
        match s.parse::<isize>() {
            Ok(int) => Operand::Int(int),
            _ => Operand::Register(Reg::from_str(s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Inp(Reg),
    Mul(Reg, Operand),
    Mod(Reg, Operand),
    Add(Reg, Operand),
    Div(Reg, Operand),
    Eql(Reg, Operand),
}

impl Instr {
    fn from_str(s: &str) -> Self {
        let (instr, reg) = s.split_once(' ').unwrap();
        match instr {
            "inp" => Instr::Inp(Reg::from_str(reg)),
            bin => {
                let (reg, op) = reg.split_once(' ').unwrap();
                let reg = Reg::from_str(reg);
                let op = Operand::from_str(op);
                match bin {
                    "mul" => Instr::Mul(reg, op),
                    "mod" => Instr::Mod(reg, op),
                    "add" => Instr::Add(reg, op),
                    "div" => Instr::Div(reg, op),
                    "eql" => Instr::Eql(reg, op),
                    _ => panic!("invalid instruction {}", s),
                }
            }
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct Alu {
    x: isize,
    y: isize,
    w: isize,
    z: isize,
}

impl Index<Reg> for Alu {
    type Output = isize;

    fn index(&self, index: Reg) -> &Self::Output {
        match index {
            Reg::w => &self.w,
            Reg::x => &self.x,
            Reg::y => &self.y,
            Reg::z => &self.z,
        }
    }
}

impl IndexMut<Reg> for Alu {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
        match index {
            Reg::w => &mut self.w,
            Reg::x => &mut self.x,
            Reg::y => &mut self.y,
            Reg::z => &mut self.z,
        }
    }
}

impl Alu {
    fn fetch(&self, op: &Operand) -> isize {
        match op {
            Operand::Register(reg) => self[*reg],
            Operand::Int(int) => *int,
        }
    }

    fn exec(&mut self, instr: &[Instr], input: isize) {
        for &i in instr {
            match i {
                Instr::Inp(r) => self[r] = input,
                Instr::Mul(r, op) => self[r] *= self.fetch(&op),
                Instr::Mod(r, op) => self[r] %= self.fetch(&op),
                Instr::Add(r, op) => self[r] += self.fetch(&op),
                Instr::Div(r, op) => self[r] /= self.fetch(&op),
                Instr::Eql(r, op) => self[r] = (self[r] == self.fetch(&op)).into(),
            }
        }
    }
}

#[derive(Debug)]
struct CipherStep(Reg, Vec<Instr>);

impl CipherStep {
    fn input(&self) -> Reg {
        self.0
    }

    fn compute(&self, input: isize, state: &mut Alu) {
        state.exec(&self.1, input);
    }
}
#[derive(Debug)]
pub struct Cipher {
    ciphers: Vec<CipherStep>,
}

impl Cipher {
    fn new(program: Vec<Instr>) -> Self {
        let ciphers = program
            .into_iter()
            .batching(|it| match it.next() {
                None => None,
                Some(init @ Instr::Inp(input)) => {
                    let rest = it.take_while_ref(|i| !matches!(i, Instr::Inp(_)));
                    Some(CipherStep(
                        input,
                        std::iter::once(init).chain(rest).collect(),
                    ))
                }
                Some(_) => unreachable!(),
            })
            .collect_vec();

        assert_eq!(ciphers.len(), 14);

        Self { ciphers }
    }

    fn compute_inputs(&self) -> Vec<HashSet<Alu>> {
        // compute the output of each step, except for the last one
        self.ciphers.iter().tuple_windows().fold(
            vec![HashSet::from([Alu::default()])],
            |mut inputs, (cipher, next_cipher)| {
                let step_inputs = inputs.last().unwrap();
                let step_outputs = step_inputs
                    .iter()
                    .flat_map(|s| (1..=9).map(move |i| (s, i)))
                    .map(|(state, input)| {
                        let mut output = state.clone();
                        cipher.compute(input, &mut output);
                        // the next step will clear this register, so its value doesn't matter here.
                        output[next_cipher.input()] = 0;
                        output
                    })
                    .collect();
                inputs.push(step_outputs);
                inputs
            },
        )
    }

    // this solution computes the possible inputs for each step, then backtracks from the last step
    // passing the valid ones back to the previous step, remembering which digit produced the valid ones
    fn solve_backtracking(&self) -> (usize, usize) {
        let mut digits = vec![];
        self.ciphers.iter().zip(self.compute_inputs()).rev().fold(
            None,
            |valid: Option<(Reg, HashSet<Alu>)>, (cipher, possible_inputs)| {
                let (valid_outputs, valid_digits): (HashSet<_>, HashSet<_>) = possible_inputs
                    .iter()
                    .flat_map(|s| (1_u8..=9).map(move |i| (s, i)))
                    .filter(|(s, input)| {
                        let mut state = (*s).clone();
                        cipher.compute(*input as isize, &mut state);
                        match &valid {
                            None => state.z == 0,
                            Some((input_reg, valid_outputs)) => {
                                state[*input_reg] = 0;
                                valid_outputs.contains(&state)
                            }
                        }
                    })
                    .map(|(s, i)| (s.clone(), i))
                    .unzip();

                let (min, max) = valid_digits.into_iter().minmax().into_option().unwrap();

                digits.push((min, max));

                Some((cipher.input(), valid_outputs))
            },
        );
        digits
            .into_iter()
            .rev()
            .fold((0, 0), |(min, max), (rmin, rmax)| {
                (min * 10 + rmin as usize, max * 10 + rmax as usize)
            })
    }

    // kind of a brute force approach, but collapses common states so that we limit the exponential growth between steps.
    fn solve_direct(&self) -> (usize, usize) {
        let init = HashMap::from([(Alu::default(), (usize::MAX, 0))]);
        let state_index: HashMap<Alu, (usize, usize)> =
            self.ciphers
                .iter()
                .enumerate()
                .fold(init, |state_index, (idx, cipher)| {
                    let mut outputs = HashMap::with_capacity(state_index.len());
                    state_index
                        .iter()
                        .flat_map(|s| (1..=9).map(move |i| (s, i)))
                        .map(|((state, (current_min, current_max)), input)| {
                            let mut output = state.clone();
                            cipher.compute(input, &mut output);
                            if let Some(next_cipher) = self.ciphers.get(idx + 1) {
                                // the next step will clear this register, so its value doesn't matter here.
                                output[next_cipher.input()] = 0;
                            }

                            let new_min = current_min * 10 + input as usize;
                            let new_max = current_max * 10 + input as usize;
                            (output, (new_min, new_max))
                        })
                        .for_each(|(key, (rmin, rmax))| {
                            outputs
                                .entry(key)
                                .and_modify(|(min, max)| {
                                    *min = usize::min(*min, rmin);
                                    *max = usize::max(*max, rmax);
                                })
                                .or_insert((rmin, rmax));
                        });
                    outputs
                });

        state_index
            .into_iter()
            .filter_map(|(state, minmax)| if state.z == 0 { Some(minmax) } else { None })
            .fold1(|(lmin, lmax), (rmin, rmax)| (lmin.min(rmin), lmax.max(rmax)))
            .unwrap()
    }
}

impl Solver for Day24 {
    type Output = usize;

    type Input = Cipher;

    fn parse(input: &str) -> Self::Input {
        Cipher::new(input.lines().map(Instr::from_str).collect())
    }

    fn part1(input: Self::Input) -> Self::Output {
        input.solve_backtracking().1
    }

    fn part2(input: Self::Input) -> Self::Output {
        input.solve_backtracking().0
    }
}

#[derive(Debug)]
pub struct Direct;
impl Solver<Direct> for Day24 {
    type Output = usize;

    type Input = Cipher;

    fn parse(input: &str) -> Self::Input {
        Cipher::new(input.lines().map(Instr::from_str).collect())
    }

    fn part1(input: Self::Input) -> Self::Output {
        input.solve_direct().1
    }

    fn part2(input: Self::Input) -> Self::Output {
        input.solve_direct().0
    }
}

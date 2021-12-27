use std::{
    collections::HashSet,
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

    fn exec(&mut self, instr: &Vec<Instr>, input: isize) {
        for &i in instr {
            match i {
                Instr::Inp(r) => self[r] = input,
                Instr::Mul(r, op) => self[r] = self[r] * self.fetch(&op),
                Instr::Mod(r, op) => self[r] = self[r] % self.fetch(&op),
                Instr::Add(r, op) => self[r] = self[r] + self.fetch(&op),
                Instr::Div(r, op) => self[r] = self[r] / self.fetch(&op),
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
                    let rest = it.take_while_ref(|i| match i {
                        Instr::Inp(_) => false,
                        _ => true,
                    });
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
        let mut inputs = vec![HashSet::new()];
        inputs[0].insert(Alu::default());
        // compute the output of each step, except for the last one
        for (idx, c) in self.ciphers.iter().dropping_back(1).enumerate() {
            let mut step_outputs = HashSet::new();
            let next_step_input_register = self.ciphers[idx + 1].input();
            for state in &inputs[idx] {
                for input in 1..=9 {
                    let mut output = state.clone();
                    c.compute(input, &mut output);
                    // the next step will clear this register, so its value doesn't matter here.
                    output[next_step_input_register] = 0;
                    step_outputs.insert(output);
                }
            }
            inputs.push(step_outputs);
        }
        inputs
    }

    fn min_max_digits(&self) -> Vec<(u8, u8)> {
        let inputs = self.compute_inputs();

        let input_ciphers = self
            .ciphers
            .iter()
            .zip(inputs.into_iter())
            .rev()
            .collect_vec();

        let mut digits = Vec::new();
        let mut max = 0_u8;
        let mut min = 9_u8;

        let ((last_step, inputs), rest) = input_ciphers.split_first().unwrap();
        let mut valid_outputs = HashSet::new();
        for input in inputs {
            for i in 1..=9 {
                let mut state = input.clone();
                last_step.compute(i, &mut state);
                if state.z == 0 {
                    max = max.max(i as u8);
                    min = min.min(i as u8);
                    valid_outputs.insert(input);
                }
            }
        }
        digits.push((min, max));

        rest.into_iter().fold(
            (last_step.input(), valid_outputs),
            |(input_reg, valid), (step, inputs)| {
                let mut valid_outputs = HashSet::new();
                let mut max = 0;
                let mut min = 9;
                for input in inputs {
                    for i in 1..=9 {
                        let mut state = input.clone();
                        step.compute(i, &mut state);
                        state[input_reg] = 0;
                        if valid.contains(&state) {
                            max = max.max(i as u8);
                            min = min.min(i as u8);
                            valid_outputs.insert(input);
                        }
                    }
                }
                digits.push((min, max));
                (step.input(), valid_outputs)
            },
        );

        digits
    }
}

impl Solver for Day24 {
    type Output = usize;

    type Input = Cipher;

    fn parse(input: &str) -> Self::Input {
        Cipher::new(input.lines().map(|s| Instr::from_str(s)).collect())
    }

    fn part1(input: Self::Input) -> Self::Output {
        input
            .min_max_digits()
            .into_iter()
            .map(|(_, max)| max)
            .enumerate()
            .map(|(idx, d)| 10_usize.pow(idx as u32) * d as usize)
            .sum::<usize>()
    }

    fn part2(input: Self::Input) -> Self::Output {
        input
            .min_max_digits()
            .into_iter()
            .map(|(min, _)| min)
            .enumerate()
            .map(|(idx, d)| 10_usize.pow(idx as u32) * d as usize)
            .sum::<usize>()
    }
}

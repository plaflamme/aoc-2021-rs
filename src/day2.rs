use std::{error::Error, str::FromStr};

pub struct Solution;

pub enum Command {
    Down(u32),
    Up(u32),
    Forward(u32),
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cmd, value)) = s.split_once(' ') {
            let value = u32::from_str(value)?;
            match cmd {
                "forward" => Ok(Command::Forward(value)),
                "up" => Ok(Command::Up(value)),
                "down" => Ok(Command::Down(value)),
                _ => panic!("unexpected command {}", s),
            }
        } else {
            panic!("unexpected command {}", s)
        }
    }
}

struct Position(i32, i32);

impl Position {
    fn new() -> Self {
        Position(0, 0)
    }

    fn apply(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(v) => self.0 += v as i32,
            Command::Down(v) => self.1 += v as i32,
            Command::Up(v) => self.1 -= v as i32,
        };
    }

    fn apply_all(&mut self, cmds: impl IntoIterator<Item = Command>) {
        cmds.into_iter().for_each(|cmd| self.apply(cmd));
    }
}

struct AimedPosition(i32, i32, i32);

impl AimedPosition {
    fn new() -> Self {
        AimedPosition(0, 0, 0)
    }

    fn apply(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(v) => {
                self.0 += v as i32;
                self.1 += self.2 * v as i32;
            }
            Command::Down(v) => self.2 += v as i32,
            Command::Up(v) => self.2 -= v as i32,
        };
    }

    fn apply_all(&mut self, cmds: impl IntoIterator<Item = Command>) {
        cmds.into_iter().for_each(|cmd| self.apply(cmd));
    }
}

impl super::Day for Solution {
    const SAMPLE: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2

";
    const LEVEL1: &'static str = "150";
    const LEVEL2: &'static str = "900";

    type Input = Vec<Command>;
    type Output = i32;

    fn parse(input: &str) -> Result<Self::Input, Box<dyn std::error::Error>> {
        Ok(input
            .lines()
            .filter(|l| l.len() > 0)
            .map(|l| Command::from_str(l).unwrap())
            .collect())
    }

    fn level1(input: Self::Input) -> Result<Self::Output, Box<dyn std::error::Error>> {
        let mut pos = Position::new();
        pos.apply_all(input);
        Ok(pos.0 * pos.1)
    }

    fn level2(input: Self::Input) -> Result<Self::Output, Box<dyn std::error::Error>> {
        let mut pos = AimedPosition::new();
        pos.apply_all(input);
        Ok(pos.0 * pos.1)
    }
}

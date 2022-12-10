use std::{io, str::FromStr};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: \n{}", part2(&input));
}

enum Error {
    ParsingError,
}

#[derive(Debug)]
pub(crate) enum Instruction {
    AddX(i32),
    NoOp,
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, instruction) = parsers::instruction(s).or(Err(Error::ParsingError))?;
        Ok(instruction)
    }
}

mod parsers {
    use nom::{branch::alt, bytes::streaming::tag, combinator::map, sequence::preceded};

    use crate::Instruction;

    pub(super) fn instruction(input: &str) -> nom::IResult<&str, Instruction> {
        alt((noop, addx))(input)
    }

    fn noop(input: &str) -> nom::IResult<&str, Instruction> {
        map(tag("noop"), |_| Instruction::NoOp)(input)
    }

    fn addx(input: &str) -> nom::IResult<&str, Instruction> {
        map(
            preceded(tag("addx "), nom::character::complete::i32),
            Instruction::AddX,
        )(input)
    }
}

struct Cpu {
    program: Vec<Instruction>,
    ip: usize,
    register_x: i64,
    instruction_time: u8,
}

impl Cpu {
    fn new() -> Self {
        Self {
            program: vec![],
            ip: 0,
            register_x: 1,
            instruction_time: 0,
        }
    }

    fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
    }

    fn tick(&mut self) {
        let instruction = &self.program[self.ip];
        match instruction {
            Instruction::AddX(v) => {
                if self.instruction_time >= 1 {
                    self.register_x += *v as i64;
                    self.instruction_time = 0;
                    self.ip += 1;
                } else {
                    self.instruction_time += 1;
                }
            }
            Instruction::NoOp => {
                self.ip += 1;
            }
        }
    }

    fn register_x(&self) -> i64 {
        self.register_x
    }
}

struct Crt {
    sprite_pos: u8,
    frame_buffer: [char; 240],
    draw_pos: u8,
}

impl Crt {
    fn new() -> Self {
        Self {
            sprite_pos: 1,
            frame_buffer: [' '; 240],
            draw_pos: 0,
        }
    }

    fn set_sprite_pos(&mut self, sprite_pos: u8) {
        self.sprite_pos = sprite_pos;
    }

    fn draw(&mut self) {
        self.frame_buffer[self.draw_pos as usize] =
            if self.sprite_pos.abs_diff(self.draw_pos % 40) <= 1 {
                '#'
            } else {
                '.'
            };
        self.draw_pos += 1;
    }

    fn screen(&self) -> String {
        itertools::intersperse(self.frame_buffer.chunks(40), &['\n'])
            .flatten()
            .collect()
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().flat_map(str::parse).collect()
}

fn part1(input: &str) -> i64 {
    let instructions = parse(input);
    let mut cpu = Cpu::new();
    let max_cycles = 220;
    cpu.load_program(instructions);
    let mut cycle_values = vec![];
    for cycle in 1usize..=max_cycles {
        cycle_values.push((cycle, cpu.register_x()));
        cpu.tick();
    }

    cycle_values
        .into_iter()
        .filter(|(cycle, _)| (cycle % 40) == 20)
        .map(|(cycle, v)| cycle as i64 * v)
        .sum()
}

fn part2(input: &str) -> String {
    let instructions = parse(input);
    let mut cpu = Cpu::new();
    let mut crt = Crt::new();
    let max_cycles = 240;
    cpu.load_program(instructions);
    for _cycle in 1usize..=max_cycles {
        crt.set_sprite_pos(cpu.register_x() as u8);
        cpu.tick();
        crt.draw();
    }

    crt.screen()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 13140);
    }

    #[test]
    fn part2_example() {
        let expected_output = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(part2(EXAMPLE_INPUT), expected_output);
    }
}

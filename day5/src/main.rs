use core::fmt;
use std::{
    io::{self, stdin},
    str::FromStr,
};

#[derive(Debug)]
enum Error {
    ParsingError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() -> io::Result<()> {
    let input = io::read_to_string(stdin())?;
    match part1(&input) {
        Ok(result) => println!("Part 1: {}", result),
        Err(err) => eprintln!("Part 1: Error = {}", err),
    }
    match part2(&input) {
        Ok(result) => println!("Part 2: {}", result),
        Err(err) => eprintln!("Part 2: Error = {}", err),
    }
    Ok(())
}

type Crate = char;

type Stack = Vec<Crate>;

type Yard = Vec<Stack>;

#[derive(Debug, PartialEq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let mut count = 0;
        let mut from = 0;
        let mut to = 0;
        while let Some(token) = tokens.next() {
            let value = tokens
                .next()
                .ok_or(Error::ParsingError)?
                .parse()
                .or(Err(Error::ParsingError))?;
            match token {
                "move" => count = value,
                "to" => to = value,
                "from" => from = value,
                _ => return Err(Error::ParsingError),
            }
        }
        Ok(Move { count, from, to })
    }
}

#[derive(Debug, PartialEq)]
struct PuzzleInput {
    yard: Yard,
    moves: Vec<Move>,
}

impl PuzzleInput {
    fn tops(&self) -> Vec<char> {
        self.yard.iter().filter_map(|s| s.last().cloned()).collect()
    }
}

impl FromStr for PuzzleInput {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();

        let mut slices = vec![];
        // Collect Stack chunks
        while let Some(&line) = lines.peek() {
            if !line.contains('[') {
                // End of Crates
                break;
            }
            lines.next(); // Consume iteration

            let level: Vec<char> = line
                .chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|c| c.get(1).unwrap_or(&' ').to_owned())
                .collect();
            slices.push(level);
        }

        // Collect Stack Labels
        let stack_labels = lines.next().ok_or(Error::ParsingError)?;
        let stack_count = stack_labels
            .split_whitespace()
            .last()
            .ok_or(Error::ParsingError)?
            .parse()
            .or(Err(Error::ParsingError))?;

        let mut yard = vec![vec![]; stack_count];

        // Put crates in stacks

        for slice in slices.into_iter().rev() {
            for (s, c) in slice.into_iter().enumerate() {
                if c != ' ' {
                    yard[s].push(c);
                }
            }
        }

        // Skip empty line
        lines.next();

        let mut moves = vec![];

        // Collect moves
        for line in lines {
            let m = line.parse()?;
            moves.push(m); // Skip move token
        }

        Ok(PuzzleInput { moves, yard })
    }
}

fn part1(input: &str) -> Result<String, Error> {
    let mut puzzle: PuzzleInput = input.parse()?;
    for Move { count, from, to } in &puzzle.moves {
        for _ in 0..*count {
            if let Some(c) = puzzle.yard[from - 1].pop() {
                puzzle.yard[to - 1].push(c);
            }
        }
    }
    Ok(puzzle.tops().into_iter().collect())
}

fn part2(input: &str) -> Result<String, Error> {
    let mut puzzle: PuzzleInput = input.parse()?;
    for Move { count, from, to } in &puzzle.moves {
        let from_stack = &mut puzzle.yard[from - 1];

        let start = from_stack.len().saturating_sub(*count);
        let mut to_move: Vec<_> = from_stack.drain(start..).collect();
        let to_stack = &mut puzzle.yard[to - 1];
        to_stack.append(&mut to_move);
    }
    Ok(puzzle.tops().into_iter().collect())
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part_1_example() -> Result<(), Error> {
        assert_eq!(part1(EXAMPLE_INPUT)?, "CMZ");
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<(), Error> {
        assert_eq!(part2(EXAMPLE_INPUT)?, "MCD");
        Ok(())
    }

    #[test]
    fn parse_example_input() -> Result<(), Error> {
        let result: PuzzleInput = EXAMPLE_INPUT.parse()?;
        assert_eq!(
            result,
            PuzzleInput {
                yard: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
                moves: vec![
                    Move {
                        count: 1,
                        from: 2,
                        to: 1
                    },
                    Move {
                        count: 3,
                        from: 1,
                        to: 3
                    },
                    Move {
                        count: 2,
                        from: 2,
                        to: 1
                    },
                    Move {
                        count: 1,
                        from: 1,
                        to: 2
                    }
                ]
            }
        );
        Ok(())
    }
}

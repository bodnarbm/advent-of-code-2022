use core::fmt;
use std::{
    io::{self, stdin},
    vec,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, multispace1, newline, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded},
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

#[derive(Debug, PartialEq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq)]
struct PuzzleInput {
    yard: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl PuzzleInput {
    fn tops(&self) -> Vec<char> {
        self.yard.iter().filter_map(|s| s.last().cloned()).collect()
    }
}

fn parse_input(input: &str) -> nom::IResult<&str, PuzzleInput> {
    let (input, crate_slices) =
        separated_list1(newline, separated_list1(tag(" "), parse_crate))(input)?;
    let (input, labels) = preceded(multispace1, separated_list1(space1, digit1))(input)?;

    let total_stacks = labels.len();
    let mut yard = vec![vec![]; total_stacks];
    for cs in crate_slices.into_iter().rev() {
        for (i, c) in cs
            .into_iter()
            .enumerate()
            .flat_map(|(i, o)| o.map(|c| (i, c)))
        {
            yard[i].push(c);
        }
    }
    let (input, moves) = preceded(multispace1, separated_list1(newline, parse_move))(input)?;
    Ok((input, PuzzleInput { moves, yard }))
}

fn parse_crate(input: &str) -> nom::IResult<&str, Option<char>> {
    let (input, c) = alt((
        map(tag("   "), |_| None),
        map(delimited(char('['), anychar, char(']')), Some),
    ))(input)?;
    Ok((input, c))
}

fn parse_move(input: &str) -> nom::IResult<&str, Move> {
    let (input, count) = preceded(tag("move "), map_res(digit1, str::parse))(input)?;
    let (input, from) = preceded(tag(" from "), map_res(digit1, str::parse))(input)?;
    let (input, to) = preceded(tag(" to "), map_res(digit1, str::parse))(input)?;
    Ok((input, Move { count, from, to }))
}

fn part1(input: &str) -> Result<String, Error> {
    let (_, mut puzzle) = parse_input(input).or(Err(Error::ParsingError))?;
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
    let (_, mut puzzle) = parse_input(input).or(Err(Error::ParsingError))?;
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
mod tests {
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
    fn test_parse_move() {
        assert_eq!(
            parse_move("move 1 from 2 to 1"),
            Ok((
                "",
                Move {
                    count: 1,
                    from: 2,
                    to: 1
                }
            ))
        );
    }

    #[test]
    fn test_parse_crate() {
        assert_eq!(parse_crate("   "), Ok(("", None)));
        assert_eq!(parse_crate("[D]"), Ok(("", Some('D'))));
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            Ok((
                "",
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
            ))
        )
    }
}

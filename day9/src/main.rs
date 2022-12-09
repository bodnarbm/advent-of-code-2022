use std::{collections::HashSet, io, str::FromStr};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
enum Error {
    InvalidDirection,
    InvalidMove,
}

#[derive(Debug)]
struct Direction(i64, i64);

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction(0, 1)),
            "R" => Ok(Direction(1, 0)),
            "D" => Ok(Direction(0, -1)),
            "L" => Ok(Direction(-1, 0)),
            _ => Err(Error::InvalidDirection),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    spaces: u32,
}

impl FromStr for Move {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, spaces) = s.split_once(' ').ok_or(Error::InvalidMove)?;
        Ok(Move {
            direction: direction.parse()?,
            spaces: spaces.parse().or(Err(Error::InvalidMove))?,
        })
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

fn count_tail_positions<const LENGTH: usize>(moves: Vec<Move>) -> usize {
    let mut knots = [(0, 0); LENGTH];

    let mut tail_positions = HashSet::new();
    tail_positions.insert(knots[LENGTH - 1]);

    for Move { spaces, direction } in moves {
        for _ in 0..spaces {
            let (head, rest) = knots.split_at_mut(1);
            let mut prior = &mut head[0];
            prior.0 += direction.0;
            prior.1 += direction.1;

            for knot in rest.iter_mut() {
                let diff = ((prior.0 - knot.0), (prior.1 - knot.1));
                if diff.0.abs() > 1 || diff.1.abs() > 1 {
                    knot.0 += diff.0.clamp(-1, 1);
                    knot.1 += diff.1.clamp(-1, 1);
                }
                prior = knot;
            }

            tail_positions.insert(knots[LENGTH - 1]);
        }
    }

    tail_positions.len()
}

fn part1(input: &str) -> usize {
    let moves = parse_input(input);
    count_tail_positions::<2>(moves)
}

fn part2(input: &str) -> usize {
    let moves = parse_input(input);
    count_tail_positions::<10>(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const LARGER_EXAMPLE_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 1);
        assert_eq!(part2(LARGER_EXAMPLE_INPUT), 36);
    }
}

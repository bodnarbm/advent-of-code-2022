use std::{
    collections::HashSet,
    fmt::Debug,
    io,
    ops::{AddAssign, Sub},
    str::FromStr,
};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector2(i64, i64);

impl Vector2 {
    fn max_magnitude(&self) -> i64 {
        self.0.abs().max(self.1.abs())
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Clone)]
struct Direction(Vector2);

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction(Vector2(0, 1))),
            "R" => Ok(Direction(Vector2(1, 0))),
            "D" => Ok(Direction(Vector2(0, -1))),
            "L" => Ok(Direction(Vector2(-1, 0))),
            _ => Err(Error::InvalidDirection),
        }
    }
}

struct Steps(Vec<Direction>);

impl FromStr for Steps {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, spaces) = s.split_once(' ').ok_or(Error::InvalidMove)?;
        let direction: Direction = direction.parse()?;
        let spaces: usize = spaces.parse().or(Err(Error::InvalidMove))?;
        Ok(Steps(vec![direction; spaces]))
    }
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(str::parse::<Steps>)
        .map(Result::unwrap)
        .flat_map(|s| s.0)
        .collect()
}

fn count_tail_positions<const LENGTH: usize>(moves: Vec<Direction>) -> usize {
    let mut knots = [Vector2(0, 0); LENGTH];

    let mut tail_positions = HashSet::new();
    tail_positions.insert(knots[LENGTH - 1]);

    for direction in moves {
        let (head, rest) = knots.split_at_mut(1);
        let mut prior = &mut head[0];
        *prior += direction.0;

        for knot in rest.iter_mut() {
            let delta = *prior - *knot;
            if delta.max_magnitude() > 1 {
                knot.0 += delta.0.clamp(-1, 1);
                knot.1 += delta.1.clamp(-1, 1);
                dbg!(&knot);
            }
            prior = knot;
        }

        tail_positions.insert(knots[LENGTH - 1]);
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

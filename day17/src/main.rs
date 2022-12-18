use std::{fmt::Display, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, PartialEq, Clone)]
enum Movement {
    Left,
    Right,
    Down,
}

fn parse_input(input: &str) -> Vec<Movement> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Movement::Right,
            '<' => Movement::Left,
            _ => unreachable!("invalid character in input"),
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum RockShape {
    HorizontalLine,
    Cross,
    Corner,
    VerticalLine,
    Square,
}

impl From<u8> for RockShape {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::HorizontalLine,
            1 => Self::Cross,
            2 => Self::Corner,
            3 => Self::VerticalLine,
            4 => Self::Square,
            _ => unimplemented!("{} not a valid value for a Rock Shape", value),
        }
    }
}

struct FallingRock {
    position: (usize, usize),
    shape: RockShape,
}

impl FallingRock {
    fn new(position: (usize, usize), shape: RockShape) -> Self {
        Self { position, shape }
    }

    fn pieces(&self) -> Vec<(usize, usize)> {
        match self.shape {
            RockShape::HorizontalLine => (self.position.0..self.position.0 + 4)
                .map(|x| (x, self.position.1))
                .collect(),
            RockShape::Cross => vec![
                (self.position.0, self.position.1 + 1),
                (self.position.0 + 1, self.position.1),
                (self.position.0 + 1, self.position.1 + 1),
                (self.position.0 + 1, self.position.1 + 2),
                (self.position.0 + 2, self.position.1 + 1),
            ],
            RockShape::Corner => vec![
                (self.position.0, self.position.1),
                (self.position.0 + 1, self.position.1),
                (self.position.0 + 2, self.position.1),
                (self.position.0 + 2, self.position.1 + 1),
                (self.position.0 + 2, self.position.1 + 2),
            ],
            RockShape::VerticalLine => vec![
                (self.position.0, self.position.1),
                (self.position.0, self.position.1 + 1),
                (self.position.0, self.position.1 + 2),
                (self.position.0, self.position.1 + 3),
            ],
            RockShape::Square => vec![
                (self.position.0, self.position.1),
                (self.position.0, self.position.1 + 1),
                (self.position.0 + 1, self.position.1),
                (self.position.0 + 1, self.position.1 + 1),
            ],
        }
    }

    fn shift(&self, movement: Movement) -> Option<Self> {
        // Check for would move out of bounds
        match movement {
            Movement::Right => {
                if self.position.0 == 6 {
                    return None;
                }
            }
            Movement::Left => {
                if self.position.0 == 0 {
                    return None;
                }
            }
            Movement::Down => {
                if self.position.1 == 0 {
                    return None;
                }
            }
        }
        let new_position = match movement {
            Movement::Right => (self.position.0 + 1, self.position.1),
            Movement::Left => (self.position.0 - 1, self.position.1),
            Movement::Down => (self.position.0, self.position.1 - 1),
        };
        Some(Self::new(new_position, self.shape))
    }
}

struct Chamber {
    falling_rock: Option<FallingRock>,
    slices: Vec<[char; 7]>,
    next_shape: RockShape,
}

impl Chamber {
    fn new() -> Self {
        Self {
            slices: vec![],
            falling_rock: None,
            next_shape: RockShape::HorizontalLine,
        }
    }

    fn generate_rock(&mut self) {
        self.falling_rock = Some(FallingRock::new(
            (2, 3 + self.slices.len()),
            self.next_shape,
        ));
        self.next_shape = RockShape::from((self.next_shape as u8 + 1) % 5);
    }

    fn collides(&self, piece: &(usize, usize)) -> bool {
        let (x, y) = piece;
        if *x >= 7 {
            return true;
        }
        if *y >= self.slices.len() {
            return false;
        }
        self.slices[*y][*x] != '.'
    }

    fn move_rock(&mut self, movement: Movement) -> bool {
        let Some(falling_rock) = &self.falling_rock else {
            unimplemented!("falling rock must be generated before movements");
        };

        let Some(candidate) = falling_rock.shift(movement) else {
            return false
        };
        let collides = candidate.pieces().iter().any(|piece| self.collides(piece));

        if !collides {
            self.falling_rock = Some(candidate)
        }
        !collides
    }

    fn freeze_rock(&mut self) {
        let Some(falling_rock) = self.falling_rock.take() else {
            unimplemented!("no falling rock to freeze")
        };
        for (x, y) in falling_rock.pieces() {
            if y + 1 > self.slices.len() {
                self.slices
                    .extend(vec![['.'; 7]; y + 1 - self.slices.len()]);
            }
            self.slices[y][x] = '#';
        }
    }

    fn height(&self) -> usize {
        self.slices.len()
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cells = self.slices.clone();

        if let Some(falling_rock) = &self.falling_rock {
            for (x, y) in falling_rock.pieces() {
                if y + 1 > cells.len() {
                    cells.extend(vec![['.'; 7]; y + 1 - cells.len()])
                }
                cells[y][x] = '@';
            }
        }

        for row in cells.iter().rev() {
            writeln!(f, "|{}|", String::from_iter(row))?
        }

        write!(f, "+-------+")
    }
}

fn part1(input: &str) -> usize {
    let mut jet_pattern = parse_input(input).into_iter().cycle();

    let mut chamber = Chamber::new();

    for _i in 0..2022 {
        chamber.generate_rock();
        for movement in jet_pattern.by_ref() {
            chamber.move_rock(movement);
            if !chamber.move_rock(Movement::Down) {
                chamber.freeze_rock();
                break;
            };
        }
    }

    chamber.height()
}

fn part2(_input: &str) -> usize {
    todo!("part 2 not started")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 3068);
    }

    #[test]
    #[ignore = "part 2 not started"]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 0);
    }
}

use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, PartialEq)]
enum Obstruction {
    Wall(usize, (usize, usize)),
    Floor((usize, usize), usize),
}

impl Obstruction {
    fn bounds(&self) -> ((usize, usize), (usize, usize)) {
        match self {
            Obstruction::Wall(x, (min_y, max_y)) => ((*x, *x), (*min_y, *max_y)),
            Obstruction::Floor((min_x, max_x), y) => ((*min_x, *max_x), (*y, *y)),
        }
    }

    fn collides(&self, pos: (usize, usize)) -> bool {
        let ((x_min, x_max), (y_min, y_max)) = self.bounds();
        pos.0 >= x_min && pos.0 <= x_max && pos.1 >= y_min && pos.1 <= y_max
    }
}

#[derive(Debug, PartialEq)]
struct RockFormation(Vec<Obstruction>);

impl RockFormation {
    fn bounds(&self) -> ((usize, usize), (usize, usize)) {
        let mut bounds = self.0.iter().map(|obstruction| obstruction.bounds());
        let ((mut min_x, mut max_x), (mut min_y, mut max_y)) =
            bounds.next().unwrap_or(((0, 0), (0, 0)));

        for ((x_min, x_max), (y_min, y_max)) in bounds {
            min_x = min_x.min(x_min);
            max_x = max_x.max(x_max);
            min_y = min_y.min(y_min);
            max_y = max_y.max(y_max);
        }

        ((min_x, max_x), (min_y, max_y))
    }

    fn collides(&self, pos: (usize, usize)) -> bool {
        self.0.iter().any(|obstruction| obstruction.collides(pos))
    }
}

struct World {
    rock_formations: Vec<RockFormation>,
    bounds: ((usize, usize), (usize, usize)),
    sand: Vec<(usize, usize)>,
    has_world_floor: bool,
    stack: Vec<(usize, usize)>,
}

impl World {
    fn new(rock_formations: Vec<RockFormation>, has_world_floor: bool) -> Self {
        let min_x = rock_formations
            .iter()
            .map(|rock_formation| rock_formation.bounds().0 .0)
            .min()
            .unwrap();
        let max_x = rock_formations
            .iter()
            .map(|rock_formation| rock_formation.bounds().0 .1)
            .max()
            .unwrap();

        let max_y = rock_formations
            .iter()
            .map(|rock_formation| rock_formation.bounds().1 .1)
            .max()
            .unwrap();

        Self {
            rock_formations,
            bounds: ((min_x, max_x), (0, max_y)),
            sand: vec![],
            has_world_floor,
            stack: vec![(500, 0)],
        }
    }

    fn collides(&self, pos: (usize, usize)) -> bool {
        (self.has_world_floor && pos.1 >= self.bounds.1 .1 + 2)
            || self.sand.contains(&pos)
            || self
                .rock_formations
                .iter()
                .any(|rock_formation| rock_formation.collides(pos))
    }

    fn free_spot(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let down = (pos.0, pos.1 + 1);
        let left_down = (pos.0 - 1, pos.1 + 1);
        let right_down = (pos.0 + 1, pos.1 + 1);
        if !self.collides(down) {
            Some(down)
        } else if !self.collides(left_down) {
            Some(left_down)
        } else if !self.collides(right_down) {
            Some(right_down)
        } else {
            None
        }
    }

    fn drop(&mut self) -> bool {
        let Some(mut grain) = self.stack.pop() else { 
            return false;
        };

        while let Some(free_spot) = self.free_spot(grain) {
            self.stack.push(grain);
            grain = free_spot;
            if !self.has_world_floor && grain.1 > self.bounds.1 .1 {
                return false;
            }
        }

        self.sand.push(grain);
        grain != (500, 0)
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ((min_x, max_x), (min_y, max_y)) = self.bounds;
        let min_x = min_x.min(
            self.sand
                .iter()
                .map(|(x, _)| *x)
                .min()
                .unwrap_or(usize::MAX),
        ) - 2;
        let max_x = max_x.max(
            self.sand
                .iter()
                .map(|(x, _)| *x)
                .max()
                .unwrap_or(usize::MIN),
        ) + 2;
        let rows = max_y - min_y + 1 + (self.has_world_floor as usize * 2);
        let cols = max_x - min_x + 1;

        let mut cells = vec![vec!['.'; cols]; rows];

        let sand_source = (500 - min_x, 0);
        cells[sand_source.1][sand_source.0] = '+';

        if self.has_world_floor {
            for x in 0..cols {
                cells[rows - 1][x] = '#';
            }
        }

        for rock_formation in &self.rock_formations {
            for obstruction in &rock_formation.0 {
                let ((x_min, x_max), (y_min, y_max)) = obstruction.bounds();
                for x in x_min..=x_max {
                    for y in y_min..=y_max {
                        cells[y - min_y][x - min_x] = '#';
                    }
                }
            }
        }

        for sand in &self.sand {
            cells[sand.1 - min_y][sand.0 - min_x] = 'o';
        }

        writeln!(
            f,
            "{:5}{:indent$}{:trail$}",
            min_x / 100,
            5,
            max_x / 100,
            indent = 500 - min_x,
            trail = max_x - 500
        )?;
        writeln!(
            f,
            "{:5}{:indent$}{:trail$}",
            min_x % 100 / 10,
            0,
            max_x % 100 / 10,
            indent = 500 - min_x,
            trail = max_x - 500
        )?;
        writeln!(
            f,
            "{:5}{:indent$}{:trail$}",
            min_x % 10,
            0,
            max_x % 10,
            indent = 500 - min_x,
            trail = max_x - 500
        )?;

        for (r, row) in cells.iter().enumerate() {
            write!(f, "{:3} ", r)?;
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

mod parsers {
    use core::panic;

    use nom::{
        bytes::complete::tag,
        character::complete::{char, newline, u32},
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    use crate::{Obstruction, RockFormation};

    fn rock_formation(input: &str) -> IResult<&str, RockFormation> {
        let (input, points) =
            separated_list1(tag(" -> "), separated_pair(u32, char(','), u32))(input)?;
        let mut iter = points.iter();
        let Some(mut first_point) = iter.next() else {
            panic!("No points in rock formation");
        };
        let mut obstructions = Vec::with_capacity(points.len() - 1);
        for second_point in iter {
            if first_point.0 == second_point.0 {
                let min_height = first_point.1.min(second_point.1) as usize;
                let max_height = first_point.1.max(second_point.1) as usize;
                obstructions.push(Obstruction::Wall(
                    first_point.0 as usize,
                    (min_height, max_height),
                ));
            } else {
                let start = first_point.0.min(second_point.0) as usize;
                let end = first_point.0.max(second_point.0) as usize;
                obstructions.push(Obstruction::Floor((start, end), first_point.1 as usize));
            }
            first_point = second_point;
        }
        Ok((input, RockFormation(obstructions)))
    }

    pub(super) fn parse_input(input: &str) -> IResult<&str, Vec<RockFormation>> {
        separated_list1(newline, rock_formation)(input)
    }
}

fn part1(input: &str) -> usize {
    let (_, rock_formations) = parsers::parse_input(input).unwrap();
    let mut world = World::new(rock_formations, false);

    while world.drop() {}

    println!("{}", world);
    world.sand.len()
}

fn part2(input: &str) -> usize {
    let (_, rock_formations) = parsers::parse_input(input).unwrap();
    let mut world = World::new(rock_formations, true);

    while world.drop() {}

    println!("{}", world);
    world.sand.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 24);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 93);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parsers::parse_input(EXAMPLE_INPUT),
            Ok((
                "",
                vec![
                    RockFormation(vec![
                        Obstruction::Wall(498, (4, 6)),
                        Obstruction::Floor((496, 498), 6),
                    ]),
                    RockFormation(vec![
                        Obstruction::Floor((502, 503), 4),
                        Obstruction::Wall(502, (4, 9)),
                        Obstruction::Floor((494, 502), 9)
                    ])
                ]
            ))
        )
    }
}

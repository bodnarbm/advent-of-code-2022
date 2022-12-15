use std::{io, ops::RangeInclusive};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input, 2000000));
    println!("Part 2: {}", part2(&input, (0..=4000000, 0..=4000000)));
}

#[derive(Debug)]
struct Reading {
    sensor: (isize, isize),
    beacon: (isize, isize),
}

impl Reading {
    fn new(sensor: (isize, isize), beacon: (isize, isize)) -> Self {
        Self { sensor, beacon }
    }

    fn covers_at_row(&self, row: isize) -> Option<(isize, isize)> {
        let manhatten_distance =
            self.beacon.0.abs_diff(self.sensor.0) + self.beacon.1.abs_diff(self.sensor.1);

        let vertical_travel = self.sensor.1.abs_diff(row);
        if vertical_travel > manhatten_distance {
            return None;
        }

        let horizontal_slack = manhatten_distance - vertical_travel;
        Some((
            self.sensor.0 - horizontal_slack as isize,
            self.sensor.0 + horizontal_slack as isize,
        ))
    }
}

mod parsers {
    use nom::{
        bytes::complete::tag,
        character::complete::{self, newline},
        multi::separated_list1,
        sequence::{preceded, separated_pair},
        IResult, Parser,
    };

    use crate::Reading;

    fn reading(input: &str) -> IResult<&str, Reading> {
        let (input, sensor) = preceded(
            tag("Sensor at "),
            separated_pair(
                preceded(tag("x="), complete::i64.map(|x| x as isize)),
                tag(", "),
                preceded(tag("y="), complete::i64.map(|x| x as isize)),
            ),
        )(input)?;
        let (input, beacon) = preceded(
            tag(": closest beacon is at "),
            separated_pair(
                preceded(tag("x="), complete::i64.map(|x| x as isize)),
                tag(", "),
                preceded(tag("y="), complete::i64.map(|x| x as isize)),
            ),
        )(input)?;
        Ok((input, Reading::new(sensor, beacon)))
    }

    pub(super) fn parse_input(input: &str) -> IResult<&str, Vec<Reading>> {
        separated_list1(newline, reading)(input)
    }
}

fn part1(input: &str, row: isize) -> usize {
    let (_, readings) = parsers::parse_input(input).unwrap();
    let mut covers = readings
        .iter()
        .flat_map(|reading| reading.covers_at_row(row))
        .collect::<Vec<_>>();
    covers.sort();

    let mut iter = covers.into_iter();
    let Some(first) = iter.next() else {
        return 0;
    };

    let covers = iter.fold(vec![first], |mut covers, cover| {
        let mut last = covers.last_mut().unwrap();
        if last.1 >= cover.0 && last.0 <= cover.1 {
            last.0 = last.0.min(cover.0);
            last.1 = last.1.max(cover.1);
        } else {
            covers.push(cover);
        }
        covers
    });

    covers
        .into_iter()
        .map(|(start, end)| (end - start) as usize)
        .sum()
}

fn part2(input: &str, bounds: (RangeInclusive<isize>, RangeInclusive<isize>)) -> isize {
    let (_, readings) = parsers::parse_input(input).unwrap();
    let rows = bounds.1;
    let x_start = *bounds.0.start();
    let x_end = *bounds.0.end();
    for y in rows {
        let mut covers = readings
            .iter()
            .flat_map(|reading| reading.covers_at_row(y))
            .map(|(start, end)| (start.max(x_start), end.min(x_end)))
            .filter(|(start, end)| *end >= x_start && *start <= x_end)
            .collect::<Vec<_>>();
        covers.sort();

        let mut iter = covers.into_iter();
        let Some(first) = iter.next() else {
                return 0;
            };

        let covers = iter.fold(vec![first], |mut covers, cover| {
            let mut last = covers.last_mut().unwrap();
            if last.1 + 1 >= cover.0 && last.0 <= cover.1 {
                last.0 = last.0.min(cover.0);
                last.1 = last.1.max(cover.1);
            } else {
                covers.push(cover);
            }
            covers
        });

        match covers.len() {
            1 => {
                if covers[0].0 == x_start && covers[0].1 == x_end {
                    continue;
                }
                let x = if covers[0].0 == x_start {
                    x_end
                } else {
                    x_start
                };
                return x * 4000000 + y;
            }
            _ => {
                let x = covers[1].0 - 1;
                return x * 4000000 + y;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT, 10), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT, (0..=20, 0..=20)), 56000011);
    }
}

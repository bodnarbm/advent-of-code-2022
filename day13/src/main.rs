use std::{cmp::Ordering, io};

use parsers::parse_input;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq)]
enum Element {
    Integer(u64),
    List(Vec<Element>),
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Element::Integer(s), Element::Integer(o)) => s.cmp(o),
            (Element::List(s), Element::List(o)) => s.cmp(o),
            (Element::Integer(s), Element::List(_)) => {
                Element::List(vec![Element::Integer(*s)]).cmp(other)
            }
            (Element::List(_), Element::Integer(o)) => {
                self.cmp(&Element::List(vec![Element::Integer(*o)]))
            }
        }
    }
}

mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{self, newline},
        combinator::map,
        multi::{separated_list0, separated_list1},
        sequence::{delimited, separated_pair},
        IResult, Parser,
    };

    use super::Element;

    pub(super) fn parse_list(input: &str) -> IResult<&str, Element> {
        map(
            delimited(
                tag("["),
                separated_list0(
                    tag(","),
                    alt((complete::u64.map(Element::Integer), parse_list)),
                ),
                tag("]"),
            ),
            Element::List,
        )(input)
    }

    pub(super) fn parse_input(input: &str) -> IResult<&str, Vec<(Element, Element)>> {
        separated_list1(tag("\n\n"), separated_pair(parse_list, newline, parse_list))(input)
    }
}

fn part1(input: &str) -> u64 {
    let (_, pairs) = parse_input(input).unwrap();
    pairs
        .into_iter()
        .enumerate()
        .filter(|(_, pair)| pair.0 < pair.1)
        .map(|(idx, _)| idx as u64 + 1)
        .sum()
}

fn part2(input: &str) -> u64 {
    let (_, pairs) = parse_input(input).unwrap();
    let divider_packets = vec![
        Element::List(vec![Element::List(vec![Element::Integer(2)])]),
        Element::List(vec![Element::List(vec![Element::Integer(6)])]),
    ];
    let mut packets: Vec<&Element> = pairs
        .iter()
        .flat_map(|(first, second)| vec![first, second])
        .chain(&divider_packets)
        .collect();

    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter_map(|(idx, packet)| {
            if divider_packets.contains(packet) {
                Some(idx as u64 + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 140);
    }
}

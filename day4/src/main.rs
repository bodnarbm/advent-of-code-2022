use std::{
    io::{self, stdin},
    str::FromStr,
};

fn main() -> io::Result<()> {
    let input = io::read_to_string(stdin())?;
    match part_1(&input) {
        Ok(result) => println!("Part 1: {}", result),
        Err(err) => eprintln!("Part 1: ERROR {}", err),
    }
    match part_2(&input) {
        Ok(result) => println!("Part 2: {}", result),
        Err(err) => eprintln!("Part 2: ERROR {}", err),
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    SectionOrderInvalid { start: u32, end: u32 },
    SectionRangeMissing,
    SectionNumberInvalid,
    AssignmentMissing,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ParseError::SectionOrderInvalid {
                start: first,
                end: second,
            } => format!(
                "section order is invalid, {} cannot be before {}",
                first, second
            ),
            ParseError::SectionRangeMissing => {
                "must supply two sections in assignment, seperated by '-'".to_string()
            }
            ParseError::SectionNumberInvalid => "section is not a valid number".to_string(),
            ParseError::AssignmentMissing => {
                "must supply two assignments, seperated by ','".to_string()
            }
        };
        write!(f, "parse failure: {}", msg)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, PartialEq)]
struct Assignment {
    start: u32,
    end: u32,
}

impl FromStr for Assignment {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(ParseError::SectionRangeMissing)?;
        let start = start.parse().or(Err(ParseError::SectionNumberInvalid))?;
        let end = end.parse().or(Err(ParseError::SectionNumberInvalid))?;
        if start > end {
            return Err(ParseError::SectionOrderInvalid { start, end });
        }
        Ok(Assignment { start, end })
    }
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        (self.start <= other.start && self.end >= other.start)
            || (other.start <= self.start && other.end >= self.start)
    }
}

#[derive(Debug, PartialEq)]
struct AssignmentPair {
    first: Assignment,
    second: Assignment,
}

impl FromStr for AssignmentPair {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(',').ok_or(ParseError::AssignmentMissing)?;
        let first = first.parse()?;
        let second = second.parse()?;
        Ok(AssignmentPair { first, second })
    }
}

fn part_1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut count = 0;
    for line in input.lines() {
        let pair: AssignmentPair = line.parse()?;
        count += (pair.first.contains(&pair.second) || pair.second.contains(&pair.first)) as u32
    }
    Ok(count)
}

fn part_2(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut count = 0;
    for line in input.lines() {
        let pair: AssignmentPair = line.parse()?;
        count += (pair.first.overlaps(&pair.second)) as u32
    }
    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part_1(EXAMPLE_INPUT)?, 2);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part_2(EXAMPLE_INPUT)?, 4);
        Ok(())
    }

    #[test]
    fn assignment_from_str() {
        assert_eq!("2-4".parse(), Ok(Assignment { start: 2, end: 4 }));
        assert_eq!(
            "10-100".parse(),
            Ok(Assignment {
                start: 10,
                end: 100
            })
        );
        assert_eq!(
            "4".parse::<Assignment>(),
            Err(ParseError::SectionRangeMissing)
        );
        assert_eq!(
            "a-b".parse::<Assignment>(),
            Err(ParseError::SectionNumberInvalid)
        );
        assert_eq!(
            "4-2".parse::<Assignment>(),
            Err(ParseError::SectionOrderInvalid { start: 4, end: 2 })
        );
    }

    #[test]
    fn assignment_pair_from_str() {
        assert_eq!(
            "2-4,6-8".parse(),
            Ok(AssignmentPair {
                first: Assignment { start: 2, end: 4 },
                second: Assignment { start: 6, end: 8 }
            })
        );
        assert_eq!(
            "10-100,6-8".parse(),
            Ok(AssignmentPair {
                first: Assignment {
                    start: 10,
                    end: 100
                },
                second: Assignment { start: 6, end: 8 }
            })
        );
        assert_eq!(
            "10-100".parse::<AssignmentPair>(),
            Err(ParseError::AssignmentMissing)
        );
        assert_eq!(
            "10-100,a-b".parse::<AssignmentPair>(),
            Err(ParseError::SectionNumberInvalid)
        );
    }

    #[test]
    fn test_overlaps() {
        assert!(!Assignment { start: 0, end: 1 }.overlaps(&Assignment { start: 2, end: 3 }));
        assert!(!Assignment { start: 2, end: 3 }.overlaps(&Assignment { start: 0, end: 1 }));

        assert!(Assignment { start: 0, end: 2 }.overlaps(&Assignment { start: 2, end: 3 }));
        assert!(Assignment { start: 2, end: 3 }.overlaps(&Assignment { start: 0, end: 2 }));

        assert!(Assignment { start: 0, end: 3 }.overlaps(&Assignment { start: 2, end: 3 }));
        assert!(Assignment { start: 2, end: 3 }.overlaps(&Assignment { start: 0, end: 3 }));
    }
}

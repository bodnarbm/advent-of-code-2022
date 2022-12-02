use std::io::{self, stdin};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn defeats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Self::Scissors => Self::Rock,
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum ParseInputError {
    InvalidPlay,
    InvalidRound,
}

impl TryFrom<char> for Shape {
    type Error = ParseInputError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err(ParseInputError::InvalidPlay),
        }
    }
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, PartialEq)]
struct Round {
    opponent: Shape,
    me: Shape,
}

impl Round {
    fn score(&self) -> u32 {
        let shape_points = self.me.score();
        let result_points = match self.result() {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        };
        shape_points + result_points
    }

    fn result(&self) -> RoundResult {
        if self.me == self.opponent {
            RoundResult::Draw
        } else if self.me.defeats() == self.opponent {
            RoundResult::Win
        } else {
            RoundResult::Lose
        }
    }
}

enum Strategy {
    Part1,
    Part2,
}

impl Round {
    fn from_str(s: &str, strategy: &Strategy) -> Result<Self, ParseInputError> {
        let opponent_play = s.chars().nth(0).ok_or(ParseInputError::InvalidRound)?;
        let my_play = s.chars().nth(2).ok_or(ParseInputError::InvalidRound)?;

        let opponent: Shape = opponent_play.try_into()?;
        // TODO: Clean up with strategy pattern for enums?
        let me = match strategy {
            Strategy::Part1 => my_play.try_into()?,
            Strategy::Part2 => match my_play {
                'X' => opponent.defeats(),
                'Y' => opponent.clone(),
                'Z' => opponent.loses_to(),
                _ => return Err(ParseInputError::InvalidPlay),
            },
        };

        Ok(Round { opponent, me })
    }
}

fn score(input: &str, strategy: &Strategy) -> Result<u32, ParseInputError> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Round::from_str(line, strategy))
        // TODO: There has to be a cleaner way to do this than a map within a map.
        .map(|r| r.map(|round| round.score()))
        .sum()
}

fn main() -> io::Result<()> {
    let input = io::read_to_string(stdin())?;
    let part_1 = score(&input, &Strategy::Part1).expect("could not score part 1");
    println!("Part 1: {part_1}");
    let part_2 = score(&input, &Strategy::Part2).expect("could not score part 2");
    println!("Part 2: {part_2}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample_input_1() -> Result<(), ParseInputError> {
        let input = r#"
A Y
B X
C Z
"#;
        assert_eq!(score(input, &Strategy::Part1)?, 15);
        Ok(())
    }

    #[test]
    fn sample_input_2() -> Result<(), ParseInputError> {
        let input = r#"
A Y
B X
C Z
"#;
        assert_eq!(score(input, &Strategy::Part2)?, 12);
        Ok(())
    }

    #[test]
    fn line_to_round_part_1() -> Result<(), ParseInputError> {
        let line = "A Y";
        let expected = Round {
            opponent: Shape::Rock,
            me: Shape::Paper,
        };
        assert_eq!(Round::from_str(line, &Strategy::Part1)?, expected);
        Ok(())
    }

    #[test]
    fn line_to_round_part_2() -> Result<(), ParseInputError> {
        let line = "A Y";
        let expected = Round {
            opponent: Shape::Rock,
            me: Shape::Rock,
        };
        assert_eq!(Round::from_str(line, &Strategy::Part2)?, expected);
        Ok(())
    }

    #[test]
    fn score_winning_round() {
        let round = Round {
            opponent: Shape::Rock,
            me: Shape::Paper,
        };
        assert_eq!(round.score(), 8)
    }

    #[test]
    fn score_lossing_round() {
        let round = Round {
            opponent: Shape::Paper,
            me: Shape::Rock,
        };
        assert_eq!(round.score(), 1)
    }

    #[test]
    fn score_draw_round() {
        let round = Round {
            opponent: Shape::Scissors,
            me: Shape::Scissors,
        };
        assert_eq!(round.score(), 6)
    }
}

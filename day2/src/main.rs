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
enum Day2Error {
    Code,
    Play,
    Round,
}

impl TryFrom<char> for Shape {
    type Error = Day2Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err(Day2Error::Play),
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

trait PlayStrategy {
    fn play(&self, code: char, other: &Shape) -> Result<Shape, Day2Error>;
}

struct Part1Strategy;
impl PlayStrategy for Part1Strategy {
    fn play(&self, code: char, _other: &Shape) -> Result<Shape, Day2Error> {
        code.try_into()
    }
}

struct Part2Strategy;
impl PlayStrategy for Part2Strategy {
    fn play(&self, code: char, other: &Shape) -> Result<Shape, Day2Error> {
        match code {
            'X' => Ok(other.defeats()),
            'Y' => Ok(other.clone()),
            'Z' => Ok(other.loses_to()),
            _ => Err(Day2Error::Code),
        }
    }
}

impl Round {
    fn from_line<S: PlayStrategy>(s: &str, strategy: &S) -> Result<Self, Day2Error> {
        let mut chars = s.chars().take(3).step_by(2);
        let Some(opponent_play) = chars.next() else {
            return Err(Day2Error::Round);
        };
        let Some(my_play) = chars.next() else {
            return Err(Day2Error::Round);
        };

        let opponent = opponent_play.try_into()?;
        let me = strategy.play(my_play, &opponent)?;

        Ok(Round { opponent, me })
    }
}

fn score<S: PlayStrategy>(input: &str, strategy: &S) -> Result<u32, Day2Error> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Round::from_line(line, strategy))
        // TODO: There has to be a cleaner way to do this than a map within a map.
        .map(|result| result.map(|round| round.score()))
        .sum()
}

fn main() -> io::Result<()> {
    let input = io::read_to_string(stdin())?;
    let part_1 = score(&input, &Part1Strategy).expect("could not score part 1");
    println!("Part 1: {part_1}");
    let part_2 = score(&input, &Part2Strategy).expect("could not score part 2");
    println!("Part 2: {part_2}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample_input_1() -> Result<(), Day2Error> {
        let input = r#"
A Y
B X
C Z
"#;
        assert_eq!(score(input, &Part1Strategy)?, 15);
        Ok(())
    }

    #[test]
    fn sample_input_2() -> Result<(), Day2Error> {
        let input = r#"
A Y
B X
C Z
"#;
        assert_eq!(score(input, &Part2Strategy)?, 12);
        Ok(())
    }

    #[test]
    fn line_to_round_part_1() -> Result<(), Day2Error> {
        let line = "A Y";
        let expected = Round {
            opponent: Shape::Rock,
            me: Shape::Paper,
        };
        assert_eq!(Round::from_line(line, &Part1Strategy)?, expected);
        Ok(())
    }

    #[test]
    fn line_to_round_part_2() -> Result<(), Day2Error> {
        let line = "A Y";
        let expected = Round {
            opponent: Shape::Rock,
            me: Shape::Rock,
        };
        assert_eq!(Round::from_line(line, &Part2Strategy)?, expected);
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

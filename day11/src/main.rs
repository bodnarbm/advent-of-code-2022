use std::{io, ops::Rem, u128};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
pub(crate) enum Operand {
    Literal(u8),
    Old,
}

#[derive(Debug)]
pub(crate) enum Operator {
    Plus,
    Multiply,
}

#[derive(Debug)]
pub(crate) struct Operation {
    operator: Operator,
    lhs: Operand,
    rhs: Operand,
}

impl Operation {
    fn evaluate(&self, old: u128) -> u128 {
        let lhs = match self.lhs {
            Operand::Old => old,
            Operand::Literal(v) => v as u128,
        };
        let rhs = match self.rhs {
            Operand::Old => old,
            Operand::Literal(v) => v as u128,
        };

        match self.operator {
            Operator::Plus => lhs + rhs,
            Operator::Multiply => lhs * rhs,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Test {
    divisible_by: u8,
    targets: (usize, usize),
}

#[derive(Debug)]
pub(crate) struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn inspect_items(
        &mut self,
        boredum_fn: &dyn Fn(u128) -> u128,
        rescaler: u128,
    ) -> Vec<(usize, u128)> {
        self.items
            .drain(0..)
            .map(|old| self.operation.evaluate(old))
            .map(boredum_fn)
            .map(|w| w % rescaler)
            .map(|worry_level| {
                let target = if worry_level.rem(self.test.divisible_by as u128) == 0 {
                    self.test.targets.1
                } else {
                    self.test.targets.0
                };
                (target, worry_level)
            })
            .collect()
    }
}

fn part1(input: &str) -> usize {
    let (_, mut monkeys) = parsers::parse(input).unwrap();

    let rounds = 20;
    let mut inspections = vec![0; monkeys.len()];
    let rescaler = monkeys
        .iter()
        .map(|m| m.test.divisible_by as u128)
        .product();

    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            let thrown_items = monkeys[idx].inspect_items(&|w| w / 3, rescaler);
            inspections[idx] += thrown_items.len();
            for (idx, item) in thrown_items {
                monkeys[idx].items.push(item);
            }
        }
    }

    inspections.sort();

    inspections.into_iter().rev().take(2).product()
}

fn part2(input: &str) -> usize {
    let (_, mut monkeys) = parsers::parse(input).unwrap();

    let rounds = 10000;

    let mut inspections = vec![0; monkeys.len()];
    let rescaler = monkeys
        .iter()
        .map(|m| m.test.divisible_by as u128)
        .product();

    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            let thrown_items = monkeys[idx].inspect_items(&|w| w, rescaler);
            inspections[idx] += thrown_items.len();
            for (idx, item) in thrown_items {
                monkeys[idx].items.push(item);
            }
        }
    }

    inspections.sort();

    inspections.into_iter().rev().take(2).product()
}

mod parsers {
    use nom::{
        branch::alt,
        bytes::streaming::tag,
        character::complete::{self, digit1, multispace1, newline},
        multi::{count, separated_list1},
        sequence::{preceded, tuple},
        IResult, Parser,
    };

    use crate::{Monkey, Operand, Operation, Operator, Test};

    fn start_items(input: &str) -> IResult<&str, Vec<u128>> {
        preceded(
            tag("Starting items: "),
            separated_list1(tag(", "), complete::u128),
        )(input)
    }

    fn operand(input: &str) -> IResult<&str, Operand> {
        alt((
            tag("old").map(|_| Operand::Old),
            complete::u8.map(Operand::Literal),
        ))(input)
    }

    fn operator(input: &str) -> IResult<&str, Operator> {
        alt((
            tag(" + ").map(|_| Operator::Plus),
            tag(" * ").map(|_| Operator::Multiply),
        ))(input)
    }

    fn operation(input: &str) -> IResult<&str, Operation> {
        let (input, (lhs, operator, rhs)) = preceded(
            tag("Operation: new = "),
            tuple((operand, operator, operand)),
        )(input)?;

        Ok((input, Operation { lhs, operator, rhs }))
    }

    fn test(input: &str) -> IResult<&str, Test> {
        let (input, divisible_by) = preceded(tag("Test: divisible by "), complete::u8)(input)?;

        let (input, true_outcome) =
            preceded(tag("\n    If true: throw to monkey "), complete::u8)(input)?;
        let (input, false_outcome) =
            preceded(tag("\n    If false: throw to monkey "), complete::u8)(input)?;
        Ok((
            input,
            Test {
                divisible_by,
                targets: (false_outcome as usize, true_outcome as usize),
            },
        ))
    }

    fn monkey(input: &str) -> IResult<&str, Monkey> {
        let (input, items) = preceded(
            tuple((tag("Monkey "), digit1, tag(":"), multispace1)),
            start_items,
        )(input)?;
        let (input, operation) = preceded(multispace1, operation)(input)?;
        let (input, test) = preceded(multispace1, test)(input)?;
        Ok((
            input,
            Monkey {
                items,
                operation,
                test,
            },
        ))
    }

    pub(super) fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
        separated_list1(count(newline, 2), monkey)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 10605)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 2713310158)
    }
}

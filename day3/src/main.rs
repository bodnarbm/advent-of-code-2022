use std::io::{self, stdin};

fn main() -> io::Result<()> {
    let input = io::read_to_string(stdin())?;
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
    Ok(())
}

fn part_1(input: &str) -> u32 {
    input.lines().map(priority_part_1).sum()
}

fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    let mut iter = input.lines();
    while let (Some(first), Some(second), Some(third)) = (iter.next(), iter.next(), iter.next()) {
        sum += priority_part_2([first, second, third]);
    }
    sum
}

fn to_bitfield(c: char) -> u64 {
    let offset = (c as u32) - ('A' as u32);
    let bit = match offset {
        0..=25 => offset + 26,
        32..=57 => offset - 32,
        _ => return 0,
    };
    1 << bit
}

fn item_set(container: &str) -> u64 {
    let mut set = 0;
    for c in container.chars() {
        set |= to_bitfield(c);
    }
    set
}

fn priority_part_1(rucksack: &str) -> u32 {
    let (first, second) = rucksack.split_at(rucksack.len() / 2);
    let first_set = item_set(first);
    let second_set = item_set(second);
    let matching_bit = (first_set & second_set).trailing_zeros();
    matching_bit + 1
}

fn priority_part_2(group: [&str; 3]) -> u32 {
    let first = item_set(group[0]);
    let second = item_set(group[1]);
    let third = item_set(group[2]);
    let matching_bit = (first & second & third).trailing_zeros();
    matching_bit + 1
}

#[cfg(test)]
mod test {
    use crate::priority_part_1;

    use super::*;

    static EXAMPLE_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT), 157);
    }

    #[test]
    fn priority_part_1_examples() {
        assert_eq!(priority_part_1("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
        assert_eq!(priority_part_1("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
        assert_eq!(priority_part_1("PmmdzqPrVvPwwTWBwg"), 42);
        assert_eq!(priority_part_1("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22);
        assert_eq!(priority_part_1("ttgJtRGJQctTZtZT"), 20);
        assert_eq!(priority_part_1("CrZsJsPPZsGzwwsLwLmpwMDw"), 19);
    }

    #[test]
    fn to_bitfield_examples() {
        assert_eq!(to_bitfield('a'), 1 << 0);
        assert_eq!(to_bitfield('z'), 1 << 25);
        assert_eq!(to_bitfield('A'), 1 << 26);
        assert_eq!(to_bitfield('Z'), 1 << 51);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT), 70);
    }

    #[test]
    fn priority_part_2_examples() {
        let group_1 = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];

        let group_2 = [
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        assert_eq!(priority_part_2(group_1), 18);
        assert_eq!(priority_part_2(group_2), 52);
    }
}

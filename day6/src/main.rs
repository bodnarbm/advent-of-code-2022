use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    match part1(&input) {
        Ok(result) => println!("Part 1: {}", result),
        Err(err) => eprintln!("Part 1: Error = {}", err),
    }
    match part2(&input) {
        Ok(result) => println!("Part 2: {}", result),
        Err(err) => eprintln!("Part 2: Error = {}", err),
    }
    Ok(())
}

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

trait Buffer<T> {
    fn occurs(&self, value: T) -> usize;
}

impl<const S: usize> Buffer<char> for [char; S] {
    fn occurs(&self, value: char) -> usize {
        self.iter().filter(|c| **c == value).count()
    }
}

fn start_of_packet<const LENGTH: usize>(input: &str) -> Result<usize> {
    if input.len() < LENGTH {
        return Err(format!(
            "messages with length less than {} cannot have a start of packet marker",
            LENGTH
        )
        .into());
    }
    let mut chars = input.chars();
    let mut buffer = [' '; LENGTH];
    let mut duplicate_count = 0;
    for (i, c) in chars.by_ref().take(LENGTH).enumerate() {
        buffer[i] = c;
        duplicate_count += buffer[..i].iter().filter(|&&o| o == c).count();
    }
    let mut pos = LENGTH;
    for c in chars {
        if duplicate_count == 0 {
            return Ok(pos);
        }
        let replace_index = pos % LENGTH;
        let to_replace = buffer[replace_index];
        duplicate_count -= buffer.occurs(to_replace) - 1;
        buffer[replace_index] = c;
        duplicate_count += buffer.occurs(c) - 1;
        pos += 1;
    }
    Err("No start of packet found".into())
}

fn part1(input: &str) -> Result<usize> {
    start_of_packet::<4>(input)
}

fn part2(input: &str) -> Result<usize> {
    start_of_packet::<14>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUTS: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn part_1_example() -> Result<()> {
        assert_eq!(part1(EXAMPLE_INPUTS[0])?, 7);
        assert_eq!(part1(EXAMPLE_INPUTS[1])?, 5);
        assert_eq!(part1(EXAMPLE_INPUTS[2])?, 6);
        assert_eq!(part1(EXAMPLE_INPUTS[3])?, 10);
        assert_eq!(part1(EXAMPLE_INPUTS[4])?, 11);

        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        assert_eq!(part2(EXAMPLE_INPUTS[0])?, 19);
        assert_eq!(part2(EXAMPLE_INPUTS[1])?, 23);
        assert_eq!(part2(EXAMPLE_INPUTS[2])?, 23);
        assert_eq!(part2(EXAMPLE_INPUTS[3])?, 29);
        assert_eq!(part2(EXAMPLE_INPUTS[4])?, 26);

        Ok(())
    }
}

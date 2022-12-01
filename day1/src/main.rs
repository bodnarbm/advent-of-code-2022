use std::io::{self, stdin};

fn most_calories_carried(input: &str) -> u32 {
    n_most_calories_carried(input, 1)
}

fn n_most_calories_carried(input: &str, n: usize) -> u32 {
    let mut most_calories = vec![u32::MIN; n];
    let mut current_calories = 0;

    for line in input.lines() {
        match (line, line.parse::<u32>()) {
            // Empty line means finished reading last elf
            ("", _) => {
                if current_calories > most_calories[0] {
                    most_calories[0] = current_calories;
                    most_calories.sort();
                }
                current_calories = 0;
            }
            (_, Ok(item_calories)) => {
                current_calories += item_calories;
            }
            (_, Err(err)) => {
                eprintln!("Error when parsing line\nline: {line}\nerror: {err}")
            }
        };
    }

    // Handle final case if no new line;
    if current_calories > most_calories[0] {
        most_calories[0] = current_calories;
    }
    most_calories.iter().sum()
}

fn main() -> io::Result<()> {
    let input = io::read_to_string(stdin())?;
    let most_calories = most_calories_carried(&input);
    let most_calories_of_3 = n_most_calories_carried(&input, 3);
    println!("Part 1: Most Calories Carried={}", most_calories);
    println!(
        "Part 2: Most Calories Carried by 3 elfs={}",
        most_calories_of_3
    );
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn sample_input_1() {
        let input = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

        assert_eq!(most_calories_carried(input), 24000);
    }

    #[test]
    fn sample_input_2() {
        let input = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

        assert_eq!(n_most_calories_carried(input, 3), 45000);
    }
}

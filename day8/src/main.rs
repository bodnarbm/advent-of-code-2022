use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);

    let width = input[0].len();
    let height = input.len();

    let mut visible = vec![vec![false; width]; height];

    // Handle visible from left and top
    let mut dp = vec![vec![('-', '-'); width]; height];
    for col in 1..width - 1 {
        dp[0][col] = ('-', input[0][col])
    }
    for row in 1..height - 1 {
        dp[row][0] = (input[row][0], '-')
    }

    for row in 1..height - 1 {
        for col in 1..width - 1 {
            let left_wall = dp[row][col - 1].0;
            let top_wall = dp[row - 1][col].1;
            let current = input[row][col];
            visible[row][col] |= current > top_wall || current > left_wall;
            dp[row][col] = (left_wall.max(current), top_wall.max(current))
        }
    }

    // Handle visible from right and bottom
    let mut dp = vec![vec![('-', '-'); width]; height];
    for col in (1..width - 1).rev() {
        dp[height - 1][col] = ('-', input[height - 1][col])
    }
    for row in (1..height - 1).rev() {
        dp[row][width - 1] = (input[row][width - 1], '-')
    }

    for row in (1..height - 1).rev() {
        for col in (1..width - 1).rev() {
            let right_wall = dp[row][col + 1].0;
            let bottom_wall = dp[row + 1][col].1;
            let current = input[row][col];
            visible[row][col] |= current > bottom_wall || current > right_wall;
            dp[row][col] = (right_wall.max(current), bottom_wall.max(current))
        }
    }

    (width + height - 2) * 2 + visible.into_iter().flatten().filter(|&v| v).count()
}

fn part2(input: &str) -> usize {
    let input = parse_input(input);
    let height = input.len();
    let width = input[0].len();

    let mut scores = vec![];

    for row in 1..height - 1 {
        for col in 1..width - 1 {
            let current = input[row][col];
            let left = col
                - (0..col)
                    .rev()
                    .find(|&c| input[row][c] >= current)
                    .unwrap_or(0);
            let right = (col + 1..width)
                .find(|&c| input[row][c] >= current)
                .unwrap_or(width - 1)
                - col;
            let up = row
                - (0..row)
                    .rev()
                    .find(|&r| input[r][col] >= current)
                    .unwrap_or(0);
            let down = (row + 1..height)
                .find(|&r| input[r][col] >= current)
                .unwrap_or(height - 1)
                - row;

            let score = up * right * down * left;
            scores.push(score);
        }
    }
    scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 8);
    }
}

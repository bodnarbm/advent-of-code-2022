use std::{cmp::Reverse, collections::BinaryHeap, io, str::FromStr};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let input: Map = input.parse().unwrap();
    input.shortest_path(input.start, input.end)
}

fn part2(input: &str) -> usize {
    let input: Map = input.parse().unwrap();

    let mut candidates = vec![];
    let rows = input.heights.len();
    let cols = input.heights[0].len();
    for r in 0..rows {
        for c in 0..cols {
            let cell = input.heights[r][c];
            if cell == b'a' {
                candidates.push((r, c));
            }
        }
    }
    candidates
        .into_iter()
        .map(|start| input.shortest_path(start, input.end))
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Map {
    heights: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn connected(&self, src: (usize, usize), dest: (usize, usize)) -> bool {
        let src_height = self.heights[src.0][src.1];
        let dest_height = self.heights[dest.0][dest.1];
        src_height >= dest_height - 1
    }

    fn neigbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        let width = self.heights[0].len();
        let height = self.heights.len();
        if pos.0 > 0 {
            let top = (pos.0 - 1, pos.1);
            if self.connected(pos, top) {
                neighbors.push(top);
            }
        }

        if pos.1 < width - 1 {
            let right = (pos.0, pos.1 + 1);
            if self.connected(pos, right) {
                neighbors.push(right);
            }
        }

        if pos.0 < height - 1 {
            let down = (pos.0 + 1, pos.1);
            if self.connected(pos, down) {
                neighbors.push(down);
            }
        }

        if pos.1 > 0 {
            let left = (pos.0, pos.1 - 1);
            if self.connected(pos, left) {
                neighbors.push(left);
            }
        }
        neighbors
    }
    fn shortest_path(&self, start: (usize, usize), end: (usize, usize)) -> usize {
        let height = self.heights.len();
        let width = self.heights[0].len();

        let mut costs = vec![vec![usize::MAX; width]; height];
        let mut heap = BinaryHeap::new();
        costs[start.0][start.1] = 0;
        heap.push(Reverse((0, start)));

        while let Some(Reverse((cost, pos))) = heap.pop() {
            for neighbor in self.neigbors(pos) {
                if cost + 1 < costs[neighbor.0][neighbor.1] {
                    costs[neighbor.0][neighbor.1] = cost + 1;
                    heap.push(Reverse((cost + 1, neighbor)));
                }
            }
        }

        costs[end.0][end.1]
    }
}

impl FromStr for Map {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights: Vec<Vec<u8>> = s.lines().map(|line| line.as_bytes().to_vec()).collect();
        let width = heights[0].len();

        let start_pos = heights
            .iter()
            .flatten()
            .position(|c| *c == b'S')
            .ok_or("no start position")?;
        let end_pos = heights
            .iter()
            .flatten()
            .position(|c| *c == b'E')
            .ok_or("no start position")?;
        let start = (start_pos / width, start_pos % width);
        let end = (end_pos / width, end_pos % width);
        heights[start.0][start.1] = b'a';
        heights[end.0][end.1] = b'z';
        Ok(Self {
            heights,
            start,
            end,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 31)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 29)
    }
}

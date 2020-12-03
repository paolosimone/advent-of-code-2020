use std::{fs::read_to_string, path::Path};

use super::{input_folder, Day};

#[derive(Default)]
pub struct Day03 {
    input: Area,
}

impl Day03 {
    fn count_trees(&self, right: usize, down: usize) -> usize {
        (0..self.input.len())
            .step_by(down)
            .enumerate()
            .filter(|(step, i)| {
                let row = &self.input[*i];
                row[(step * right) % row.len()] == Square::Tree
            })
            .count()
    }

    fn parse_input(s: &str) -> Area {
        s.lines()
            .map(|line| line.chars().map(Square::from).collect())
            .collect()
    }
}

impl Day for Day03 {
    fn load_input(&mut self) {
        let path = Path::new(&input_folder()).join("day_03");
        let content = read_to_string(path).expect("Load input failed");
        self.input = Day03::parse_input(&content);
    }

    fn first_challenge(&self) -> String {
        self.count_trees(3, 1).to_string()
    }

    fn second_challenge(&self) -> String {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(right, down)| self.count_trees(*right, *down))
            .fold(1, std::ops::Mul::mul)
            .to_string()
    }
}

type Area = Vec<Vec<Square>>;

#[derive(PartialEq)]
enum Square {
    Open,
    Tree,
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            '.' => Square::Open,
            '#' => Square::Tree,
            _ => panic!("Invalid square"),
        }
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_first_challenge() {
        let mut day = Day03::default();
        day.input = Day03::parse_input(INPUT);
        assert_eq!(day.first_challenge(), "7");
    }

    #[test]
    fn test_second_challenge() {
        let mut day = Day03::default();
        day.input = Day03::parse_input(INPUT);
        assert_eq!(day.second_challenge(), "336");
    }
}

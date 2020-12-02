use std::{collections::HashSet, fs::read_to_string, path::Path};

use super::{input_folder, Day};

const TARGET: i32 = 2020;

#[derive(Default)]
pub struct Day01 {
    input: Vec<i32>,
}

impl Day for Day01 {
    fn load_input(&mut self) {
        let path = Path::new(&input_folder()).join("day_01");
        let content = read_to_string(path).expect("Load input failed");

        self.input = content
            .lines()
            .map(|line| line.parse::<i32>())
            .collect::<Result<_, _>>()
            .expect("Parse input failed");
    }

    // O(N)
    fn first_challenge(&self) -> String {
        let mut seen = HashSet::new();
        for &x in self.input.iter() {
            if seen.contains(&(TARGET - x)) {
                return (x * (TARGET - x)).to_string();
            }
            seen.insert(x);
        }

        "NOT FOUND".to_string()
    }

    // O(N^3) - Look for better solutions elsewhere...
    fn second_challenge(&self) -> String {
        for i in 0..self.input.len() {
            for j in (i + 1)..self.input.len() {
                for k in (j + 1)..self.input.len() {
                    if self.input[i] + self.input[j] + self.input[k] == TARGET {
                        return (self.input[i] * self.input[j] * self.input[k]).to_string();
                    }
                }
            }
        }

        "NOT FOUND".to_string()
    }
}

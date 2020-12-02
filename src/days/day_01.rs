use std::{collections::HashSet, fs::File, io::Read};

use super::{input_path, Day};

const TARGET: i32 = 2020;

pub struct Day01 {
    input: Vec<i32>,
}

impl Day01 {
    const NUMBER: u8 = 1;

    pub fn new() -> Self {
        Self {
            input: Self::load_input().expect("Error loading input"),
        }
    }

    fn load_input() -> std::io::Result<Vec<i32>> {
        let mut file = File::open(input_path(Self::NUMBER))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect())
    }
}

impl Day for Day01 {
    fn number(&self) -> u8 {
        Self::NUMBER
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

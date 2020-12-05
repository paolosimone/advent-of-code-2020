use std::collections::HashSet;

use super::Day;

const TARGET: i32 = 2020;

pub struct Day01 {
    input: Vec<i32>,
}

impl Day01 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> Vec<i32> {
        s.lines()
            .map(|line| line.parse::<i32>())
            .collect::<Result<_, _>>()
            .expect("Parse input failed")
    }
}

impl Day for Day01 {
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

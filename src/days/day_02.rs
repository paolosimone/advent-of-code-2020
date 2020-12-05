use std::{error::Error, str::FromStr};

use super::Day;

pub struct Day02 {
    input: Vec<Entry>,
}

impl Day02 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> Vec<Entry> {
        s.lines()
            .map(Entry::from_str)
            .collect::<Result<_, _>>()
            .expect("Parse input failed")
    }

    fn count_valid<V>(&self) -> usize
    where
        V: Validator,
    {
        self.input
            .iter()
            .filter(|&entry| V::is_valid(&entry.policy, &entry.password))
            .count()
    }
}

impl Day for Day02 {
    fn first_challenge(&self) -> String {
        self.count_valid::<OldValidator>().to_string()
    }

    fn second_challenge(&self) -> String {
        self.count_valid::<NewValidator>().to_string()
    }
}

struct Entry {
    password: String,
    policy: Policy,
}

impl FromStr for Entry {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s.split(": ").collect::<Vec<_>>();

        Ok(Self {
            policy: vec[0].parse()?,
            password: vec[1].trim().to_string(),
        })
    }
}

struct Policy {
    letter: char,
    first: usize,
    second: usize,
}

impl FromStr for Policy {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s.split(|c| c == '-' || c == ' ').collect::<Vec<_>>();

        Ok(Self {
            first: vec[0].parse::<usize>()?,
            second: vec[1].parse::<usize>()?,
            letter: vec[2].chars().next().ok_or("Letter not found")?,
        })
    }
}

struct OldValidator;
struct NewValidator;

trait Validator {
    fn is_valid(policy: &Policy, password: &str) -> bool;
}

impl Validator for OldValidator {
    fn is_valid(policy: &Policy, password: &str) -> bool {
        let count = password.chars().filter(|&c| c == policy.letter).count();
        (policy.first..=policy.second).contains(&count)
    }
}

impl Validator for NewValidator {
    fn is_valid(policy: &Policy, password: &str) -> bool {
        let chars = password.as_bytes();
        let target = policy.letter as u8;
        (chars[policy.first - 1] == target) ^ (chars[policy.second - 1] == target)
    }
}

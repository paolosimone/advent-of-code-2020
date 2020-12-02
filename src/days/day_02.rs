use std::{error::Error, fs::File, io::Read, str::FromStr};

use super::{input_path, Day};
use counter::Counter;

pub struct Day02 {
    input: Vec<Entry>,
}

impl Day02 {
    const NUMBER: u8 = 2;

    pub fn new() -> Self {
        Self {
            input: Self::load_input().expect("Error loading input"),
        }
    }

    fn load_input() -> Result<Vec<Entry>, Box<dyn Error>> {
        let mut file = File::open(input_path(Self::NUMBER))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        contents.lines().map(Entry::from_str).collect()
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
    fn number(&self) -> u8 {
        Self::NUMBER
    }

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
        let counter = password.chars().collect::<Counter<_>>();
        (policy.first..=policy.second).contains(&counter[&policy.letter])
    }
}

impl Validator for NewValidator {
    fn is_valid(policy: &Policy, password: &str) -> bool {
        let chars = password.as_bytes();
        let target = policy.letter as u8;
        (chars[policy.first - 1] == target) ^ (chars[policy.second - 1] == target)
    }
}

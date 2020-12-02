use std::{fs::File, io::Read};

use super::{input_path, Day};
use counter::Counter;

pub struct Day02 {
    input: Vec<Entry>,
}

#[derive(Copy, Clone)]
struct Policy {
    letter: char,
    first: usize,
    second: usize,
}

struct PolicyOld(Policy);
struct PolicyNew(Policy);

struct Entry {
    password: String,
    policy: Policy,
}

trait Validator: From<Policy> {
    fn is_valid(&self, password: &str) -> bool;
}

impl From<&str> for Policy {
    fn from(string: &str) -> Self {
        let vec = string.split(|c| c == '-' || c == ' ').collect::<Vec<_>>();

        Self {
            first: vec[0].parse::<usize>().unwrap(),
            second: vec[1].parse::<usize>().unwrap(),
            letter: vec[2].chars().next().unwrap(),
        }
    }
}

impl From<&str> for Entry {
    fn from(string: &str) -> Self {
        let vec = string.split(": ").collect::<Vec<_>>();

        Entry {
            password: vec[1].trim().to_string(),
            policy: vec[0].into(),
        }
    }
}

impl From<Policy> for PolicyOld {
    fn from(policy: Policy) -> Self {
        Self(policy)
    }
}

impl Validator for PolicyOld {
    fn is_valid(&self, password: &str) -> bool {
        let policy = &self.0;
        let counter = password.chars().collect::<Counter<_>>();
        (policy.first..=policy.second).contains(&counter[&policy.letter])
    }
}

impl From<Policy> for PolicyNew {
    fn from(policy: Policy) -> Self {
        Self(policy)
    }
}

impl Validator for PolicyNew {
    fn is_valid(&self, password: &str) -> bool {
        let policy = &self.0;
        let chars = password.as_bytes();
        let target = policy.letter as u8;
        (chars[policy.first - 1] == target) ^ (chars[policy.second - 1] == target)
    }
}

impl Day02 {
    const NUMBER: u8 = 2;

    pub fn new() -> Self {
        Self {
            input: Self::load_input().expect("Error loading input"),
        }
    }

    fn load_input() -> std::io::Result<Vec<Entry>> {
        let mut file = File::open(input_path(Self::NUMBER))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents.lines().map(Entry::from).collect())
    }

    fn count_valid<T>(&self) -> usize
    where
        T: Validator,
    {
        self.input
            .iter()
            .filter(|&entry| T::from(entry.policy).is_valid(&entry.password))
            .count()
    }
}

impl Day for Day02 {
    fn number(&self) -> u8 {
        Self::NUMBER
    }

    fn first_challenge(&self) -> String {
        self.count_valid::<PolicyOld>().to_string()
    }

    fn second_challenge(&self) -> String {
        self.count_valid::<PolicyNew>().to_string()
    }
}

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
    policy_old: PolicyOld,
    policy_new: PolicyNew,
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
        let policy = vec[0].into();

        Entry {
            password: vec[1].trim().to_string(),
            policy_old: PolicyOld(policy),
            policy_new: PolicyNew(policy),
        }
    }
}

impl PolicyOld {
    pub fn is_valid(&self, password: &str) -> bool {
        let policy = &self.0;
        let counter = password.chars().collect::<Counter<_>>();
        (policy.first..=policy.second).contains(&counter[&policy.letter])
    }
}

impl PolicyNew {
    pub fn is_valid(&self, password: &str) -> bool {
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

        Ok(contents.lines().map(|line| line.into()).collect())
    }
}

impl Day for Day02 {
    fn number(&self) -> u8 {
        Self::NUMBER
    }

    fn first_challenge(&self) -> String {
        self.input
            .iter()
            .filter(|&entry| entry.policy_old.is_valid(&entry.password))
            .count()
            .to_string()
    }

    fn second_challenge(&self) -> String {
        self.input
            .iter()
            .filter(|&entry| entry.policy_new.is_valid(&entry.password))
            .count()
            .to_string()
    }
}

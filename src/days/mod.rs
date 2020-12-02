use std::{fmt::Display, path::Path};

mod day_01;

pub struct Advent {
    pub days: Vec<Box<dyn Day>>,
}

impl Advent {
    pub fn new() -> Self {
        Self {
            days: vec![Box::new(day_01::Day01::new())],
        }
    }
}

pub trait Day {
    fn number(&self) -> u8;
    fn first_challenge(&self) -> String;
    fn second_challenge(&self) -> String;
}

impl Display for dyn Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Day {:02} - first: {} | second: {}",
            self.number(),
            self.first_challenge(),
            self.second_challenge()
        )
    }
}

pub fn input_path(number: u8) -> String {
    // it's christmas: let's unwrap all options!
    // (I hope I'm missing a better way...)
    Path::new(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("input")
        .join(format!("day_{:02}", number))
        .as_path()
        .to_str()
        .unwrap()
        .into()
}

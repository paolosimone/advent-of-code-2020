use std::{fmt::Display, path::Path};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

pub struct Advent {
    pub days: Vec<Box<dyn Day>>,
}

impl Advent {
    pub fn new() -> Self {
        Self {
            days: vec![
                Box::new(day_01::Day01::default()),
                Box::new(day_02::Day02::default()),
                Box::new(day_03::Day03::default()),
                Box::new(day_04::Day04::default()),
                Box::new(day_05::Day05::default()),
            ],
        }
    }
}

pub trait Day {
    fn load_input(&mut self);
    fn first_challenge(&self) -> String;
    fn second_challenge(&self) -> String;
}

impl Display for dyn Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Day - first: {} | second: {}",
            self.first_challenge(),
            self.second_challenge()
        )
    }
}

pub fn input_folder() -> String {
    // it's christmas: let's unwrap all options!
    // (I hope I'm missing a better way...)
    Path::new(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("input")
        .as_path()
        .to_str()
        .unwrap()
        .into()
}

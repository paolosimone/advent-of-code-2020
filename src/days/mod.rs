use std::{fs::read_to_string, ops::RangeInclusive, path::Path};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

pub struct Advent {
    input_folder: String,
}

impl Advent {
    pub const DAY_NUMBERS: RangeInclusive<usize> = 1..=6;

    pub fn new(input_folder: String) -> Self {
        Self { input_folder }
    }

    pub fn load_day(&self, number: usize) -> Box<dyn Day> {
        let ref input_path = Path::new(&self.input_folder)
            .join(format!("day_{:02}", number))
            .to_str()
            .unwrap()
            .to_owned();

        let ref input = read_to_string(input_path).expect("Load input failed");

        match number {
            1 => Box::new(day_01::Day01::load(input)),
            2 => Box::new(day_02::Day02::load(input)),
            3 => Box::new(day_03::Day03::load(input)),
            4 => Box::new(day_04::Day04::load(input)),
            5 => Box::new(day_05::Day05::load(input)),
            6 => Box::new(day_06::Day06::load(input)),
            _ => panic!("Error 404: day {} not found!"),
        }
    }
}

pub trait Day {
    fn first_challenge(&self) -> String;
    fn second_challenge(&self) -> String;
}

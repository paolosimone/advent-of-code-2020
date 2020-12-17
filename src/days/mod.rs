use std::{fs::read_to_string, ops::RangeInclusive, path::Path};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

pub trait Day {
    fn first_challenge(&self) -> String;
    fn second_challenge(&self) -> String;
}

pub struct Advent {
    input_folder: String,
}

impl Advent {
    pub const DAY_NUMBERS: RangeInclusive<usize> = 1..=17;

    pub fn new(input_folder: String) -> Self {
        Self { input_folder }
    }

    pub fn load_day(&self, number: usize) -> Box<dyn Day> {
        let input_path = &Path::new(&self.input_folder)
            .join(format!("day_{:02}", number))
            .to_str()
            .unwrap()
            .to_owned();

        let input = &read_to_string(input_path).expect("Load input failed");

        match number {
            1 => Box::new(day_01::Day01::load(input)),
            2 => Box::new(day_02::Day02::load(input)),
            3 => Box::new(day_03::Day03::load(input)),
            4 => Box::new(day_04::Day04::load(input)),
            5 => Box::new(day_05::Day05::load(input)),
            6 => Box::new(day_06::Day06::load(input)),
            7 => Box::new(day_07::Day07::load(input)),
            8 => Box::new(day_08::Day08::load(input)),
            9 => Box::new(day_09::Day09::load(input)),
            10 => Box::new(day_10::Day10::load(input)),
            11 => Box::new(day_11::Day11::load(input)),
            12 => Box::new(day_12::Day12::load(input)),
            13 => Box::new(day_13::Day13::load(input)),
            14 => Box::new(day_14::Day14::load(input)),
            15 => Box::new(day_15::Day15::load(input)),
            16 => Box::new(day_16::Day16::load(input)),
            17 => Box::new(day_17::Day17::load(input)),
            _ => panic!("Error 404: day {} not found!", number),
        }
    }
}

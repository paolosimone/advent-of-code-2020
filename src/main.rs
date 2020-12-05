use std::path::Path;

use days::Advent;
use output::{build_table, MERRY_CHRISTMAS};
use report::build_report;

#[macro_use]
extern crate lazy_static;

mod days;

fn main() {
    println!("{}", MERRY_CHRISTMAS);

    let day = std::env::args()
        .nth(1)
        .map(|s| s.parse::<usize>().expect("Invalid number"));

    let advent = Advent::new(input_folder());
    let report = match day {
        Some(number) => build_report(advent, number..=number),
        _ => build_report(advent, Advent::DAY_NUMBERS),
    };

    build_table(&report)
        .print_stdout()
        .expect("Error printing results");
}

fn input_folder() -> String {
    Path::new(file!())
        .parent()
        .unwrap()
        .join("input")
        .to_str()
        .unwrap()
        .into()
}

mod report {
    use std::{
        ops::RangeInclusive,
        time::{Duration, Instant},
    };

    use crate::days::Advent;

    pub struct DayResult {
        pub number: usize,
        pub load_elapsed: Duration,
        pub first_result: String,
        pub first_elapsed: Duration,
        pub second_result: String,
        pub second_elapsed: Duration,
    }

    pub type Report = Vec<DayResult>;
    pub type ReportSlice<'a> = &'a [DayResult];

    pub fn build_report(advent: Advent, day_numbers: RangeInclusive<usize>) -> Report {
        day_numbers
            .map(|number| build_day_result(&advent, number))
            .collect()
    }

    macro_rules! elapsed {
        ($expression:expr) => {{
            let clock = Instant::now();
            let result = $expression;
            (result, clock.elapsed())
        }};
    }

    fn build_day_result(advent: &Advent, number: usize) -> DayResult {
        let (day, load_elapsed) = elapsed!(advent.load_day(number));
        let (first_result, first_elapsed) = elapsed!(day.first_challenge());
        let (second_result, second_elapsed) = elapsed!(day.second_challenge());

        DayResult {
            number,
            first_result,
            second_result,
            load_elapsed,
            first_elapsed,
            second_elapsed,
        }
    }
}

mod output {
    use cli_table::{Cell, Row, Table};

    use crate::report::{DayResult, ReportSlice};

    pub fn build_table(report: ReportSlice) -> Table {
        let mut rows = vec![build_header()];
        rows.extend(report.iter().map(build_row));
        Table::new(rows, Default::default()).unwrap()
    }

    fn build_header() -> Row {
        Row::new(vec![
            Cell::new("day", Default::default()),
            Cell::new("load_elapsed", Default::default()),
            Cell::new("first_result", Default::default()),
            Cell::new("first_elapsed", Default::default()),
            Cell::new("second_result", Default::default()),
            Cell::new("second_elapsed", Default::default()),
        ])
    }

    fn build_row(day: &DayResult) -> Row {
        Row::new(vec![
            Cell::new(&format!("{:02}", &day.number), Default::default()),
            Cell::new(&format!("{:?}", &day.load_elapsed), Default::default()),
            Cell::new(&day.first_result.to_string(), Default::default()),
            Cell::new(&format!("{:?}", &day.first_elapsed), Default::default()),
            Cell::new(&day.second_result.to_string(), Default::default()),
            Cell::new(&format!("{:?}", &day.second_elapsed), Default::default()),
        ])
    }

    pub const MERRY_CHRISTMAS: &str = r"

         ,---.  ,---.  ,---.  .-.   .-.                          
         |\    /| | .-'  | .-.\ | .-.\  \ \_/ )/                          
         |(\  / | | `-.  | `-'/ | `-'/   \   (_)                          
         (_)\/  | | .-'  |   (  |   (     ) (                             
         | \  / | |  `--.| |\ \ | |\ \    | |                             
         | |\/| | /( __.'|_| \)\|_| \)\  /(_|                             
         '-'  '-'(__)        (__)   (__)(__)                              
           ,--,  ,---.  .-. .-.   .---.  _______           .--.     .---. 
         .' .')  | .-.\ | | | |  ( .-._)|__   __||\    /| / /\ \   ( .-._)
         |  |(_) | `-'/ | | | | (_) \     )| |   |(\  / |/ /__\ \ (_) \   
         \  \    |   (  | | | | _  \ \   (_) |   (_)\/  ||  __  | _  \ \  
          \  `-. | |\ \ | `-')|( `-'  )    | |   | \  / || |  |)|( `-'  ) 
           \____\|_| \)\`---(_) `----'     `-'   | |\/| ||_|  (_) `----'  
                     (__)                        '-'  '-'                 

    ";
}

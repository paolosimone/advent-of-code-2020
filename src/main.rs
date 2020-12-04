use std::time::{Duration, Instant};

use cli_table::{Cell, Row, Table};
use days::{Advent, Day};

#[macro_use]
extern crate lazy_static;

mod days;

fn main() {
    println!("{}", MERRY_CHRISTMAS);

    let day = std::env::args()
        .nth(1)
        .map(|s| s.parse::<usize>().expect("Invalid number"));

    let advent = Advent::new();
    let report = match day {
        Some(number) => build_day_report(advent, number),
        _ => build_report(advent),
    };

    build_table(&report)
        .print_stdout()
        .expect("Error printing results");
}

#[derive(Debug)]
struct DayResult {
    number: usize,
    load_elapsed: Duration,
    first_result: String,
    first_elapsed: Duration,
    second_result: String,
    second_elapsed: Duration,
}

type Report = Vec<DayResult>;
type ReportSlice<'a> = &'a [DayResult];

fn build_report(advent: Advent) -> Report {
    advent
        .days
        .into_iter()
        .enumerate()
        .map(|(i, day)| build_day_result(i + 1, day))
        .collect()
}

fn build_day_report(advent: Advent, number: usize) -> Report {
    let day = advent
        .days
        .into_iter()
        .nth(number - 1)
        .expect("Day is missing");

    vec![build_day_result(number, day)]
}

fn build_day_result(number: usize, mut day: Box<dyn Day>) -> DayResult {
    // TODO maybe use macro to measure time?
    let load_input_start = Instant::now();
    day.load_input();

    let first_challenge_start = Instant::now();
    let first_challenge = day.first_challenge();

    let second_challenge_start = Instant::now();
    let second_challenge = day.second_challenge();
    let final_tick = Instant::now();

    DayResult {
        number,
        first_result: first_challenge,
        second_result: second_challenge,
        load_elapsed: first_challenge_start.duration_since(load_input_start),
        first_elapsed: second_challenge_start.duration_since(first_challenge_start),
        second_elapsed: final_tick.duration_since(second_challenge_start),
    }
}

fn build_table(report: ReportSlice) -> Table {
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

const MERRY_CHRISTMAS: &str = r"

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

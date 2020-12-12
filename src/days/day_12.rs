use std::{error::Error, str::FromStr};

use super::Day;

pub struct Day12 {
    input: Vec<Action>,
}

impl Day12 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> Vec<Action> {
        s.lines()
            .map(Action::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }
}

impl Day for Day12 {
    fn first_challenge(&self) -> String {
        let start = Position::new();
        let end = self.input.iter().fold(start, NavigatorOld::next);

        end.distance(start).to_string()
    }

    fn second_challenge(&self) -> String {
        let start = Position::new();
        let waypoint = Position {
            x: 10,
            y: 1,
            dir: Direction::East,
        };
        let (end, _) = self
            .input
            .iter()
            .fold((start, waypoint), NavigatorNew::next);

        end.distance(start).to_string()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Position {
    const DIRECTIONS: &'static [Direction; 4] = &[
        Direction::East,
        Direction::South,
        Direction::West,
        Direction::North,
    ];

    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Direction::East,
        }
    }

    fn translate(&self, delta_x: i32, delta_y: i32) -> Self {
        Self {
            x: self.x + delta_x,
            y: self.y + delta_y,
            dir: self.dir,
        }
    }

    fn rotate(&self, degree: i32) -> Self {
        let i = Self::DIRECTIONS
            .iter()
            .position(|dir| dir == &self.dir)
            .unwrap();

        let new_dir = Self::DIRECTIONS[(i + degree as usize / 90) % Self::DIRECTIONS.len()];

        Self {
            x: self.x,
            y: self.y,
            dir: new_dir,
        }
    }

    fn rotate_origin(&self, degree: i32) -> Self {
        match degree {
            90 => Self {
                x: self.y,
                y: -self.x,
                dir: self.dir,
            },
            180 => Self {
                x: -self.x,
                y: -self.y,
                dir: self.dir,
            },
            270 => Self {
                x: -self.y,
                y: self.x,
                dir: self.dir,
            },
            _ => panic!("Unsupported rotation"),
        }
    }

    fn distance(&self, other: Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Move(i32, i32),
    Rotate(i32),
    Forward(i32),
}

impl FromStr for Action {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letter = s.chars().next().unwrap();
        let number = s[1..].parse::<i32>()?;
        let action = match letter {
            'E' => Self::Move(number, 0),
            'W' => Self::Move(-number, 0),
            'N' => Self::Move(0, number),
            'S' => Self::Move(0, -number),
            'L' => Self::Rotate(360 - number),
            'R' => Self::Rotate(number),
            'F' => Self::Forward(number),
            _ => panic!("Invalid action"),
        };
        Ok(action)
    }
}

struct NavigatorOld;

impl NavigatorOld {
    fn next(pos: Position, action: &Action) -> Position {
        match *action {
            Action::Move(delta_x, delta_y) => pos.translate(delta_x, delta_y),
            Action::Rotate(degrees) => pos.rotate(degrees),
            Action::Forward(delta) => Self::forward(pos, delta),
        }
    }

    fn forward(pos: Position, delta: i32) -> Position {
        match pos.dir {
            Direction::East => pos.translate(delta, 0),
            Direction::West => pos.translate(-delta, 0),
            Direction::North => pos.translate(0, delta),
            Direction::South => pos.translate(0, -delta),
        }
    }
}

struct NavigatorNew;

impl NavigatorNew {
    fn next((boat, waypoint): (Position, Position), action: &Action) -> (Position, Position) {
        match *action {
            Action::Move(delta_x, delta_y) => (boat, waypoint.translate(delta_x, delta_y)),
            Action::Rotate(degrees) => (boat, waypoint.rotate_origin(degrees)),
            Action::Forward(mul) => (boat.translate(mul * waypoint.x, mul * waypoint.y), waypoint),
        }
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "F10
N3
F7
R90
F11";
        let day = Day12::load(input);
        assert_eq!(day.first_challenge(), "25");
    }

    #[test]
    fn test_second_challenge() {
        let input = "F10
N3
F7
R90
F11";
        let day = Day12::load(input);
        assert_eq!(day.second_challenge(), "286");
    }
}

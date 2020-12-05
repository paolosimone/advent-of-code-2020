use std::{fs::read_to_string, path::Path};

use super::{input_folder, Day};

#[derive(Default)]
pub struct Day05 {
    input: Vec<Seat>,
}

impl Day05 {
    // Optimized for second challenge: I want to find my passport fast!
    // O(N * log(N))
    fn parse_input(s: &str) -> Vec<Seat> {
        let mut input = s.lines().map(Self::parse_seat).collect::<Vec<_>>();
        input.sort_by_key(|seat| seat.value);
        input
    }

    fn parse_seat(s: &str) -> Seat {
        let binary = s
            .chars()
            .map(|c| match c {
                'F' => '0',
                'B' => '1',
                'L' => '0',
                'R' => '1',
                _ => panic!("invalid seat!"),
            })
            .collect::<String>();

        let value = u32::from_str_radix(&binary, 2).unwrap();
        let row = u32::from_str_radix(&binary[0..7], 2).unwrap();
        let column = u32::from_str_radix(&binary[7..10], 2).unwrap();

        Seat {
            id: row * 8 + column,
            value,
        }
    }
}

impl Day for Day05 {
    fn load_input(&mut self) {
        let path = Path::new(&input_folder()).join("day_05");
        let content = read_to_string(path).expect("Load input failed");
        self.input = Day05::parse_input(&content);
    }

    // O(N)
    fn first_challenge(&self) -> String {
        self.input
            .iter()
            .map(|seat| seat.id)
            .max()
            .unwrap()
            .to_string()
    }

    // O(log(N))
    fn second_challenge(&self) -> String {
        let min = self.input[0].value;

        let mut l: usize = 0;
        let mut r: usize = self.input.len();

        while l < r {
            let m = (l + r) / 2;
            let expected_index = (self.input[m].value - min) as usize;
            if expected_index == m {
                l = m + 1;
            } else {
                r = m;
            }
        }

        (self.input[r].id - 1).to_string()
    }
}

struct Seat {
    id: u32,
    value: u32,
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let mut day = Day05::default();
        day.input = Day05::parse_input(
            "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL",
        );
        assert_eq!(day.first_challenge(), "820");
    }

    #[test]
    fn test_second_challenge() {
        let mut day = Day05::default();
        day.input = Day05::parse_input(
            "FFFFFFFFFB
FFFFFFFFBB",
        );
        assert_eq!(day.second_challenge(), "2");
    }
}

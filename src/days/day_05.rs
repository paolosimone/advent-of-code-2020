use super::Day;

pub struct Day05 {
    input: Vec<Seat>,
}

struct Seat {
    id: u32,
    value: u32,
}

impl Day05 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    // Optimized for second challenge: I want to find my seat fast 'cause I'm tired!
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

        let mut l = 0usize;
        let mut r = self.input.len();

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

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
        let day = Day05::load(input);
        assert_eq!(day.first_challenge(), "820");
    }

    #[test]
    fn test_second_challenge() {
        let input = "FFFFFFFFFB
FFFFFFFFBB";
        let day = Day05::load(input);
        assert_eq!(day.second_challenge(), "2");
    }
}

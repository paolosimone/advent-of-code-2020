use itertools::Itertools;

use super::Day;

pub struct Day14 {
    input: Vec<Op>,
}

impl Day14 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> Vec<Op> {
        s.lines().map(Self::parse_op).collect::<Vec<_>>()
    }

    fn parse_op(s: &str) -> Op {
        match &s[..3] {
            "mas" => {
                let mask = s.split("= ").skip(1).next().unwrap();
                Op::Mask(mask.to_string())
            }

            "mem" => {
                let (i, j) = (s.find('[').unwrap() + 1, s.find(']').unwrap());
                let key = s[i..j].parse::<usize>().unwrap();

                let (_, value_str) = s.splitn(2, "= ").collect_tuple().unwrap();
                let value = value_str.parse::<usize>().unwrap();

                Op::Mem(key, value)
            }

            _ => panic!("Unknown operation"),
        }
    }
}

impl Day for Day14 {
    fn first_challenge(&self) -> String {
        let mut state = v1::State::default();
        self.input.iter().for_each(|op| state.apply(op));
        state.total_sum().to_string()
    }

    // my original super clever solution was wrong... but was wrong 30x faster :)
    fn second_challenge(&self) -> String {
        let mut state = v2::State::default();
        self.input.iter().for_each(|op| state.apply(op));
        state.total_sum().to_string()
    }
}

enum Op {
    Mask(String),
    Mem(usize, usize),
}

mod v1 {
    use std::collections::HashMap;

    use super::Op;

    #[derive(Debug, Default)]
    pub(super) struct State {
        registers: HashMap<usize, usize>,
        mask: Mask,
    }

    impl State {
        pub(super) fn apply(&mut self, op: &Op) {
            match op {
                Op::Mask(bits) => {
                    self.mask = Mask::from_str(bits);
                }

                Op::Mem(key, value) => {
                    self.registers.insert(*key, self.mask.apply(*value));
                }
            }
        }

        pub(super) fn total_sum(&self) -> usize {
            self.registers.values().sum::<usize>()
        }
    }

    #[derive(Debug, Default)]
    struct Mask {
        and: usize,
        or: usize,
    }

    impl Mask {
        fn from_str(s: &str) -> Self {
            Self {
                and: usize::from_str_radix(&s.replace("X", "1"), 2).unwrap(),
                or: usize::from_str_radix(&s.replace("X", "0"), 2).unwrap(),
            }
        }

        fn apply(&self, x: usize) -> usize {
            x & self.and | &self.or
        }
    }
}

mod v2 {
    use std::collections::HashMap;

    use super::Op;

    #[derive(Debug, Default)]
    pub(super) struct State {
        registers: HashMap<usize, usize>,
        mask: Mask,
    }

    impl State {
        pub(super) fn apply(&mut self, op: &Op) {
            match op {
                Op::Mask(bits) => {
                    self.mask = Mask::from_str(&bits);
                }

                Op::Mem(key, value) => {
                    self.mask
                        .apply(*key)
                        .iter()
                        .for_each(|&key| drop(self.registers.insert(key, *value)));
                }
            }
        }

        pub(super) fn total_sum(&self) -> usize {
            self.registers.values().sum::<usize>()
        }
    }

    #[derive(Debug, Default)]
    struct Mask {
        free: usize,
        fixed: Vec<usize>,
    }

    impl Mask {
        fn from_str(s: &str) -> Self {
            Self {
                free: Self::parse_free(s),
                fixed: Self::parse_fixed(s),
            }
        }

        fn parse_free(s: &str) -> usize {
            let free = s
                .chars()
                .map(|c| match c {
                    'X' => '0',
                    _ => '1',
                })
                .collect::<String>();

            usize::from_str_radix(&free, 2).unwrap()
        }

        fn parse_fixed(s: &str) -> Vec<usize> {
            let mut fixed = vec![0];
            for (i, c) in s.chars().rev().enumerate() {
                let bit_mask = 1 << i;
                match c {
                    // force 1
                    '1' => {
                        fixed.iter_mut().for_each(|f| *f |= bit_mask);
                    }
                    // floating: both 0 and 1
                    'X' => {
                        let half = fixed.len();
                        fixed.extend(fixed.to_vec());
                        fixed[..half].iter_mut().for_each(|f| *f |= bit_mask);
                    }
                    // do nothing
                    _ => {}
                }
            }
            fixed
        }

        fn apply(&self, x: usize) -> Vec<usize> {
            let free = x & self.free;
            self.fixed.iter().map(|forced| free | forced).collect()
        }
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let day = Day14::load(input);
        assert_eq!(day.first_challenge(), "165");
    }

    #[test]
    fn test_second_challenge() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let day = Day14::load(input);
        assert_eq!(day.second_challenge(), "208");
    }
}

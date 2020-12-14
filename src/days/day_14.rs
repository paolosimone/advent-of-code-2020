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

    // ... apparently doesn't work :/
    fn second_challenge(&self) -> String {
        let mut state = v2::State::default();
        self.input.iter().for_each(|op| {
            state.apply(op);
            // println!("{:?}", state);
        });
        state.total_sum().to_string();
        "INCORRECT".to_string()
    }
}

const BITS: usize = 36;
const ONES: usize = (1 << BITS) - 1;

enum Op {
    Mask(String),
    Mem(usize, usize),
}

mod v1 {
    use std::collections::HashMap;

    use super::{Op, BITS};

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

    #[derive(Debug)]
    struct Mask {
        and: usize,
        or: usize,
    }

    impl Default for Mask {
        fn default() -> Self {
            let default = &String::from_utf8(vec![b'X'; BITS]).unwrap();
            Self::from_str(default)
        }
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
    use std::fmt::Debug;

    use super::{Op, BITS, ONES};

    #[derive(Debug, Default)]
    pub(super) struct State {
        registers: Vec<Register>,
        mask: FloatingValue,
    }

    impl State {
        pub(super) fn apply(&mut self, op: &Op) {
            match op {
                Op::Mask(bits) => {
                    self.mask = FloatingValue::from_str(bits);
                }
                Op::Mem(key, value) => {
                    let floating_key = FloatingValue {
                        fixed: self.mask.fixed,
                        value: (key | self.mask.value) & self.mask.fixed,
                    };

                    self.mem(floating_key, *value);
                }
            }
        }

        pub(super) fn total_sum(&self) -> usize {
            self.registers
                .iter()
                .filter(|&r| r.valid)
                .map(|r| r.value * Self::count_real_keys(r))
                .sum::<usize>()
        }

        fn mem(&mut self, key: FloatingValue, value: usize) {
            // probably we could do something better than a full scan?
            self.registers
                .iter_mut()
                .filter(|r| r.key.overlaps(&key))
                .for_each(|mut r| Self::update(&mut r, &key));

            self.registers.push(Register {
                key,
                value,
                valid: true,
            });
        }

        fn update(old: &mut Register, new_key: &FloatingValue) {
            old.valid = !new_key.contains(&old.key);

            let to_fix = !old.key.fixed & new_key.fixed;
            old.key.value = (old.key.value & !to_fix) | (to_fix & !new_key.value);
            old.key.fixed |= new_key.fixed;
        }

        fn count_real_keys(register: &Register) -> usize {
            1 << Self::count_ones(!register.key.fixed)
        }

        fn count_ones(x: usize) -> usize {
            (0..BITS).map(|i| (x >> i) & 1).sum()
        }
    }

    #[derive(Debug)]
    struct Register {
        key: FloatingValue,
        value: usize,
        valid: bool,
    }

    // 1XX0X -> fixed: 10010, value: 10000
    struct FloatingValue {
        fixed: usize,
        value: usize,
    }

    impl Debug for FloatingValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!(
                "FV {{fix: {:0b}, valb: {:0b}, val: {}}}",
                self.fixed, self.value, self.value
            ))
        }
    }

    impl Default for FloatingValue {
        fn default() -> Self {
            FloatingValue {
                fixed: ONES,
                value: 0,
            }
        }
    }

    impl FloatingValue {
        fn from_str(s: &str) -> Self {
            let fixed = s
                .chars()
                .map(|c| match c {
                    'X' => '0',
                    _ => '1',
                })
                .collect::<String>();

            Self {
                fixed: usize::from_str_radix(&fixed, 2).unwrap(),
                value: usize::from_str_radix(&s.replace("X", "0"), 2).unwrap(),
            }
        }

        fn overlaps(&self, other: &FloatingValue) -> bool {
            let both_fixed = self.fixed & other.fixed;
            self.value & both_fixed == other.value & both_fixed
        }

        fn contains(&self, other: &FloatingValue) -> bool {
            self.overlaps(other) && (self.fixed | other.fixed) == other.fixed
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

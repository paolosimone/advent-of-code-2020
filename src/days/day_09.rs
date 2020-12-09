use std::collections::HashSet;

use super::Day;

pub struct Day09 {
    input: Vec<i64>,
}

impl Day09 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> Vec<i64> {
        s.lines().map(|l| l.parse::<i64>().unwrap()).collect()
    }

    fn find_error(&self, window_size: usize) -> Option<i64> {
        let window = &mut HashSet::new();

        for i in 0..window_size {
            window.insert(self.input[i]);
        }

        for i in window_size..self.input.len() {
            let target = self.input[i];

            let ok = window.iter().any(|prev| window.contains(&(target - prev)));
            if !ok {
                return Some(target);
            }

            window.remove(&self.input[i - window_size]);
            window.insert(target);
        }

        None
    }

    fn find_range_with_sum(&self, target: i64) -> Option<&[i64]> {
        let (mut i, mut j, mut sum) = (0, 0, self.input[0]);

        while i < self.input.len() {
            if sum == target && j > i {
                return Some(&self.input[i..=j]);
            }

            if sum < target && j < self.input.len() - 1 {
                j += 1;
                sum += self.input[j];
            } else {
                sum -= self.input[i];
                i += 1;
            }
        }

        None
    }

    fn encryption_weakness(&self, slice: &[i64]) -> i64 {
        slice.iter().min().unwrap() + slice.iter().max().unwrap()
    }
}

impl Day for Day09 {
    fn first_challenge(&self) -> String {
        self.find_error(25).unwrap().to_string()
    }

    fn second_challenge(&self) -> String {
        self.find_range_with_sum(1492208709)
            .map(|slice| self.encryption_weakness(slice).to_string())
            .unwrap_or_else(|| "NOT_FOUND".to_string())
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let day = Day09 {
            input: Day09::parse_input(input),
        };
        assert_eq!(day.find_error(5), Some(127));
    }

    #[test]
    fn test_second_challenge() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let day = Day09 {
            input: Day09::parse_input(input),
        };
        assert_eq!(
            day.find_range_with_sum(127)
                .map(|range| day.encryption_weakness(range)),
            Some(62)
        );
    }
}

use std::collections::HashMap;

use super::Day;

pub struct Day15 {
    input: Vec<usize>,
}

impl Day15 {
    pub fn load(input: &str) -> Self {
        let input = input
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self { input }
    }
}

impl Day for Day15 {
    fn first_challenge(&self) -> String {
        Game::new(&self.input)
            .nth(2020 - self.input.len() - 1)
            .unwrap()
            .to_string()
    }

    fn second_challenge(&self) -> String {
        Game::new(&self.input)
            .nth(30000000 - self.input.len() - 1)
            .unwrap()
            .to_string()
    }
}

#[derive(Debug, Default)]
struct Game {
    index: usize,
    last: usize,
    last_seen: HashMap<usize, usize>,
}

impl Game {
    fn new(numbers: &[usize]) -> Self {
        let mut game = Game::default();

        game.last = numbers[0];
        for &n in numbers.iter().skip(1) {
            game.last_seen.insert(game.last, game.index);
            game.index += 1;
            game.last = n;
        }

        game
    }
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self
            .last_seen
            .get(&self.last)
            .map(|&prev_index| self.index - prev_index)
            .unwrap_or_default();

        self.last_seen.insert(self.last, self.index);
        self.last = next;
        self.index += 1;

        Some(next)
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "0,3,6";
        let day = Day15::load(input);
        assert_eq!(day.first_challenge(), "436");
    }

    #[test]
    fn test_first_challenge_1() {
        let input = "1,3,2";
        let day = Day15::load(input);
        assert_eq!(day.first_challenge(), "1");
    }

    #[test]
    fn test_first_challenge_2() {
        let input = "2,1,3";
        let day = Day15::load(input);
        assert_eq!(day.first_challenge(), "10");
    }

    #[test]
    fn test_first_challenge_3() {
        let input = "1,2,3";
        let day = Day15::load(input);
        assert_eq!(day.first_challenge(), "27");
    }

    #[test]
    #[ignore]
    fn test_second_challenge() {
        let input = "0,3,6";
        let day = Day15::load(input);
        assert_eq!(day.second_challenge(), "175594");
    }
}

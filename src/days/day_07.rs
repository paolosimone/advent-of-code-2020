use regex::Regex;
use std::collections::HashMap;

use super::Day;

pub struct Day07 {
    input: RuleSet,
}

type RuleSet = HashMap<Color, Vec<BagCount>>;
type BagCount = (Color, usize);
type Color = String;

impl Day07 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> RuleSet {
        s.lines().map(Self::parse_rule).collect()
    }

    fn parse_rule(s: &str) -> (Color, Vec<BagCount>) {
        lazy_static! {
            static ref SUBJECT_REGEX: Regex = Regex::new(r"^(?P<color>\w+ \w+) bags").unwrap();
            static ref OBJECT_REGEX: Regex =
                Regex::new(r"(?P<count>\d+) (?P<color>\w+ \w+) bag[s]?[,\.]").unwrap();
        }

        let color = SUBJECT_REGEX.captures(s).unwrap()["color"].to_owned();
        let contains = OBJECT_REGEX
            .captures_iter(s)
            .map(|c| (c["color"].to_owned(), c["count"].parse::<usize>().unwrap()))
            .collect();

        (color, contains)
    }

    fn count_paths(&self, target: Color) -> usize {
        let mut memo = HashMap::new();
        self.input
            .keys()
            .filter(|&color| color != &target)
            .map(|color| self.count_paths_dfs(&color, &target, &mut memo))
            .filter(|&paths| paths > 0)
            .count()
    }

    fn count_paths_dfs(
        &self,
        color: &str,
        target: &str,
        memo: &mut HashMap<Color, usize>,
    ) -> usize {
        if color == target {
            return 1;
        }

        if memo.contains_key(color) {
            return memo[color];
        }

        let paths = self.input[color]
            .iter()
            .map(|(color, _)| self.count_paths_dfs(color, target, memo))
            .sum();

        memo.insert(color.to_string(), paths);
        memo[color]
    }

    fn count_nested(&self, source: Color) -> usize {
        self.count_nested_dfs(&source, &mut HashMap::new()) - 1
    }

    fn count_nested_dfs(&self, color: &str, memo: &mut HashMap<Color, usize>) -> usize {
        if memo.contains_key(color) {
            return memo[color];
        }

        let nested = self.input[color]
            .iter()
            .map(|(color, bag_count)| self.count_nested_dfs(color, memo) * bag_count)
            .sum::<usize>();

        memo.insert(color.to_string(), nested + 1);
        memo[color]
    }
}

impl Day for Day07 {
    fn first_challenge(&self) -> String {
        let target = "shiny gold".to_string();
        self.count_paths(target).to_string()
    }

    fn second_challenge(&self) -> String {
        let source = "shiny gold".to_string();
        self.count_nested(source).to_string()
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let day = Day07::load(input);
        assert_eq!(day.first_challenge(), "4");
    }

    #[test]
    fn test_second_challenge() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let day = Day07::load(input);
        assert_eq!(day.second_challenge(), "32");
    }

    #[test]
    fn test_second_challenge_2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let day = Day07::load(input);
        assert_eq!(day.second_challenge(), "126");
    }
}

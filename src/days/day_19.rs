use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use super::Day;

#[derive(Clone)]
pub struct Day19 {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    And(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
    Plus(usize),
    Nested((usize, usize), usize),
}

impl Day19 {
    pub fn load(input: &str) -> Self {
        let (rules, messages) = input.splitn(2, "\n\n").collect_tuple().unwrap();

        Self {
            rules: Self::parse_rules(rules),
            messages: Self::parse_messages(messages),
        }
    }

    fn parse_rules(s: &str) -> HashMap<usize, Rule> {
        s.lines()
            .map(|l| {
                let (index, rule) = l.splitn(2, ": ").collect_tuple().unwrap();
                let index = index.parse::<usize>().unwrap();
                let rule = Self::parse_rule(rule);
                (index, rule)
            })
            .collect::<HashMap<_, _>>()
    }

    fn parse_rule(s: &str) -> Rule {
        if s.contains("\"") {
            return Rule::Char(s.chars().nth(1).unwrap());
        }

        if s.contains("|") {
            let (left, right) = s.splitn(2, " | ").collect_tuple().unwrap();
            return Rule::Or(Self::parse_nums(left), Self::parse_nums(right));
        }

        Rule::And(Self::parse_nums(s))
    }

    fn parse_nums(s: &str) -> Vec<usize> {
        s.split(" ")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec()
    }

    fn parse_messages(s: &str) -> Vec<String> {
        s.lines().map(|l| l.to_string()).collect()
    }
}

impl Day for Day19 {
    fn first_challenge(&self) -> String {
        let regex = build_regex(&self.rules);

        self.messages
            .iter()
            .filter(|&mex| regex.is_match(mex))
            .count()
            .to_string()
    }

    // there are bonus points for creativity, right? :D
    fn second_challenge(&self) -> String {
        let mut rules = self.rules.clone();
        rules.insert(8, Rule::Plus(42));

        // if you can't count to infinity, at which number should you stop?
        let (count, _) = (1..=42).fold((0, self.messages.clone()), |(count, ko), n| {
            rules.insert(11, Rule::Nested((42, 31), n));

            let regex = build_regex(&rules);

            let (ok, ko) = ko
                .into_iter()
                .partition::<Vec<String>, _>(|mex| regex.is_match(mex));

            (count + ok.len(), ko)
        });

        count.to_string()
    }
}

fn build_regex(rules: &HashMap<usize, Rule>) -> Regex {
    let regex = build_regex_string(rules, 0, &mut HashMap::new());
    Regex::new(format!("^{}$", regex).as_str()).unwrap()
}

fn build_regex_string(
    rules: &HashMap<usize, Rule>,
    index: usize,
    memo: &mut HashMap<usize, String>,
) -> String {
    if let Some(regex) = memo.get(&index) {
        return regex.to_string();
    }

    let regex = match rules.get(&index).unwrap() {
        Rule::Char(char) => char.to_string(),

        Rule::And(indexes) => format!("{}", build_regex_and(rules, indexes, memo)),

        Rule::Or(left, right) => format!(
            "({}|{})",
            build_regex_and(rules, left, memo),
            build_regex_and(rules, right, memo)
        ),

        &Rule::Plus(index) => format!("{}+", build_regex_string(rules, index, memo)),

        &Rule::Nested((left, right), n) => {
            let left = build_regex_string(rules, left, memo);
            let right = build_regex_string(rules, right, memo);
            format!("({}{{{}}}{}{{{}}})", left, n, right, n)
        }
    };

    memo.insert(index, regex.to_string());
    regex
}

fn build_regex_and(
    rules: &HashMap<usize, Rule>,
    indexes: &[usize],
    memo: &mut HashMap<usize, String>,
) -> String {
    indexes
        .iter()
        .map(|&i| build_regex_string(rules, i, memo).to_string())
        .join("")
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        let day = Day19::load(input);
        assert_eq!(day.first_challenge(), "2");
    }

    #[test]
    fn test_second_challenge() {
        let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        let day = Day19::load(input);
        assert_eq!(day.second_challenge(), "12");
    }

    #[test]
    fn test_second_challenge_plus() {
        let input = "0: 8
1: \"a\"
2: \"b\"
8: 1
42: 1

a
aaaaa
b
bbbbbbb
ab
ba
abababab
aaaabbbb";
        let day = Day19::load(input);
        assert_eq!(day.second_challenge(), "2");
    }

    #[test]
    fn test_second_challenge_nested() {
        let input = "0: 11
1: \"a\"
2: \"b\"
42: 1
31: 2

ab
aabb
aaaaaaabbbbbbb
aa
aaaa
a
aaaaa
b
bb
bbbbbbb
ba
bbaa
abababab";
        let day = Day19::load(input);
        assert_eq!(day.second_challenge(), "3");
    }
}

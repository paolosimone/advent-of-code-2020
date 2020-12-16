use std::{collections::HashSet, ops::RangeInclusive};

use itertools::Itertools;

use super::Day;

pub struct Day16 {
    rules: Vec<Rule>,
    mine: Ticket,
    others: Vec<Ticket>,
}

type Rule = (String, RangeInclusive<usize>, RangeInclusive<usize>);
type Ticket = Vec<usize>;

impl Day16 {
    pub fn load(input: &str) -> Self {
        let (rules, mine, others) = input.splitn(3, "\n\n").collect_tuple().unwrap();

        Self {
            rules: Self::parse_rules(rules),
            mine: Self::parse_tickets(mine)[0].to_vec(),
            others: Self::parse_tickets(others),
        }
    }

    fn parse_rules(s: &str) -> Vec<Rule> {
        s.lines()
            .map(|line| {
                let (field, ranges) = line.splitn(2, ": ").collect_tuple().unwrap();
                let (range1, range2) = ranges.splitn(2, " or ").collect_tuple().unwrap();

                (
                    field.to_string(),
                    Self::parse_range(range1),
                    Self::parse_range(range2),
                )
            })
            .collect()
    }

    fn parse_range(s: &str) -> RangeInclusive<usize> {
        let (start, stop) = s
            .splitn(2, "-")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        start..=stop
    }

    fn parse_tickets(s: &str) -> Vec<Ticket> {
        s.lines()
            .skip(1)
            .map(|line| {
                line.split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect()
            })
            .collect()
    }

    fn is_valid(&self, field: &usize) -> bool {
        self.rules.iter().any(|rule| Self::in_range(rule, field))
    }

    fn in_range((_, range1, range2): &Rule, field: &usize) -> bool {
        range1.contains(field) || range2.contains(field)
    }
}

impl Day for Day16 {
    fn first_challenge(&self) -> String {
        self.others
            .iter()
            .flatten()
            .filter(|&field| !self.is_valid(field))
            .sum::<usize>()
            .to_string()
    }

    fn second_challenge(&self) -> String {
        // consider only valid tickets
        let valid_others = self
            .others
            .iter()
            .filter(|&ticket| ticket.iter().all(|field| self.is_valid(field)));

        // each field can represent any rule...
        let all_rules = self.rules.iter().collect::<HashSet<_>>();
        let mut constraints = self
            .mine
            .iter()
            .map(|_| all_rules.clone())
            .collect::<Vec<_>>();

        // ..unless proved otherwise
        for ticket in valid_others {
            for (i, field) in ticket.iter().enumerate() {
                let mismatched = constraints[i]
                    .iter()
                    .filter(|&rule| !Self::in_range(rule, field))
                    .map(|&rule| rule.clone())
                    .collect::<Vec<_>>();

                mismatched
                    .iter()
                    .for_each(|rule| drop(constraints[i].remove(rule)));
            }
        }

        // keep only field names
        let mut fields = constraints
            .iter()
            .map(|rules| {
                rules
                    .iter()
                    .map(|(name, _, _)| name)
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        // resolve ambiguity
        while fields.iter().any(|f| f.len() > 1) {
            let unique = fields
                .iter()
                .filter(|f| f.len() == 1)
                .map(|f| f.iter().next().unwrap().to_string())
                .collect::<Vec<_>>();

            fields
                .iter_mut()
                .filter(|f| f.len() > 1)
                .for_each(|f| unique.iter().for_each(|name| drop(f.remove(name))));
        }

        // filter departure fields
        let departure_fields = fields
            .iter()
            .map(|f| f.iter().next().unwrap())
            .enumerate()
            .filter(|(_, name)| name.starts_with("departure"));

        // compute result
        departure_fields
            .map(|(i, _)| self.mine[i])
            .fold(1, |acc, field| acc * field)
            .to_string()
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let day = Day16::load(input);
        assert_eq!(day.first_challenge(), "71");
    }

    #[test]
    fn test_second_challenge() {
        let input = "class: 0-1 or 4-19
departure row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let day = Day16::load(input);
        assert_eq!(day.second_challenge(), "143");
    }
}

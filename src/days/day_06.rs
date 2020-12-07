use super::Day;

pub struct Day06 {
    input: Vec<GroupAnswers>,
}

const QUESTIONS: u32 = 26;

type GroupAnswers = Vec<PersonAnswers>;
type PersonAnswers = u32;

impl Day06 {
    const BLANK_LINE: &'static str = "\n\n";

    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> Vec<GroupAnswers> {
        s.split(Self::BLANK_LINE)
            .map(Self::parse_group_answers)
            .collect()
    }

    fn parse_group_answers(s: &str) -> GroupAnswers {
        s.lines().map(Self::parse_person_answers).collect()
    }

    // "daec" -> 1101
    fn parse_person_answers(s: &str) -> PersonAnswers {
        s.chars()
            .fold(0, |acc, c| acc | (1 << (c as u32 - 'a' as u32)))
    }

    // 1101 -> 3
    fn count_answers(answers: u32) -> u32 {
        (0..QUESTIONS).map(|i| (answers >> i) & 1).sum()
    }
}

impl Day for Day06 {
    fn first_challenge(&self) -> String {
        self.input
            .iter()
            .map(|group| {
                let union = group.iter().fold(0, |acc, answers| acc | answers);
                Self::count_answers(union)
            })
            .sum::<u32>()
            .to_string()
    }

    fn second_challenge(&self) -> String {
        self.input
            .iter()
            .map(|group| {
                let all_yes = (1 << QUESTIONS) - 1;
                let intersection = group.iter().fold(all_yes, |acc, answers| acc & answers);
                Self::count_answers(intersection)
            })
            .sum::<u32>()
            .to_string()
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let day = Day06::load(input);
        assert_eq!(day.first_challenge(), "11");
    }

    #[test]
    fn test_second_challenge() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let day = Day06::load(input);
        assert_eq!(day.second_challenge(), "6");
    }
}

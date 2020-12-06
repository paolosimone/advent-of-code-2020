use super::Day;

pub struct Day06 {
    input: Vec<GroupAnswers>,
}

const QUESTIONS: usize = 26;

type GroupAnswers = Vec<PersonAnswers>;
type PersonAnswers = Vec<bool>;

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

    fn parse_person_answers(s: &str) -> PersonAnswers {
        lazy_static! {
            static ref QUESTION_IDS: Vec<char> = (0..QUESTIONS as u32)
                .map(|q| std::char::from_u32('a' as u32 + q).unwrap())
                .collect();
        };

        QUESTION_IDS.iter().map(|q| s.contains(*q)).collect()
    }
}

impl Day for Day06 {
    fn first_challenge(&self) -> String {
        self.input
            .iter()
            .map(|group| {
                (0..QUESTIONS)
                    .filter(|&question| group.iter().any(|person| person[question]))
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn second_challenge(&self) -> String {
        self.input
            .iter()
            .map(|group| {
                (0..QUESTIONS)
                    .filter(|&question| group.iter().all(|person| person[question]))
                    .count()
            })
            .sum::<usize>()
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

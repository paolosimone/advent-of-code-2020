use super::Day;

pub struct Day10 {
    input: Vec<i64>,
}

impl Day10 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> Vec<i64> {
        let mut vec = s
            .lines()
            .map(|l| l.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        vec.push(0);
        vec.push(vec.iter().max().unwrap() + 3);

        vec.sort_unstable();
        vec
    }
}

impl Day for Day10 {
    fn first_challenge(&self) -> String {
        let (ones, threes) =
            self.input
                .windows(2)
                .map(|w| w[1] - w[0])
                .fold((0, 0), |(ones, threes), diff| match diff {
                    1 => (ones + 1, threes),
                    3 => (ones, threes + 1),
                    _ => (ones, threes),
                });

        (ones * threes).to_string()
    }

    fn second_challenge(&self) -> String {
        let mut combinations = vec![0u64; self.input.len()];

        combinations[0] = 1;
        for i in 1..self.input.len() {
            for j in i.saturating_sub(3)..i {
                if self.input[i] - self.input[j] <= 3 {
                    combinations[i] += combinations[j];
                }
            }
        }

        combinations.last().unwrap().to_string()
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let day = Day10::load(input);
        assert_eq!(day.first_challenge(), "220");
    }

    #[test]
    fn test_second_challenge() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        let day = Day10::load(input);
        assert_eq!(day.second_challenge(), "8");
    }

    #[test]
    fn test_second_challenge_large() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let day = Day10::load(input);
        assert_eq!(day.second_challenge(), "19208");
    }
}

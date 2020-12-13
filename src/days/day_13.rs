use num::Integer;

use super::Day;

pub struct Day13 {
    target: usize,
    buses: Vec<Option<usize>>,
}

impl Day13 {
    pub fn load(input: &str) -> Self {
        let target = input.lines().next().unwrap().parse::<usize>().unwrap();
        let buses = input
            .lines()
            .skip(1)
            .next()
            .unwrap()
            .split(',')
            .map(|id| id.parse::<usize>().ok())
            .collect::<Vec<_>>();
        Self { target, buses }
    }
}

impl Day for Day13 {
    fn first_challenge(&self) -> String {
        let mut deltas = self
            .buses
            .iter()
            .filter_map(|&maybe_id| maybe_id)
            .map(|id| {
                let next_trip = self.target / id + (self.target % id != 0) as usize;
                let next_trip_time = next_trip * id;
                (id, next_trip_time - self.target)
            })
            .collect::<Vec<_>>();

        deltas.sort_by_key(|&(_, delta)| delta);

        deltas
            .first()
            .map(|(id, delta)| id * delta)
            .unwrap()
            .to_string()
    }

    // For this one I have to thank an old friend: I owe you another one... Wolfram Alpha!
    //
    // Idea:
    // -> find res s.t. {res % id_0 = 0; (res + i) % id_i = 0}
    // -> note that res has period lcm(id_0, id_i)
    // -> rinse and repeat with next i
    fn second_challenge(&self) -> String {
        let (result, _) = self
            .buses
            .iter()
            .enumerate()
            .filter_map(|(delta, maybe_id)| {
                maybe_id.map(|id| {
                    // (res + delta) % id = 0 -> res % id = remainder
                    let iid = id as isize;
                    let remainder = ((iid + (iid - delta as isize) % iid) % iid) as usize;
                    (id, remainder)
                })
            })
            .fold((0, 1), |(result, period), (id, remainder)| {
                // solve equation by brute force... I can feel the disapproval of my college math teacher :(
                let new_result = (0..=id)
                    .map(|i| result + period * i)
                    .find(|res| res % id == remainder)
                    .unwrap();
                let new_period = period.lcm(&id);
                (new_result, new_period)
            });

        result.to_string()
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "939
7,13,x,x,59,x,31,19";
        let day = Day13::load(input);
        assert_eq!(day.first_challenge(), "295");
    }

    #[test]
    fn test_second_challenge_1() {
        let input = "939
7,13,x,x,59,x,31,19";
        let day = Day13::load(input);
        assert_eq!(day.second_challenge(), "1068781");
    }

    #[test]
    fn test_second_challenge_2() {
        let input = "939
17,x,13,19";
        let day = Day13::load(input);
        assert_eq!(day.second_challenge(), "3417");
    }

    #[test]
    fn test_second_challenge_3() {
        let input = "939
67,7,59,61";
        let day = Day13::load(input);
        assert_eq!(day.second_challenge(), "754018");
    }
}

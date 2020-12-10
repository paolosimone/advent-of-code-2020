use itertools::Itertools;

use super::Day;

pub struct Day08 {
    input: Program,
}

type Program = Vec<Instruction>;
type Instruction = (Op, i32);
#[derive(Copy, Clone)]
enum Op {
    Nop,
    Acc,
    Jmp,
}
type State = (usize, i32);

impl Day08 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> Program {
        s.lines().map(Self::parse_instruction).collect()
    }

    fn parse_instruction(s: &str) -> Instruction {
        let (op, arg) = s.splitn(2, ' ').collect_tuple().unwrap();
        (Self::parse_op(op), arg.parse::<i32>().unwrap())
    }

    fn parse_op(s: &str) -> Op {
        match s {
            "nop" => Op::Nop,
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            _ => panic!("Invalid operation!"),
        }
    }

    fn next(&self, state: State) -> State {
        if self.is_final(&state) {
            return state;
        }

        let (i, value) = state;
        match &self.input[i] {
            (Op::Acc, acc) => (i + 1, value + acc),
            (Op::Jmp, jmp) => ((i as i32 + jmp) as usize, value),
            _ => (i + 1, value),
        }
    }

    fn is_final(&self, (i, _): &State) -> bool {
        *i >= self.input.len()
    }

    fn find_cycle_start(&self) -> State {
        // https://en.wikipedia.org/wiki/Cycle_detection#Floyd's_Tortoise_and_Hare
        let mut slow = self.next((0, 0));
        let mut fast = self.next(self.next((0, 0)));

        while fast.0 != slow.0 {
            slow = self.next(slow);
            fast = self.next(self.next(fast));
        }

        slow = (0, 0);
        while fast.0 != slow.0 {
            slow = self.next(slow);
            fast = self.next(fast);
        }
        slow
    }

    fn switch((op, arg): Instruction) -> Instruction {
        match op {
            Op::Nop => (Op::Jmp, arg),
            Op::Jmp => (Op::Nop, arg),
            _ => (op, arg),
        }
    }
}

impl Day for Day08 {
    // O(N)
    fn first_challenge(&self) -> String {
        let cycle_start = self.find_cycle_start();

        // continue until we reach cycle start again
        let mut state = cycle_start;
        state = self.next(state);
        while state.0 != cycle_start.0 {
            state = self.next(state);
        }

        state.1.to_string()
    }

    // O(N^2)
    fn second_challenge(&self) -> String {
        let to_switch = self
            .input
            .iter()
            .enumerate()
            .filter_map(|(i, (op, _))| match op {
                Op::Nop | Op::Jmp => Some(i),
                _ => None,
            })
            .collect::<Vec<usize>>();

        let mut mut_self = Self {
            input: self.input.to_vec(),
        };

        for i in to_switch {
            mut_self.input[i] = Self::switch(mut_self.input[i]);

            let cycle_start = mut_self.find_cycle_start();
            if mut_self.is_final(&cycle_start) {
                return cycle_start.1.to_string();
            }

            mut_self.input[i] = Self::switch(mut_self.input[i]);
        }

        "NOT_FOUND".to_string()
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let day = Day08::load(input);
        assert_eq!(day.first_challenge(), "5");
    }

    #[test]
    fn test_second_challenge() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let day = Day08::load(input);
        assert_eq!(day.second_challenge(), "8");
    }
}

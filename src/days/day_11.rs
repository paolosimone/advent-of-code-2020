use std::cmp::min;

use super::Day;

pub struct Day11 {
    input: State,
}

type State = Vec<Vec<Slot>>;
type StateSlice<'a> = &'a [Vec<Slot>];
type MutStateSlice<'a> = &'a mut [Vec<Slot>];

#[derive(Debug, PartialEq, Copy, Clone)]
enum Slot {
    Empty,
    Occupied,
    Floor,
}

type Coord = (usize, usize);
type Line = Vec<Coord>;
type LineSlice<'a> = &'a [Coord];

impl Day11 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> State {
        s.lines()
            .map(|l| l.chars().map(Self::parse_slot).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    fn parse_slot(c: char) -> Slot {
        match c {
            'L' => Slot::Empty,
            '#' => Slot::Occupied,
            '.' => Slot::Floor,
            _ => panic!("Invalid slot!"),
        }
    }

    fn update(state: MutStateSlice, occupied: &[Vec<usize>], threshold: usize) {
        for i in 0..state.len() {
            for j in 0..state[0].len() {
                if Self::should_update_slot(state[i][j], occupied[i][j], threshold) {
                    match state[i][j] {
                        Slot::Occupied => state[i][j] = Slot::Empty,
                        Slot::Empty => state[i][j] = Slot::Occupied,
                        _ => {}
                    }
                }
            }
        }
    }

    fn is_stable(state: MutStateSlice, occupied: &[Vec<usize>], threshold: usize) -> bool {
        for i in 0..state.len() {
            for j in 0..state[0].len() {
                if Self::should_update_slot(state[i][j], occupied[i][j], threshold) {
                    return false;
                }
            }
        }
        true
    }

    fn should_update_slot(slot: Slot, occupied: usize, threshold: usize) -> bool {
        match (slot, occupied) {
            (Slot::Occupied, count) => count >= threshold,
            (Slot::Empty, 0) => true,
            _ => false,
        }
    }

    fn count_occupied_adjacent(state: StateSlice) -> Vec<Vec<usize>> {
        state
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, _)| Self::count_occupied_adjacent_slot(state, i, j))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn count_occupied_adjacent_slot(state: StateSlice, i: usize, j: usize) -> usize {
        let range_i = i.saturating_sub(1)..min(i + 2, state.len());
        let range_j = j.saturating_sub(1)..min(j + 2, state[0].len());

        // uglier but faster than itertools::cartesian_product :/
        let mut res = 0;
        for ii in range_i {
            for jj in range_j.clone() {
                if (ii != i || jj != j) && state[ii][jj] == Slot::Occupied {
                    res += 1;
                }
            }
        }
        res
    }

    fn count_occupied_view(state: StateSlice, lines: &[Line]) -> Vec<Vec<usize>> {
        let (m, n) = (state.len(), state[0].len());
        let mut occupied = vec![vec![0; n]; m];
        lines
            .iter()
            .for_each(|line| Self::increment_occupied_view_line(state, &mut occupied, line));
        occupied
    }

    fn increment_occupied_view_line(
        state: StateSlice,
        occupied: &mut [Vec<usize>],
        line: LineSlice,
    ) {
        let mut view_occupied = false;
        for &(i, j) in line {
            if view_occupied {
                occupied[i][j] += 1;
            }

            view_occupied = match state[i][j] {
                Slot::Occupied => true,
                Slot::Empty => false,
                Slot::Floor => view_occupied,
            };
        }
    }

    fn generate_lines(state: StateSlice) -> Vec<Line> {
        let (m, n) = (state.len(), state[0].len());
        let mut lines = Vec::new();

        // rows
        for i in 0..m {
            let line = (0..n).map(|j| (i, j)).collect::<Vec<_>>();
            lines.push(line.clone());
            lines.push(line.into_iter().rev().collect());
        }

        // columns
        for j in 0..n {
            let line = (0..m).map(|i| (i, j)).collect::<Vec<_>>();
            lines.push(line.clone());
            lines.push(line.into_iter().rev().collect());
        }

        // diagonals
        for j in 0..n {
            let down = (0..min(n - j, m)).map(|d| (d, j + d)).collect::<Vec<_>>();
            lines.push(down.clone());
            lines.push(down.into_iter().rev().collect());

            let up = (0..min(n - j, m))
                .map(|d| (m - 1 - d, j + d))
                .collect::<Vec<_>>();
            lines.push(up.clone());
            lines.push(up.into_iter().rev().collect());
        }

        for i in 1..(m - 1) {
            let down = (0..min(n, m - i)).map(|d| (i + d, d)).collect::<Vec<_>>();
            lines.push(down.clone());
            lines.push(down.into_iter().rev().collect());

            let up = (0..min(n, i + 1)).map(|d| (i - d, d)).collect::<Vec<_>>();
            lines.push(up.clone());
            lines.push(up.into_iter().rev().collect());
        }

        lines
    }

    fn count_occupied(state: StateSlice) -> usize {
        state
            .iter()
            .map(|row| row.iter().filter(|&slot| slot == &Slot::Occupied))
            .flatten()
            .count()
    }
}

impl Day for Day11 {
    fn first_challenge(&self) -> String {
        let threshold = 4;
        let state = &mut self.input.to_vec();
        loop {
            let occupied = &Self::count_occupied_adjacent(state);
            if Self::is_stable(state, occupied, threshold) {
                return Self::count_occupied(&state).to_string();
            }
            Self::update(state, occupied, threshold);
        }
    }

    fn second_challenge(&self) -> String {
        let threshold = 5;
        let state = &mut self.input.to_vec();
        let lines = &Self::generate_lines(state);
        loop {
            let occupied = &Self::count_occupied_view(state, lines);
            if Self::is_stable(state, occupied, threshold) {
                return Self::count_occupied(&state).to_string();
            }
            Self::update(state, occupied, threshold);
        }
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let day = Day11::load(input);
        assert_eq!(day.first_challenge(), "37");
    }

    #[test]
    fn test_second_challenge() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let day = Day11::load(input);
        assert_eq!(day.second_challenge(), "26");
    }
}

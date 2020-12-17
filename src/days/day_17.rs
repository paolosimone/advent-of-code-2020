use std::cmp::min;

use itertools::Itertools;

use super::Day;

pub struct Day17 {
    input: Plane<Cube>,
}

impl Day17 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> Plane<Cube> {
        s.lines()
            .map(|l| l.chars().map(Self::parse_cube).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    fn parse_cube(c: char) -> Cube {
        match c {
            '#' => Cube::Active,
            '.' => Cube::Inactive,
            _ => panic!("Invalid cube!"),
        }
    }

    fn run(&self, iter: usize, enable_4d: bool) -> HyperSpace<Cube> {
        let expansion = 2 * iter;

        let size = (
            1 + if enable_4d { expansion } else { 0 },
            1 + expansion,
            self.input.len() + expansion,
            self.input[0].len() + expansion,
        );

        let mut state = HyperSpace::new(Cube::Inactive, size);

        // initialize middle plane
        for i in 0..self.input.len() {
            for j in 0..self.input[0].len() {
                let coord = (size.0 / 2, size.1 / 2, iter + i, iter + j);
                state.set(coord, self.input[i][j]);
            }
        }

        // run the simulation
        for _ in 0..iter {
            let active = &Self::count_active_adjacent(&state);
            Self::update(&mut state, active);
        }

        state
    }

    fn update(state: &mut HyperSpace<Cube>, active: &HyperSpace<usize>) {
        let to_update = state
            .coords
            .iter()
            .filter(|&coord| Self::should_update_cube(state.get(*coord), active.get(*coord)))
            .cloned()
            .collect::<Vec<_>>();

        to_update
            .into_iter()
            .for_each(|coord| match state.get(coord) {
                Cube::Active => state.set(coord, Cube::Inactive),
                Cube::Inactive => state.set(coord, Cube::Active),
            });
    }

    fn should_update_cube(cube: Cube, active: usize) -> bool {
        match (cube, active) {
            (Cube::Active, 2) => false,
            (Cube::Active, 3) => false,
            (Cube::Active, _) => true,

            (Cube::Inactive, 3) => true,
            (Cube::Inactive, _) => false,
        }
    }

    fn count_active_adjacent(state: &HyperSpace<Cube>) -> HyperSpace<usize> {
        let mut active = HyperSpace::new(0, state.size);
        for &coord in state.coords.iter() {
            active.set(coord, Self::count_active_adjacent_cube(state, coord));
        }
        active
    }

    fn count_active_adjacent_cube(state: &HyperSpace<Cube>, (t, k, i, j): Coord) -> usize {
        let range_t = t.saturating_sub(1)..min(t + 2, state.size.0);
        let range_k = k.saturating_sub(1)..min(k + 2, state.size.1);
        let range_i = i.saturating_sub(1)..min(i + 2, state.size.2);
        let range_j = j.saturating_sub(1)..min(j + 2, state.size.3);

        let mut res = 0;

        for tt in range_t {
            for kk in range_k.clone() {
                for ii in range_i.clone() {
                    for jj in range_j.clone() {
                        if state.get((tt, kk, ii, jj)) == Cube::Active {
                            res += 1;
                        }
                    }
                }
            }
        }

        if state.get((t, k, i, j)) == Cube::Active {
            res -= 1;
        }

        res
    }

    fn count_active(state: &HyperSpace<Cube>) -> usize {
        state
            .coords
            .iter()
            .filter(|&coord| state.get(*coord) == Cube::Active)
            .count()
    }
}

impl Day for Day17 {
    fn first_challenge(&self) -> String {
        let state = &self.run(6, false);
        Self::count_active(state).to_string()
    }

    fn second_challenge(&self) -> String {
        let state = &self.run(6, true);
        Self::count_active(state).to_string()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cube {
    Inactive,
    Active,
}

type Coord = (usize, usize, usize, usize);
type Size = (usize, usize, usize, usize);

type Plane<T> = Vec<Vec<T>>;
type Space<T> = Vec<Plane<T>>;

struct HyperSpace<T> {
    space: Vec<Space<T>>,
    coords: Vec<Coord>,
    size: Size,
}

impl<T> HyperSpace<T>
where
    T: Clone + Copy,
{
    fn new(value: T, (wsize, zsize, xsize, ysize): Size) -> HyperSpace<T> {
        let space = vec![vec![vec![vec![value; ysize]; xsize]; zsize]; wsize];

        let coords = (0..wsize)
            .cartesian_product(0..zsize)
            .cartesian_product(0..xsize)
            .cartesian_product(0..ysize)
            .map(|(((t, k), i), j)| (t, k, i, j))
            .collect();

        let size = (wsize, zsize, xsize, ysize);

        Self {
            space,
            coords,
            size,
        }
    }

    fn get(&self, (t, k, i, j): Coord) -> T {
        self.space[t][k][i][j]
    }

    fn set(&mut self, (t, k, i, j): Coord, value: T) {
        self.space[t][k][i][j] = value
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = ".#.
..#
###";
        let day = Day17::load(input);
        assert_eq!(day.first_challenge(), "112");
    }

    #[test]
    #[ignore]
    fn test_second_challenge() {
        let input = ".#.
..#
###";
        let day = Day17::load(input);
        assert_eq!(day.second_challenge(), "848");
    }
}

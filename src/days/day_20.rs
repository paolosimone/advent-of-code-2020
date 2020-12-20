use itertools::Itertools;

use super::Day;

pub struct Day20 {
    input: Vec<Tile>,
}

impl Day20 {
    pub fn load(input: &str) -> Self {
        let input = input.split("\n\n").map(Tile::from_str).collect::<Vec<_>>();

        Self { input }
    }
}

impl Day for Day20 {
    fn first_challenge(&self) -> String {
        let solution = part_1::rearrange(&self.input);
        let size = solution.len();

        let mul_corners = solution[0][0].id
            * solution[0][size - 1].id
            * solution[size - 1][size - 1].id
            * solution[size - 1][0].id;

        mul_corners.to_string()
    }

    fn second_challenge(&self) -> String {
        let solution = part_1::rearrange(&self.input);
        let merged = part_2::merge(&solution);
        let combinations = merged.generate_combinations();

        let (tile, dragons) = combinations
            .iter()
            .map(|tile| (tile, part_2::count_dragons(tile)))
            .max_by_key(|(_, count)| *count)
            .unwrap();

        let all_rough = tile
            .pixels
            .iter()
            .map(|row| row.iter().filter(|&pixel| *pixel).count())
            .sum::<usize>();

        let dragon_rough = (&*part_2::DRAGON)
            .iter()
            .map(|row| row.len())
            .sum::<usize>();

        let not_dragon = all_rough - dragons * dragon_rough;

        not_dragon.to_string()
    }
}

type Id = usize;
type Matrix<T> = Vec<Vec<T>>;
type MatrixSlice<'a, T> = &'a [Vec<T>];
type MatrixSliceMut<'a, T> = &'a mut [Vec<T>];

#[derive(Clone)]
struct Tile {
    id: Id,
    pixels: Matrix<bool>,
}

impl Tile {
    fn from_str(s: &str) -> Tile {
        let mut lines = s.lines();

        let id = lines.next().unwrap()[5..9].parse::<usize>().unwrap();

        let pixels = lines
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { id, pixels }
    }

    fn rotate(&self) -> Self {
        let size = self.pixels.len();
        let pixels = (0..size)
            .map(|j| (0..size).rev().map(|i| self.pixels[i][j]).collect_vec())
            .collect_vec();

        Self {
            id: self.id,
            pixels,
        }
    }

    fn flip(&self) -> Self {
        let size = self.pixels.len();
        let pixels = (0..size)
            .map(|i| (0..size).rev().map(|j| self.pixels[i][j]).collect_vec())
            .collect_vec();

        Self {
            id: self.id,
            pixels,
        }
    }

    fn generate_combinations(&self) -> Vec<Tile> {
        let mut combinations = Vec::new();

        let mut comb = self.clone();
        for _ in 0..4 {
            combinations.push(comb.clone());
            comb = comb.rotate();
        }

        comb = self.flip();
        for _ in 0..4 {
            combinations.push(comb.clone());
            comb = comb.rotate();
        }

        combinations
    }
}

mod part_1 {
    use std::collections::{HashMap, HashSet};

    use itertools::Itertools;
    use num::integer::Roots;

    use super::{Id, Matrix, MatrixSlice, MatrixSliceMut, Tile};

    type Edge = usize;
    type EdgeIndex = HashMap<Edge, HashSet<IdComb>>;
    type OutlineIndex = Outline<EdgeIndex>;

    type CombinationMap = HashMap<Id, Vec<Tile>>;
    type OutlineMap = HashMap<IdComb, Outline<Edge>>;

    type Comb = usize;
    type IdComb = (Id, Comb);
    type Coord = (usize, usize);

    #[derive(Debug, Clone)]
    struct Outline<T> {
        top: T,
        right: T,
        bottom: T,
        left: T,
    }

    impl<T> Outline<T> {
        fn get(&self, dir: &Dir) -> &T {
            match dir {
                Dir::Top => &self.top,
                Dir::Right => &self.right,
                Dir::Bottom => &self.bottom,
                Dir::Left => &self.left,
            }
        }
    }

    #[derive(Debug, Clone)]
    enum Dir {
        Top,
        Right,
        Bottom,
        Left,
    }

    pub(super) fn rearrange(tiles: &[Tile]) -> Matrix<Tile> {
        // all orientations (combinations) by tile id
        let combinations_by_id = group_combinations_by_id(tiles);

        // precompute combination edges
        let outlines_by_id = group_outlines_by_id(&combinations_by_id);

        // inverted index to search combinations with matching edges
        let outline_index = Outline {
            top: group_ids_by_edge(&combinations_by_id, &outlines_by_id, Dir::Top),
            right: group_ids_by_edge(&combinations_by_id, &outlines_by_id, Dir::Right),
            bottom: group_ids_by_edge(&combinations_by_id, &outlines_by_id, Dir::Bottom),
            left: group_ids_by_edge(&combinations_by_id, &outlines_by_id, Dir::Left),
        };

        // backtrack
        let size = tiles.len().sqrt();
        let empty_solution = &mut vec![vec![(0, 0); size]; size];
        let used = &mut HashSet::new();
        let solution = backtrack(
            empty_solution,
            (0, 0),
            used,
            &combinations_by_id,
            &outlines_by_id,
            &outline_index,
        )
        .unwrap();

        // build rearrenged tiles
        solution
            .iter()
            .map(|row| {
                row.iter()
                    .map(|(id, comb)| combinations_by_id.get(id).unwrap()[*comb].clone())
                    .collect_vec()
            })
            .collect_vec()
    }

    fn backtrack(
        solution: MatrixSliceMut<IdComb>,
        (i, j): Coord,
        used: &mut HashSet<Id>,
        combinations_by_id: &CombinationMap,
        outlines_by_id: &OutlineMap,
        outline_index: &OutlineIndex,
    ) -> Option<Matrix<IdComb>> {
        if i >= solution.len() {
            return Some(solution.to_vec());
        }

        // compute next tile position
        let next = Some(j + 1)
            .filter(|&j| j < solution.len())
            .map(|j| (i, j))
            .unwrap_or((i + 1, 0));

        // compute combinations matching both up and left
        let empty_set = &HashSet::new();

        let matching_up = i.checked_sub(1).map(|up_i| {
            let up_id = solution[up_i][j];
            let up_edge = outlines_by_id.get(&up_id).unwrap().bottom;

            outline_index
                .get(&Dir::Top)
                .get(&up_edge)
                .unwrap_or(empty_set)
        });

        let matching_left = j.checked_sub(1).map(|left_j| {
            let left_id = solution[i][left_j];
            let left_edge = outlines_by_id.get(&left_id).unwrap().right;

            outline_index
                .get(&Dir::Left)
                .get(&left_edge)
                .unwrap_or(empty_set)
        });

        // consider unused tiles
        let remaining_ids = combinations_by_id
            .keys()
            .filter(|id| !used.contains(id))
            .cloned()
            .collect_vec();

        for id in remaining_ids {
            // keep only combinations matching both up and left
            let valid_combinations = combinations_by_id
                .get(&id)
                .unwrap()
                .iter()
                .enumerate()
                .map(|(comb, _)| (id, comb))
                .filter(|pair| {
                    let ok_up = matching_up
                        .map(|matches| matches.contains(pair))
                        .unwrap_or(true);

                    let ok_left = matching_left
                        .map(|matches| matches.contains(pair))
                        .unwrap_or(true);

                    ok_up && ok_left
                })
                .collect_vec();

            // try and see if we reach a valid solution
            used.insert(id);
            for comb in valid_combinations {
                solution[i][j] = comb;

                let complete_solution = backtrack(
                    solution,
                    next,
                    used,
                    combinations_by_id,
                    outlines_by_id,
                    outline_index,
                );

                if let Some(solution) = complete_solution {
                    return Some(solution);
                }
            }
            used.remove(&id);
        }

        // no valid solution found -> backtrack
        None
    }

    fn group_combinations_by_id(tiles: &[Tile]) -> CombinationMap {
        tiles
            .iter()
            .map(|tile| (tile.id, tile.generate_combinations()))
            .collect::<HashMap<_, _>>()
    }

    fn group_ids_by_edge(
        combinations_by_id: &CombinationMap,
        outlines_by_id: &OutlineMap,
        dir: Dir,
    ) -> EdgeIndex {
        let mut ids_by_edge: EdgeIndex = HashMap::new();

        for (&id, tiles) in combinations_by_id.iter() {
            for comb in 0..tiles.len() {
                let edge = outlines_by_id.get(&(id, comb)).unwrap().get(&dir);

                match ids_by_edge.get_mut(edge) {
                    Some(ids) => {
                        ids.insert((id, comb));
                    }

                    None => {
                        let mut ids = HashSet::new();
                        ids.insert((id, comb));
                        ids_by_edge.insert(*edge, ids);
                    }
                }
            }
        }

        ids_by_edge
    }

    fn group_outlines_by_id(combinations_by_id: &CombinationMap) -> OutlineMap {
        combinations_by_id
            .iter()
            .flat_map(|(&id, combinations)| {
                combinations
                    .iter()
                    .enumerate()
                    .map(|(comb, tile)| ((id, comb), compute_outline(&tile.pixels)))
                    .collect_vec()
            })
            .collect::<HashMap<_, _>>()
    }

    fn compute_outline(pixels: MatrixSlice<bool>) -> Outline<Edge> {
        let size = pixels.len();
        let top = (0..size).fold(0, |acc, j| (acc << 1) + pixels[0][j] as usize);
        let bottom = (0..size).fold(0, |acc, j| (acc << 1) + pixels[size - 1][j] as usize);
        let left = (0..size).fold(0, |acc, i| (acc << 1) + pixels[i][0] as usize);
        let right = (0..size).fold(0, |acc, i| (acc << 1) + pixels[i][size - 1] as usize);

        Outline {
            top,
            right,
            bottom,
            left,
        }
    }
}

mod part_2 {
    use itertools::Itertools;

    use super::{MatrixSlice, Tile};

    lazy_static! {
        pub(super) static ref DRAGON: [Vec<usize>; 3] = [
            vec![18],
            vec![0, 5, 6, 11, 12, 17, 18, 19],
            vec![1, 4, 7, 10, 13, 16],
        ];
    }

    const DRAGON_LENGHT: usize = 20;

    pub(super) fn merge(tiles: MatrixSlice<Tile>) -> Tile {
        let size = tiles[0][0].pixels.len();
        let new_size = tiles.len() * (size - 2);
        let mut pixels = vec![vec![false; new_size]; new_size];
        for ti in 0..tiles.len() {
            for tj in 0..tiles[0].len() {
                for i in 1..(size - 1) {
                    for j in 1..(size - 1) {
                        let pi = ti * (size - 2) + i - 1;
                        let pj = tj * (size - 2) + j - 1;
                        pixels[pi][pj] = tiles[ti][tj].pixels[i][j];
                    }
                }
            }
        }

        Tile { id: 0, pixels }
    }

    pub(super) fn count_dragons(tile: &Tile) -> usize {
        let size = tile.pixels[0].len();

        tile.pixels
            .windows(DRAGON.len())
            .map(|candidate| {
                (0..(size - DRAGON_LENGHT))
                    .filter(|&offset| is_dragon(candidate, offset))
                    .count()
            })
            .sum::<usize>()
    }

    fn is_dragon(candidate: MatrixSlice<bool>, offset: usize) -> bool {
        candidate
            .iter()
            .zip_eq(&*DRAGON)
            .all(|(candidate_row, dragon_row)| dragon_row.iter().all(|i| candidate_row[i + offset]))
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let day = Day20::load(INPUT);
        assert_eq!(day.first_challenge(), "20899048083289");
    }

    #[test]
    fn test_second_challenge() {
        let day = Day20::load(INPUT);
        assert_eq!(day.second_challenge(), "273");
    }

    const INPUT: &'static str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
}

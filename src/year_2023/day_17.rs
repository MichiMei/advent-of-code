use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use crate::errors::AoCError;
use crate::geometrics::{Direction, Grid};

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let grid = Grid::parse_digits(input)?;
    let start = (0, 0);
    let size = grid.get_dimension();
    let end = (size.0-1, size.1-1);
    assert!(grid.get_tile(&end).is_some());
    grid.get_shortest_path(start, end, 0, 3)
        .map(|v| v.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let grid = Grid::parse_digits(input)?;
    let start = (0, 0);
    let size = grid.get_dimension();
    let end = (size.0-1, size.1-1);
    assert!(grid.get_tile(&end).is_some());
    grid.get_shortest_path(start, end, 4, 10)
        .map(|v| v.to_string())
}

type Point = crate::geometrics::Point<usize>;

impl Grid<u8> {
    fn get_shortest_path(&self, start: Point, end: Point, min_to_turn: usize, max_to_turn: usize)
        -> Result<usize, AoCError<String>>
    {
        let mut cache = HashMap::new();
        let mut heap = BinaryHeap::new();
        let path_north = Path{ pos: start, dir: Direction::North, steps: 0, cooling: 0,
            min_to_turn, max_to_turn };
        heap.push(Reverse(path_north));
        let path_east = Path{ pos: start, dir: Direction::East, steps: 0, cooling: 0,
            min_to_turn, max_to_turn };
        heap.push(Reverse(path_east));
        let path_south = Path{ pos: start, dir: Direction::South, steps: 0, cooling: 0,
            min_to_turn, max_to_turn };
        heap.push(Reverse(path_south));
        let path_west = Path{ pos: start, dir: Direction::West, steps: 0, cooling: 0,
            min_to_turn, max_to_turn };
        heap.push(Reverse(path_west));
        while let Some(path) = heap.pop() {
            let path = path.0;
            if let Some(value) = path.get_from_cache(&cache) {
                assert!(value <= path.cooling);
//                println!("\tskipped {:?}", path);
                continue;
            }
//            println!("reached {:?}", path);
            path.put_into_cache(&mut cache);
            //assert!(path.cooling < 60);
            //assert!(path.cooling < 103);
            if path.pos == end && path.ready_to_stop() {
                return Ok(path.cooling)
            }
            path.get_possible(self)
                .into_iter()
                .filter(|path| {
                    if let Some(s) = path.get_from_cache(&cache) {
                        assert!(s <= path.cooling);
                        false
                    } else {
                        true
                    }
                })
                .for_each(|elem| heap.push(Reverse(elem)));
        }
        Err(AoCError::NoSolutionFoundError(
            format!("Could not find path from ({}, {}) to ({}, {})",
                    start.0, start.1, end.0, end.1)))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Path {
    pos: Point,
    dir: Direction,
    steps: usize,
    cooling: usize,
    min_to_turn: usize,
    max_to_turn: usize,
}

impl Path {
    fn get_possible(&self, grid: &Grid<u8>) -> Vec<Self> {
        let mut res = vec![];
        if let Some(path) = self.step_forward(grid) {
            res.push(path);
        }
        if let Some(path) = self.step_right(grid) {
            res.push(path);
        }
        if let Some(path) = self.step_left(grid) {
            res.push(path);
        }
        res
    }

    fn step_forward(&self, grid: &Grid<u8>) -> Option<Self> {
        if self.steps >= self.max_to_turn {
            return None
        }
        if let Some(point) = self.dir.move_point(&self.pos) {
            grid.get_tile(&point).map(|&value| Self {
                    pos: point,
                    dir: self.dir,
                    steps: self.steps+1,
                    cooling: self.cooling + (value as usize),
                    min_to_turn: self.min_to_turn,
                    max_to_turn: self.max_to_turn,
                })
        } else {
            None
        }
    }

    fn step_right(&self, grid: &Grid<u8>) -> Option<Self> {
        if self.steps < self.min_to_turn {
            return None
        }
        let dir = self.dir.get_right();
        if let Some(point) = dir.move_point(&self.pos) {
            grid.get_tile(&point).map(|&value| Self {
                    pos: point,
                    dir,
                    steps: 1,
                    cooling: self.cooling + (value as usize),
                    min_to_turn: self.min_to_turn,
                    max_to_turn: self.max_to_turn,
                })
        } else {
            None
        }
    }

    fn step_left(&self, grid: &Grid<u8>) -> Option<Self> {
        if self.steps < self.min_to_turn {
            return None
        }
        let dir = self.dir.get_left();
        if let Some(point) = dir.move_point(&self.pos) {
            grid.get_tile(&point).map(|&value| Self {
                    pos: point,
                    dir,
                    steps: 1,
                    cooling: self.cooling + (value as usize),
                    min_to_turn: self.min_to_turn,
                    max_to_turn: self.max_to_turn,
                })
        } else {
            None
        }
    }

    fn put_into_cache(&self, cache: &mut HashMap<(Point, Direction, usize), usize>) {
        cache.insert((self.pos, self.dir, self.steps), self.cooling);
    }

    fn get_from_cache(&self, cache: &HashMap<(Point, Direction, usize), usize>) -> Option<usize> {
        cache.get(&(self.pos, self.dir, self.steps)).copied()
    }

    fn ready_to_stop(&self) -> bool {
        assert!(self.steps <= self.max_to_turn);
        self.steps > self.min_to_turn
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cooling.cmp(&other.cooling))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cooling.cmp(&other.cooling)
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "2413432311323".to_string(),
            "3215453535623".to_string(),
            "3255245654254".to_string(),
            "3446585845452".to_string(),
            "4546657867536".to_string(),
            "1438598798454".to_string(),
            "4457876987766".to_string(),
            "3637877979653".to_string(),
            "4654967986887".to_string(),
            "4564679986453".to_string(),
            "1224686865563".to_string(),
            "2546548887735".to_string(),
            "4322674655533".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("102".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 17)?;
        assert_eq!(part_1(&input), Ok("959".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("94".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 17)?;
        assert_eq!(part_2(&input), Ok("1135".to_string()));
        Ok(())
    }
}
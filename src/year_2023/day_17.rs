use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let grid = Grid::parse(input)?;
    let start = (0, 0);
    let end = (grid.grid[0].len()-1, grid.grid.len()-1);
    assert!(grid.get_value(&end).is_some());
    grid.get_shortest_path(start, end, 0, 3)
        .map(|v| v.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let grid = Grid::parse(input)?;
    let start = (0, 0);
    let end = (grid.grid[0].len()-1, grid.grid.len()-1);
    assert!(grid.get_value(&end).is_some());
    grid.get_shortest_path(start, end, 4, 10)
        .map(|v| v.to_string())
}

struct Grid {
    grid: Vec<Vec<u8>>
}

impl Grid {
    fn parse(input: &[String]) -> Result<Self, AoCError<String>> {
        if input.is_empty() {
            return Err(AoCError::UnexpectedInputLength("Input cannot be empty".to_string()))
        }
        let mut grid = Vec::with_capacity(input.len());
        let width = input[0].len();
        for line in input {
            if width != line.len() {
                return Err(AoCError::BadInputFormat(
                    "Lines need to have same number of digits.".to_string()))
            }
            let row = line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| (c as u8) - b'0')
                .collect::<Vec<_>>();
            if width != row.len() {
                return Err(AoCError::BadInputFormat(
                    "Input can only contain digits '0' to '9'".to_string()))
            }
            grid.push(row);
        }
        Ok(Self { grid })
    }

    fn get_value(&self, pos: &Point) -> Option<u8> {
        if let Some(row) = self.grid.get(pos.1) {
            row.get(pos.0).copied()
        } else {
            None
        }
    }

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
    fn get_possible(&self, grid: &Grid) -> Vec<Self> {
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

    fn step_forward(&self, grid: &Grid) -> Option<Self> {
        if self.steps >= self.max_to_turn {
            return None
        }
        if let Some(point) = self.dir.move_point(&self.pos) {
            grid.get_value(&point).map(|value| Self {
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

    fn step_right(&self, grid: &Grid) -> Option<Self> {
        if self.steps < self.min_to_turn {
            return None
        }
        let dir = self.dir.get_right();
        if let Some(point) = dir.move_point(&self.pos) {
            grid.get_value(&point).map(|value| Self {
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

    fn step_left(&self, grid: &Grid) -> Option<Self> {
        if self.steps < self.min_to_turn {
            return None
        }
        let dir = self.dir.get_left();
        if let Some(point) = dir.move_point(&self.pos) {
            grid.get_value(&point).map(|value| Self {
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

type Point = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn move_point(&self, point: &Point) -> Option<Point> {
        match self {
            Direction::North => {
                if point.1 == 0 {
                    None
                } else {
                    Some((point.0, point.1-1))
                }
            }
            Direction::East => {
                Some((point.0+1, point.1))
            }
            Direction::South => {
                Some((point.0, point.1+1))
            }
            Direction::West => {
                if point.0 == 0 {
                    None
                } else {
                    Some((point.0-1, point.1))
                }
            }
        }
    }

    fn get_right(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
        }
    }

    fn get_left(&self) -> Self {
        match self {
            Direction::North => Self::West,
            Direction::East => Self::North,
            Direction::South => Self::East,
            Direction::West => Self::South,
        }
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
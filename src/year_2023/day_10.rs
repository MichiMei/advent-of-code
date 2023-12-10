use std::collections::HashSet;
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let mut grid = Grid::parse(input)?;
    grid.get_loop_length().map(|len| (len/2).to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let mut grid = Grid::parse(input)?;
    let path = grid.get_path()?;
    grid.remove_irrelevant(&path);
    Ok(grid.count_enclosed().to_string())
}

struct Grid {
    grid: Vec<Vec<Tile>>,
    size: Point,
}

impl Grid {
    fn parse(input: &Vec<String>) -> Result<Self, AoCError<String>> {
        if input.is_empty() {
            return Err(AoCError::UnexpectedInputLength(
                "Input is empty, needs to be at least one line".to_string()))
        }
        let grid = input.iter().map(|line| {
            line.chars()
                .map(Tile::parse)
                .collect::<Result<Vec<_>, _>>()
        })
            .collect::<Result<Vec<_>, _>>()?;
        let size = (grid[0].len(), grid.len());
        assert_eq!(grid.iter().filter(|row| row.len() != size.0).count(), 0);
        Ok(Self{grid, size})
    }

    fn get_start(&self) -> Result<Point, AoCError<String>> {
        let mut start = None;
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, tile) in row.iter().enumerate() {
                if *tile == Tile::Start {
                    if start.is_some() {
                        return Err(AoCError::NoSolutionFoundError(
                            "Multiple starts in input".to_string()))
                    } else {
                        start = Some((col_index, row_index));
                    }
                }
            }
        }
        start.ok_or_else(|| AoCError::NoSolutionFoundError("No start in input".to_string()))
    }

    fn get_loop_length(&mut self) -> Result<usize, AoCError<String>> {
        self.get_path().map(|path| path.len())
    }

    fn get_path(&mut self) -> Result<HashSet<Point>, AoCError<String>> {
        let start = self.get_start()?;
        let mut result = None;
        let mut start_directions = None;
        let mut remaining = HashSet::from([
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]);
        while remaining.len() >= 2 {
            let direction = *remaining.iter().next()
                .expect("Set is not empty");
            remaining.remove(&direction);
            if let Some((length, arrival_direction)) =
                self.check_loop(start, direction) {
                if result.is_some() {
                    return Err(AoCError::MultipleSolutionsFoundError(
                        "Found two circles from start".to_string()))
                }
                result = Some(length);
                start_directions = Some((direction, arrival_direction));
                assert_ne!(direction, arrival_direction);
                remaining.remove(&arrival_direction);
            }
        }
        if let Some((d0, d1)) = start_directions {
            self.grid[start.1][start.0] = Tile::Pipe(d0,d1);
        }
        result.ok_or_else(|| AoCError::NoSolutionFoundError("Found no circle".to_string()))
    }

    fn check_loop(&self, start: Point, direction: Direction)
        -> Option<(HashSet<Point>, Direction)> {
        let mut curr_pos = start;
        let mut curr_dir = direction;
        let mut path = HashSet::from([start]);
        loop {
            curr_pos = curr_dir.follow(curr_pos, self.size)?;
            if curr_pos == start {
                break
            }
            let next_tile = self.get_tile(curr_pos)
                .expect("Only valid positions are returned");
            curr_dir = next_tile.get_other_direction(curr_dir)?;
            path.insert(curr_pos);
        }
        Some((path, curr_dir.get_opposing()))
    }

    fn get_tile(&self, position: Point) -> Option<Tile> {
        if position.0 < self.size.0 || position.1 < self.size.1 {
            return Some(self.grid[position.1][position.0])
        }
        None
    }

    fn remove_irrelevant(&mut self, path: &HashSet<Point>) {
        for (row_index, row) in self.grid.iter_mut().enumerate() {
            for (col_index, tile) in row.iter_mut().enumerate() {
                if !path.contains(&(col_index, row_index)) {
                    *tile = Tile::Empty;
                }
            }
        }
    }

    fn count_enclosed(&self) -> usize {
        let mut count = 0;
        for row in self.grid.iter() {
            let mut enclosed = false;
            for tile in row.iter() {
                if *tile == Tile::Empty && enclosed {
                    count += 1;
                }
                if tile.should_flip() {
                    enclosed = !enclosed;
                }
            }
        }
        count
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Pipe(Direction, Direction),
    Empty,
    Start,
}

impl Tile {
    fn parse(c: char) -> Result<Self, AoCError<String>> {
        match c {
            '.' => Ok(Self::Empty),
            'S' => Ok(Self::Start),
            c => {
                let (d0, d1) = Direction::parse(c)?;
                Ok(Self::Pipe(d0, d1))
            }
        }
    }

    /// Returns the direction the tile leads given the previous direction.
    /// The given direction is the leaving one from the previous tile, therefore is opposing to
    /// the arriving one of this tile
    /// Returns None, if the tile has no (unique) leaving direction or the arrival direction is
    /// wrong
    fn get_other_direction(&self, leaving: Direction) -> Option<Direction> {
        let arriving = leaving.get_opposing();
        match self {
            Tile::Pipe(d0, d1) => {
                if *d0 == arriving {
                    return Some(*d1)
                }
                if *d1 == arriving {
                    return Some(*d0)
                }
                None
            }
            Tile::Empty => None,
            Tile::Start => None,
        }
    }

    fn should_flip(&self) -> bool {
        match self {
            Tile::Pipe(d0, d1) =>
                *d0 == Direction::North || *d1 == Direction::North,
            Tile::Empty => false,
            Tile::Start => false,
        }
    }
}

type Point = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn parse(c: char) -> Result<(Self, Self), AoCError<String>> {
        match c {
            '|' => Ok((Self::North, Self::South)),
            '-' => Ok((Self::East, Self::West)),
            'F' => Ok((Self::East, Self::South)),
            '7' => Ok((Self::South, Self::West)),
            'L' => Ok((Self::North, Self::East)),
            'J' => Ok((Self::North, Self::West)),
            _ => Err(AoCError::BadInputFormat(
                format!("Only chars '|', '-', 'L', 'F', '7', 'J', 'S' and '.' supported, \
                found '{}'", c))),
        }
    }

    fn follow(&self, position: Point, size: Point) -> Option<Point> {
        match self {
            Direction::North => {
                if position.1 == 0 {
                    None
                } else {
                    Some((position.0, position.1-1))
                }
            }
            Direction::East => {
                if position.0+1 == size.0 {
                    None
                } else {
                    Some((position.0+1, position.1))
                }
            }
            Direction::South => {
                if position.1+1 == size.1 {
                    None
                } else {
                    Some((position.0, position.1+1))
                }
            }
            Direction::West => {
                if position.0 == 0 {
                    None
                } else {
                    Some((position.0-1, position.1))
                }
            }
        }
    }

    fn get_opposing(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_1_input() -> Vec<String> {
        vec![
            ".....".to_string(),
            ".S-7.".to_string(),
            ".|.|.".to_string(),
            ".L-J.".to_string(),
            ".....".to_string(),
        ]
    }

    fn get_example_2_input() -> Vec<String> {
        vec![
            "-L|F7".to_string(),
            "7S-7|".to_string(),
            "L|7||".to_string(),
            "-L-J|".to_string(),
            "L|-JF".to_string(),
        ]
    }

    fn get_example_3_input() -> Vec<String> {
        vec![
            "..F7.".to_string(),
            ".FJ|.".to_string(),
            "SJ.L7".to_string(),
            "|F--J".to_string(),
            "LJ...".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_1_input();
        assert_eq!(part_1(&input), Ok("4".to_string()));
        let input = get_example_2_input();
        assert_eq!(part_1(&input), Ok("4".to_string()));
        let input = get_example_3_input();
        assert_eq!(part_1(&input), Ok("8".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 10)?;
        assert_eq!(part_1(&input), Ok("6860".to_string()));
        Ok(())
    }

    fn get_example_4_input() -> Vec<String> {
        vec![
            "...........".to_string(),
            ".S-------7.".to_string(),
            ".|F-----7|.".to_string(),
            ".||.....||.".to_string(),
            ".||.....||.".to_string(),
            ".|L-7.F-J|.".to_string(),
            ".|..|.|..|.".to_string(),
            ".L--J.L--J.".to_string(),
            "...........".to_string(),
        ]
    }

    fn get_example_5_input() -> Vec<String> {
        vec![
            "..........".to_string(),
            ".S------7.".to_string(),
            ".|F----7|.".to_string(),
            ".||....||.".to_string(),
            ".||....||.".to_string(),
            ".|L-7F-J|.".to_string(),
            ".|..||..|.".to_string(),
            ".L--JL--J.".to_string(),
            "..........".to_string(),
        ]
    }

    fn get_example_6_input() -> Vec<String> {
        vec![
            ".F----7F7F7F7F-7....".to_string(),
            ".|F--7||||||||FJ....".to_string(),
            ".||.FJ||||||||L7....".to_string(),
            "FJL7L7LJLJ||LJ.L-7..".to_string(),
            "L--J.L7...LJS7F-7L7.".to_string(),
            "....F-J..F7FJ|L7L7L7".to_string(),
            "....L7.F7||L7|.L7L7|".to_string(),
            ".....|FJLJ|FJ|F7|.LJ".to_string(),
            "....FJL-7.||.||||...".to_string(),
            "....L---J.LJ.LJLJ...".to_string(),
        ]
    }

    fn get_example_7_input() -> Vec<String> {
        vec![
            "FF7FSF7F7F7F7F7F---7".to_string(),
            "L|LJ||||||||||||F--J".to_string(),
            "FL-7LJLJ||||||LJL-77".to_string(),
            "F--JF--7||LJLJ7F7FJ-".to_string(),
            "L---JF-JLJ.||-FJLJJ7".to_string(),
            "|F|F-JF---7F7-L7L|7|".to_string(),
            "|FFJF7L7F-JF7|JL---7".to_string(),
            "7-L-JL7||F7|L7F-7F7|".to_string(),
            "L.L7LFJ|||||FJL7||LJ".to_string(),
            "L7JLJL-JLJLJL--JLJ.L".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_4_input();
        assert_eq!(part_2(&input), Ok("4".to_string()));
        let input = get_example_5_input();
        assert_eq!(part_2(&input), Ok("4".to_string()));
        let input = get_example_6_input();
        assert_eq!(part_2(&input), Ok("8".to_string()));
        let input = get_example_7_input();
        assert_eq!(part_2(&input), Ok("10".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 10)?;
        assert_eq!(part_2(&input), Ok("343".to_string()));
        Ok(())
    }
}
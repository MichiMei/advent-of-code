use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let grid = Grid::parse(input)?;
    let start = find_start(&grid)?;
    let dir = Direction::Down;
    follow_path(&grid, start, dir).map(|(waypoints, _)| waypoints)
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let grid = Grid::parse(input)?;
    let start = find_start(&grid)?;
    let dir = Direction::Down;
    follow_path(&grid, start, dir).map(|(_, count)| count.to_string())
}

fn find_start(grid: &Grid) -> Result<Point, AoCError<String>> {
    if grid.grid.is_empty() {
        return Err(AoCError::NoSolutionFoundError("Grid is empty.".to_string()))
    }
    let starts = grid.grid[0].iter()
        .enumerate()
        .filter(|(_, tile)| tile.is_path())
        .collect::<Vec<_>>();
    if starts.is_empty() {
        return Err(AoCError::NoSolutionFoundError("No path in first line found.".to_string()))
    }
    if starts.len() > 1 {
        return Err(AoCError::NoSolutionFoundError("Multiple paths in first line found.".to_string()))
    }
    Ok((starts[0].0, 0))
}

fn follow_path(grid: &Grid, start: Point, mut dir: Direction) -> Result<(String, usize), AoCError<String>> {
    let mut point = start;
    let mut waypoints = String::new();
    let mut count = 0;
    while let Some(tile) = grid.get_tile(point) {
        match tile {
            Tile::Empty => return Ok((waypoints, count)),
            Tile::Corner => dir = grid.get_new_direction(point, dir)
                .ok_or_else(|| AoCError::NoSolutionFoundError(format!(
                    "Path ended surprisingly after {:?} {:?}.", point, dir)))?,
            Tile::Path => {}
            Tile::Waypoint(wp) => waypoints = format!("{}{}", waypoints, wp),
        }
        point = dir.move_point(point)
            .ok_or_else(|| AoCError::NoSolutionFoundError(format!(
                "Path ended surprisingly after {:?} {:?}.", point, dir)))?;
        count += 1;

    }
    Err(AoCError::NoSolutionFoundError(format!(
        "Path ended surprisingly after {:?} {:?}.", point, dir)))
}

struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    fn parse(input: &[String]) -> Result<Self, AoCError<String>> {
        let grid = input.iter()
            .map(|line| Self::parse_line(line))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self{grid})
    }

    fn parse_line(line: &str) -> Result<Vec<Tile>, AoCError<String>> {
        line.chars().map(Tile::parse).collect()
    }

    fn get_tile(&self, point: Point) -> Option<Tile> {
        if point.1 >= self.grid.len() {
            return None
        }
        if point.0 >= self.grid[point.1].len() {
            return None
        }
        Some(self.grid[point.1][point.0])
    }

    fn get_new_direction(&self, point: Point, curr_dir: Direction) -> Option<Direction> {
        if self.get_tile(point) != Some(Tile::Corner) {
            return None
        }
        let new_directions = curr_dir.get_other_directions()
            .iter()
            .copied()
            .map(|dir| (dir, dir.move_point(point)))
            .filter(|(_, point)| point.is_some())
            .map(|(dir, point)| (dir, self.get_tile(point.unwrap())))
            .filter(|(_, tile)| tile.is_some() && tile.unwrap().is_path())
            .map(|(dir, _)| dir)
            .collect::<Vec<_>>();
        if new_directions.len() != 1 {
            return None
        }
        Some(new_directions[0])
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for line in self.grid.iter() {
            str = format!("{}[", str);
            for elem in line.iter() {
                str = format!("{}{}", str, elem);
            }
            str = format!("{}]\n", str);
        }
        write!(f, "{}", str)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Corner,
    Path,
    Waypoint(char),
}

impl Tile {
    fn parse(c: char) -> Result<Self, AoCError<String>> {
        match c {
            ' ' => Ok(Tile::Empty),
            '+' => Ok(Self::Corner),
            '-'|'|' => Ok(Self::Path),
            c => {
                if c.is_ascii_alphabetic() {
                    Ok(Self::Waypoint(c))
                } else {
                    Err(AoCError::BadInputFormat(format!(
                        "Unknown character, expected '+'. '-', '|' or 'A'-'Z'. Found '{}'.", c)))
                }
            }
        }
    }

    fn is_path(&self) -> bool {
        match self {
            Tile::Empty => false,
            Tile::Corner => false,
            Tile::Waypoint(_) => true,
            Tile::Path => true,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, " "),
            Tile::Corner => write!(f, "+"),
            Tile::Path => write!(f, "*"),
            Tile::Waypoint(c) => write!(f, "{}",c),
        }
    }
}

type Point = (usize, usize);

#[derive(Copy, Clone,Debug)]
enum Direction {
    Up, Right, Down, Left,
}

impl Direction {
    fn move_point(&self, point: Point) -> Option<Point> {
        match self {
            Direction::Up => {
                if point.1 == 0 {
                    None
                } else {
                    Some((point.0, point.1-1))
                }
            }
            Direction::Right => {
                Some((point.0+1, point.1))
            }
            Direction::Down => {
                Some((point.0, point.1+1))
            }
            Direction::Left => {
                if point.0 == 0 {
                    None
                } else {
                    Some((point.0-1, point.1))
                }
            }
        }
    }

    fn get_other_directions(&self) -> Vec<Direction> {
        match self {
            Direction::Up => vec![Direction::Right, Direction::Left],
            Direction::Right => vec![Direction::Up, Direction::Down],
            Direction::Down => vec![Direction::Right, Direction::Left],
            Direction::Left => vec![Direction::Up, Direction::Down],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "     |          ".to_string(),
            "     |  +--+    ".to_string(),
            "     A  |  C    ".to_string(),
            " F---|----E|--+ ".to_string(),
            "     |  |  |  D ".to_string(),
            "     +B-+  +--+ ".to_string(),
            "                ".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let v = get_example_input();
        assert_eq!(part_1(&v), Ok("ABCDEF".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 19)?;
        assert_eq!(part_1(&input), Ok("XYFDJNRCQA".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = get_example_input();
        assert_eq!(part_2(&v), Ok("38".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 19)?;
        assert_eq!(part_2(&input), Ok("17450".to_string()));
        Ok(())
    }
}
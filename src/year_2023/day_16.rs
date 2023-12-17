use std::cmp::max;
use std::collections::HashSet;
use crate::errors::{AoCError, AoCResult};
use crate::geometrics::{Direction, Grid, Parsable, Point};

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let grid = Grid::parse(input)?;
    Ok(grid.follow_all_paths((0, 0), Direction::East).to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let grid = Grid::parse(input)?;
    Ok(grid.follow_optimum_path().to_string())
}

impl Grid<Tile> {
    fn follow_all_paths(&self, start_pos: Point, start_dir: Direction) -> usize {
        let mut visited = HashSet::new();
        let mut unfinished = vec![(start_pos, start_dir)];

        while let Some((pos, dir)) = unfinished.pop() {
            if let Some(tile) = self.get_tile(&pos) {
                if !visited.insert((pos, dir)) {
                    continue
                }
                let mut next = tile.move_step(&pos, &dir);
                unfinished.append(&mut next);
            }
        }

        let visited = visited.into_iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>();
        visited.len()
    }

    fn follow_optimum_path(&self) -> usize {
        let mut maximum = 0;
        let size = self.get_dimension();
        for row in 0..size.1 {
            let start = (0, row);
            maximum = max(maximum,
                          self.follow_all_paths(start, Direction::East));
            let start =(size.1-1, row);
            maximum = max(maximum,
                          self.follow_all_paths(start, Direction::West));
        }
        for col in 0..size.0 {
            let start = (col, 0);
            maximum = max(maximum,
                          self.follow_all_paths(start, Direction::South));
            let start =(col, size.0-1);
            maximum = max(maximum,
                          self.follow_all_paths(start, Direction::North));
        }
        maximum
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    SplitterHorizontal,
    SplitterVertical,
    MirrorTopLeft,
    MirrorTopRight,
}

impl Tile {
    fn move_step(&self, point: &Point, direction: &Direction) -> Vec<(Point, Direction)> {
        let mut res = vec![];
        match self {
            Tile::Empty => {
                if let Some(point) = direction.move_point(point) {
                    res.push((point, *direction));
                }
            }
            Tile::SplitterHorizontal => {
                if direction.is_horizontal() {
                    if let Some(point) = direction.move_point(point) {
                        res.push((point, *direction));
                    }
                } else {
                    Direction::get_horizontal()
                        .into_iter()
                        .map(|dir| (dir.move_point(point), dir))
                        .filter(|(point, _)| point.is_some())
                        .for_each(|(point, dir)|
                            res.push((point.expect("Was filtered for Some(..)"), dir)));
                }
            }
            Tile::SplitterVertical => {
                if direction.is_vertical() {
                    if let Some(point) = direction.move_point(point) {
                        res.push((point, *direction));
                    }
                } else {
                    Direction::get_vertical()
                        .into_iter()
                        .map(|dir| (dir.move_point(point), dir))
                        .filter(|(point, _)| point.is_some())
                        .for_each(|(point, dir)|
                            res.push((point.expect("Was filtered for Some(..)"), dir)));
                }
            }
            Tile::MirrorTopLeft => {
                let dir = direction.mirror_top_left();
                if let Some(point) = dir.move_point(point) {
                    res.push((point, dir));
                }
            }
            Tile::MirrorTopRight => {
                let dir = direction.mirror_top_right();
                if let Some(point) = dir.move_point(point) {
                    res.push((point, dir));
                }
            }
        }
        res
    }
}

impl Parsable for Tile {
    fn parse(c: char) -> AoCResult<Self> {
        match c {
            '.' => Ok(Self::Empty),
            '-' => Ok(Self::SplitterHorizontal),
            '|' => Ok(Self::SplitterVertical),
            '/' => Ok(Self::MirrorTopRight),
            '\\' => Ok(Self::MirrorTopLeft),
            c => Err(AoCError::BadInputFormat(
                format!("Only '.', '-', '|', '/' and '\\' supported. Found '{}'", c))),
        }
    }
}

impl Direction {
    fn mirror_top_left(&self) -> Self {
        match self {
            Direction::North => Self::West,
            Direction::East => Self::South,
            Direction::South => Self::East,
            Direction::West => Self::North,
        }
    }

    fn mirror_top_right(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::North,
            Direction::South => Self::West,
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
            ".|...\\....".to_string(),
            "|.-.\\.....".to_string(),
            ".....|-...".to_string(),
            "........|.".to_string(),
            "..........".to_string(),
            ".........\\".to_string(),
            "..../.\\\\..".to_string(),
            ".-.-/..|..".to_string(),
            ".|....-|.\\".to_string(),
            "..//.|....".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("46".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 16)?;
        assert_eq!(part_1(&input), Ok("7482".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("51".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 16)?;
        assert_eq!(part_2(&input), Ok("7896".to_string()));
        Ok(())
    }
}
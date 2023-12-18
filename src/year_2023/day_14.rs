use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::errors::AoCError;
use crate::geometrics::{Direction, Grid, Parsable};

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut grid = Grid::parse(input)?;
    grid.roll_direction(Direction::North);
    Ok(grid.sum_rows().to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut grid = Grid::parse(input)?;
    grid.multi_circle(1000000000);
    Ok(grid.sum_rows().to_string())
}

type Point = crate::geometrics::Point<usize>;

impl Grid<Tile> {
    fn get_rounded(&self) -> Vec<(usize, usize)> {
        self.get_all_positions_of(&Tile::Rounded)
    }

    fn multi_circle(&mut self, count: usize) {
        let mut cache = HashMap::new();
        cache.insert(self.get_rounded(), 0);
        for index in 1..=count {
            self.circle();
            if let Some(prev) = cache.insert(self.get_rounded(), index) {
                let remaining = count - prev;
                let loop_len = index - prev;
                let remaining = remaining % loop_len;
                for _ in 0..remaining {
                    self.circle();
                }
                return;
            }
        }
    }

    fn circle(&mut self) {
        self.roll_direction(Direction::North);
        self.roll_direction(Direction::West);
        self.roll_direction(Direction::South);
        self.roll_direction(Direction::East);
    }

    fn move_tile(&mut self, p0: &Point, p1: &Point) -> bool {
        if self.get_tile(p0) != Some(&Tile::Rounded) {
            return false;
        }
        if self.get_tile(p1) != Some(&Tile::Empty) {
            return false;
        }
        self.set_tile(p0, Tile::Empty) &&
            self.set_tile(p1, Tile::Rounded)
    }

    fn roll_direction(&mut self, dir: Direction) {
        let size = self.get_dimension();
        let row_range = if dir == Direction::South {
            itertools::Either::Left((0..size.1).rev())
        } else {
            itertools::Either::Right(0..size.1)
        };
        let col_range =if dir == Direction::East {
            itertools::Either::Left((0..size.0).rev())
        } else {
            itertools::Either::Right(0..size.0)
        };

        for y in row_range {
            for x in col_range.clone() {
                let point = (x, y);
                if let Some(&Tile::Rounded) = self.get_tile(&(x, y)) {
                    let mut target = point;
                    while let Some(next_point) = dir.move_point(&target) {
                        if self.get_tile(&next_point) != Some(&Tile::Empty) {
                            break;
                        }
                        target = next_point;
                    }
                    if target != point {
                        assert!(self.move_tile(&point, &target));
                    }
                }
            }
        }
    }

    fn sum_rows(&self) -> usize {
        let size = self.get_dimension();
        let nums = self.iter().zip((1..=size.1).rev())
            .map(|(row, index)| row.iter()
                .filter(|elem| **elem == Tile::Rounded)
                .count()*index)
            .collect::<Vec<_>>();
        nums.iter().sum()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Tile {
    Square,
    Rounded,
    Empty,
}

impl Parsable for Tile {
    fn parse(c: char) -> Result<Self, AoCError<String>> {
        match c {
            '#' => Ok(Self::Square),
            'O' => Ok(Self::Rounded),
            '.' => Ok(Self::Empty),
            c => Err(AoCError::BadInputFormat(
                format!("Only '#', 'O' and '.' are supported as tiles. Found '{}'", c))),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Square => write!(f, "#"),
            Tile::Rounded => write!(f, "O"),
            Tile::Empty => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "O....#....".to_string(),
            "O.OO#....#".to_string(),
            ".....##...".to_string(),
            "OO.#O....O".to_string(),
            ".O.....O#.".to_string(),
            "O.#..O.#.#".to_string(),
            "..O..#O..O".to_string(),
            ".......O..".to_string(),
            "#....###..".to_string(),
            "#OO..#....".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("136".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 14)?;
        assert_eq!(part_1(&input), Ok("106186".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("64".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 14)?;
        assert_eq!(part_2(&input), Ok("106390".to_string()));
        Ok(())
    }
}
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut grid = Grid::parse(input)?;
    grid.roll_north();
    Ok(grid.sum_rows().to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut grid = Grid::parse(input)?;
    grid.multi_circle(1000000000);
    Ok(grid.sum_rows().to_string())
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    fn parse(input: &[String]) -> Result<Self, AoCError<String>> {
        let grid = input.iter()
            .map(|line| line.chars()
                .map(Tile::parse)
                .collect::<Result<_, _>>())
            .collect::<Result<_, _>>()?;
        Ok(Self { grid })
    }

    fn get_rounded(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, elem) in row.iter().enumerate() {
                if *elem == Tile::Rounded {
                    res.push((row_index, col_index));
                }
            }
        }
        res
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
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn roll_north(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if let Tile::Rounded = self.grid[y][x] {
                    let mut target = y;
                    while target > 0 && Tile::Empty == self.grid[target-1][x] {
                        target -= 1;
                    }
                    if target != y {
                        self.grid[y][x] = Tile::Empty;
                        self.grid[target][x] = Tile::Rounded;
                    }
                }
            }
        }
    }

    fn roll_south(&mut self) {
        for y in (0..self.grid.len()).rev() {
            for x in 0..self.grid[y].len() {
                if let Tile::Rounded = self.grid[y][x] {
                    let mut target = y;
                    while target+1 < self.grid.len() && Tile::Empty == self.grid[target+1][x] {
                        target += 1;
                    }
                    if target != y {
                        self.grid[y][x] = Tile::Empty;
                        self.grid[target][x] = Tile::Rounded;
                    }
                }
            }
        }
    }

    fn roll_east(&mut self) {
        for y in 0..self.grid.len() {
            for x in (0..self.grid[y].len()).rev() {
                if let Tile::Rounded = self.grid[y][x] {
                    let mut target = x;
                    while target+1 < self.grid[y].len() && Tile::Empty == self.grid[y][target+1] {
                        target += 1;
                    }
                    if target != x {
                        self.grid[y][x] = Tile::Empty;
                        self.grid[y][target] = Tile::Rounded;
                    }
                }
            }
        }
    }

    fn roll_west(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if let Tile::Rounded = self.grid[y][x] {
                    let mut target = x;
                    while target > 0 && Tile::Empty == self.grid[y][target-1] {
                        target -= 1;
                    }
                    if target != x {
                        self.grid[y][x] = Tile::Empty;
                        self.grid[y][target] = Tile::Rounded;
                    }
                }
            }
        }
    }

    fn sum_rows(&self) -> usize {
        /*self.grid.iter().zip((1..=self.grid.len()).rev())
            .map(|(row, index)| row.iter()
                .filter(|elem| **elem == Tile::Rounded)
                .count()*index)
            .sum()

         */
        let nums = self.grid.iter().zip((1..=self.grid.len()).rev())
            .map(|(row, index)| row.iter()
                .filter(|elem| **elem == Tile::Rounded)
                .count()*index)
            .collect::<Vec<_>>();
        nums.iter().sum()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter() {
            for elem in line.iter() {
                write!(f, "{}", elem).unwrap();
            }
            writeln!(f).unwrap();
        }
        write!(f, "")
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Tile {
    Square,
    Rounded,
    Empty,
}

impl Tile {
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
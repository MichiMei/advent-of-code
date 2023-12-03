use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let schematic = Schematic::parse(input)?;
    let sum = schematic.sum_part_numbers();
    Ok(sum.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let schematic = Schematic::parse(input)?;
    let sum = schematic.sum_gear_numbers();
    Ok(sum.to_string())
}

type Point = (usize, usize);

struct Schematic {
    grid: Vec<Vec<Symbol>>
}

impl Schematic {
    fn parse(input: &Vec<String>) -> Result<Self, AoCError<String>> {
        if input.is_empty() {
            return Err(AoCError::BadInputFormat("Empty grid not supported".to_string()));
        }
        let mut grid = Vec::with_capacity(input.len());
        let len = input[0].len();
        for line in input {
            if line.len() != len {
                return Err(AoCError::BadInputFormat(
                    format!("Row lengths differ, expected {}, found {}", len, line.len())));
            }
            let mut row = Vec::with_capacity(len);
            for char in line.chars() {
                row.push(Symbol::parse(char));
            }
            grid.push(row);
        }
        Ok(Self { grid })
    }

    fn sum_part_numbers(mut self) -> u32 {
        let mut sum = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if let Symbol::Symbol(_) = self.grid[y][x] {
                    let point = (x, y);
                    let part_sum = self.sum_surrounding_numbers(point);
                    sum += part_sum;
                }
            }
        }
        sum
    }

    fn sum_surrounding_numbers(&mut self, point: Point) -> u32 {
        self.get_surrounding_numbers(point).iter().sum()
    }

    fn get_surrounding_numbers(&mut self, point: Point) -> Vec<u32> {
        let mut numbers = vec![];
        let x_start = if point.0 > 0 {
            point.0 - 1
        } else {
            0
        };
        let x_end = if point.0+1 < self.grid[0].len() {
            point.0 + 1
        } else {
            point.0
        };
        let y_start = if point.1 > 0 {
            point.1 - 1
        } else {
            0
        };
        let y_end = if point.1+1 < self.grid.len() {
            point.1 + 1
        } else {
            point.1
        };
        for y in y_start..=y_end {
            for x in x_start..=x_end {
                if x == point.0 && y == point.1 {
                    continue
                }
                if let Symbol::Digit(_) = self.grid[y][x] {
                    let start = self.find_number_start((x, y));
                    numbers.push(self.get_number_from_start(start));
                }
            }
        }
        numbers
    }

    fn find_number_start(&self, mut start: Point) -> Point {
        while start.0 > 0 {
            let x = start.0 - 1;
            match self.grid[start.1][x] {
                Symbol::Digit(_) => start = (x, start.1),
                _ => break,
            }
        }
        start
    }

    /// Starts to read the part number from left to right
    /// Empties every encountered digit
    ///
    /// Given starting coordinates should be the left most digit of the part number
    fn get_number_from_start(&mut self, start: Point) -> u32 {
        let (mut x, y) = start;
        let mut number = 0;
        while let Symbol::Digit(d) = self.grid[y][x] {
            self.grid[y][x] = Symbol::Empty;
            number = number*10 + d as u32;
            x += 1;
            if x == self.grid[y].len() {
                break;
            }
        }
        number
    }

    fn sum_gear_numbers(mut self) -> u32 {
        let mut sum = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if let Symbol::Symbol('*') = self.grid[y][x] {
                    let gear_ratio = self.get_gear_ratio((x, y));
                    sum += gear_ratio;
                }
            }
        }
        sum
    }

    fn get_gear_ratio(&mut self, point: Point) -> u32 {
        let numbers = self.get_surrounding_numbers(point);
        if numbers.len() == 2 {
            numbers.iter().product()
        } else {
            0
        }
    }
}

enum Symbol {
    Empty,
    Digit(u8),
    Symbol(char),
}

impl Symbol {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '0'..='9' => Self::Digit(c as u8 - b'0'),
            c => Self::Symbol(c),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Empty => write!(f, "."),
            Symbol::Digit(d) => write!(f, "{}", d),
            Symbol::Symbol(c) => write!(f, "{}", c),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("4361".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 3)?;
        assert_eq!(part_1(&input), Ok("530849".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("467835".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 3)?;
        assert_eq!(part_2(&input), Ok("84900879".to_string()));
        Ok(())
    }
}
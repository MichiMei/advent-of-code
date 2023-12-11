use std::cmp::{max, min};
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let universe = Universe::parse(input)?;
    Ok(universe.sum_shortest_paths(2).to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let universe = Universe::parse(input)?;
    Ok(universe.sum_shortest_paths(1000000).to_string())
}

struct Universe {
    galaxies: Vec<Point>,
    empty_rows: Vec<bool>,
    empty_cols: Vec<bool>,
}

impl Universe {
    fn parse(input: &Vec<String>) -> Result<Self, AoCError<String>> {
        if input.is_empty() {
            return Err(AoCError::UnexpectedInputLength(
                "Input is empty, needs to be at least one line.".to_string()))
        }
        if input[0].is_empty() {
            return Err(AoCError::UnexpectedInputLength(
                "Input line is empty, mst be at least one character".to_string()))
        }
        let size = (input[0].len(), input.len());
        let galaxies = input.iter()
            .enumerate()
            .flat_map(|(index_row, line)| line.chars().enumerate()
                .map(move |(index_col, c)| ((index_col, index_row), c)))
            .filter(|(_, c)| *c == '#')
            .map(|(point, _)| point)
            .collect::<Vec<_>>();
        let mut empty_rows = vec![true; size.1];
        let mut empty_cols = vec![true; size.0];
        for (col, row) in galaxies.iter() {
            empty_rows[*row] = false;
            empty_cols[*col] = false;
        }
        Ok(Self {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }

    fn sum_shortest_paths(&self, empty_multiplier: usize) -> usize {
        let mut sum = 0;
        for (start, first) in self.galaxies.iter().enumerate() {
            for second in self.galaxies[start+1..].iter() {
                let row_range =
                    min(first.1, second.1)..max(first.1,second.1);
                let col_range =
                    min(first.0, second.0)..max(first.0,second.0);
                let distance = first.0.abs_diff(second.0) +
                    first.1.abs_diff(second.1) +
                    self.empty_rows[row_range].iter()
                        .filter(|empty| **empty).count() * (empty_multiplier-1) +
                    self.empty_cols[col_range].iter()
                        .filter(|empty| **empty).count() * (empty_multiplier-1);
                sum += distance;
            }
        }
        sum
    }
}

type Point = (usize, usize);

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("374".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 11)?;
        assert_eq!(part_1(&input), Ok("9563821".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() -> Result<(), AoCError<String>> {
        let universe = Universe::parse(&get_example_input())?;
        assert_eq!(universe.sum_shortest_paths(100), 8410);
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 11)?;
        assert_eq!(part_2(&input), Ok("827009909817".to_string()));
        Ok(())
    }
}
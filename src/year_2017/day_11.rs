use std::cmp::{max, min};
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing a list of directions.".to_string()))
    }
    count_directions(&input[0]).map(|c| c.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing a list of directions.".to_string()))
    }
    find_furthest(&input[0]).map(|c| c.to_string())
}

fn count_directions(line: &str) -> Result<usize, AoCError<String>> {
    let mut directions = [0usize;6];
    for direction in line
        .split(',').map(Direction::parse) {
        directions[direction?.to_index()] += 1;
    }
    while eliminate_opposing(&mut directions) | eliminate_adjacent(&mut directions) {}
    Ok(directions.iter().sum())
}

fn find_furthest(line: &str) -> Result<usize, AoCError<String>> {
    let mut max_distance = 0;
    let mut directions = [0usize;6];
    for direction in line
        .split(',').map(Direction::parse) {
        directions[direction?.to_index()] += 1;
        while eliminate_opposing(&mut directions) | eliminate_adjacent(&mut directions) {}
        max_distance = max(max_distance, directions.iter().sum())
    }
    Ok(max_distance)
}

fn eliminate_opposing(directions: &mut [usize; 6]) -> bool {
    let mut changed = false;
    for index in 0..3 {
        let redundant = min(directions[index], directions[index+3]);
        if redundant > 0 {
            changed = true;
            directions[index] -= redundant;
            directions[index+3] -= redundant;
        }
    }
    changed
}

fn eliminate_adjacent(directions: &mut [usize; 6]) -> bool {
    let mut changed = false;
    for index in 0..6 {
        let redundant = min(directions[index], directions[(index+2)%6]);
        if redundant > 0 {
            changed = true;
            directions[index] -= redundant;
            directions[(index+2)%6] -= redundant;
            directions[(index+1)%6] += redundant;
        }
    }
    changed
}

enum Direction {
    North, NorthEast, SouthEast, South, SouthWest, NorthWest,
}

impl Direction {
    fn parse(str: &str) -> Result<Self, AoCError<String>> {
        match str {
            "n" => Ok(Self::North),
            "ne" => Ok(Self::NorthEast),
            "se" => Ok(Self::SouthEast),
            "s" => Ok(Self::South),
            "sw" => Ok(Self::SouthWest),
            "nw" => Ok(Self::NorthWest),
            _ => Err(AoCError::BadInputFormat(format!(
                "Unknown direction '{}'. Only 'n', 'ne', 'se', 's', 'sw' and 'nw allowed.", str))),
        }
    }

    fn to_index(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::NorthEast => 1,
            Direction::SouthEast => 2,
            Direction::South => 3,
            Direction::SouthWest => 4,
            Direction::NorthWest => 5,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["ne,ne,ne".to_string()]), Ok("3".to_string()));
        assert_eq!(part_1(&vec!["ne,ne,sw,sw".to_string()]), Ok("0".to_string()));
        assert_eq!(part_1(&vec!["ne,ne,s,s".to_string()]), Ok("2".to_string()));
        assert_eq!(part_1(&vec!["se,sw,se,sw,sw".to_string()]), Ok("3".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 11)?;
        assert_eq!(part_1(&input), Ok("805".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 11)?;
        assert_eq!(part_2(&input), Ok("1535".to_string()));
        Ok(())
    }
}
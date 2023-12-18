use std::collections::HashSet;
use crate::errors::{AoCError, AoCResult};
use crate::geometrics::{Direction, Point};

pub fn part_1(input: &[String]) -> AoCResult<String> {
    let path = Path::from_instructions(input, false)?;
    Ok(path.calculate_area().to_string())
}

pub fn part_2(input: &[String]) -> AoCResult<String> {
    let path = Path::from_instructions(input, true)?;
    Ok(path.calculate_area().to_string())
}

struct Path {
    corners: Vec<Point<i32>>,
    len: usize,
}

impl Path {
    fn new() -> Self {
        let mut positions = HashSet::new();
        positions.insert((0, 0));
        Self {
            //positions,
            corners: vec![(0, 0)],
            len: 0,
            //current: (0, 0),
            //min: (0, 0),
            //max: (0, 0),
        }
    }

    fn from_instructions(input: &[String], parse_color: bool) -> AoCResult<Self> {
        let mut path = Self::new();
        let mut current = (0, 0);
        for line in input {
            let (mut dir, mut steps, color) = Self::parse_instruction(line)?;
            if parse_color {
                (dir, steps) = Self::parse_color(&color)?;
            }
            current = dir.move_point_steps(&current, steps as i32)
                .expect("Only fails on overflow");
            path.corners.push(current);
            path.len += steps;
        }
        Ok(path)
    }

    fn parse_instruction(line: &str) -> AoCResult<(Direction, usize, String)> {
        let split = line.split_whitespace().collect::<Vec<_>>();
        if split.len() != 3 {
            return Err(AoCError::BadInputFormat(
                format!("Expected Instruction '<U|R|D|L> <steps> (<color>)'. Found: '{}'", line)))
        }
        let dir = Self::parse_direction(split[0])?;
        let steps = Self::parse_steps(split[1])?;
        let color = split[2].to_string();
        Ok((dir, steps, color))
    }

    fn parse_direction(str: &str) -> AoCResult<Direction> {
        if str.len() != 1 {
            return Err(AoCError::BadInputFormat(
                format!("Direction has to be a single character. Found '{}'", str)))
        }
        match str.chars().next().expect("Slice is not empty") {
            'U' => Ok(Direction::North),
            'R' => Ok(Direction::East),
            'D' => Ok(Direction::South),
            'L' => Ok(Direction::West),
            c => Err(AoCError::BadInputFormat(
                format!("Only directions 'U', 'R', 'D' and 'L' allowed. Found '{}'", c))),
        }
    }

    fn parse_steps(str: &str) -> AoCResult<usize> {
        str.parse()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing steps from '{}' failed. {}", str, e)))
    }

    fn parse_color(str: &str) -> AoCResult<(Direction, usize)> {
        let step = usize::from_str_radix(&str[2..=6], 16)
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing color-step from '{}' failed. {}", str, e)))?;
        let direction = match &str[7..=7] {
            "0" => Ok(Direction::East),
            "1" => Ok(Direction::South),
            "2" => Ok(Direction::West),
            "3" => Ok(Direction::North),
            _ => Err(AoCError::BadInputFormat(
                format!("Parsing color-direction from '{}' failed.", str)))
        }?;
        Ok((direction, step))
    }

    fn calculate_area(&self) -> usize {
        if self.corners.len() < 3 {
            return 0;
        }
        let len = if self.corners.first().unwrap() == self.corners.last().unwrap() {
            self.corners.len()-1
        } else {
            self.corners.len()
        };
        let mut sum = 0;
        for index in 0..len {
            let curr_corner = self.corners[index];
            let next_corner = self.corners[(index+1)%len];
            sum += (curr_corner.1+next_corner.1) as i64 * (curr_corner.0-next_corner.0) as i64;
        }
        assert_eq!(sum%2, 0);
        assert!(sum > 0);
        let internal = (sum/2) as usize;
        internal+self.len/2+1
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "R 6 (#70c710)".to_string(),
            "D 5 (#0dc571)".to_string(),
            "L 2 (#5713f0)".to_string(),
            "D 2 (#d2c081)".to_string(),
            "R 2 (#59c680)".to_string(),
            "D 2 (#411b91)".to_string(),
            "L 5 (#8ceee2)".to_string(),
            "U 2 (#caa173)".to_string(),
            "L 1 (#1b58a2)".to_string(),
            "U 2 (#caa171)".to_string(),
            "R 2 (#7807d2)".to_string(),
            "U 3 (#a77fa3)".to_string(),
            "L 2 (#015232)".to_string(),
            "U 2 (#7a21e3)".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("62".to_string()));
    }

    #[test]
    fn check_input_part_1() -> AoCResult<()> {
        let input = get_input(2023, 18)?;
        assert_eq!(part_1(&input), Ok("46334".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("952408144115".to_string()));
    }

    #[test]
    fn check_input_part_2() -> AoCResult<()> {
        let input = get_input(2023, 18)?;
        assert_eq!(part_2(&input), Ok("102000662718092".to_string()));
        Ok(())
    }
}
use std::collections::HashSet;

pub fn part_1(input: &[String]) -> Result<String, &str> {
    assert_eq!(input.len(), 1);
    let line = &input[0];
    let commands = Commands::new(line);

    let mut point = (0,0);
    let mut direction = Direction::North;
    for (dir_change, steps) in commands {
        match dir_change {
            true => direction = direction.turn_right(),
            false => direction = direction.turn_left(),
        }
        direction.move_position(&mut point, steps);
    }

    let dist = point.0.abs() + point.1.abs();

    Ok(dist.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, &str> {
    assert_eq!(input.len(), 1);
    let line = &input[0];
    let commands = Commands::new(line);

    let mut point = (0,0);
    let mut direction = Direction::North;
    let mut visited = HashSet::new();
    visited.insert(point);
    let mut found = false;
    'outer: for (dir_change, steps) in commands {
        match dir_change {
            true => direction = direction.turn_right(),
            false => direction = direction.turn_left(),
        }
        for _ in 0..steps {
            direction.move_position(&mut point, 1);
            if visited.contains(&point) {
                found = true;
                break 'outer;
            }
            visited.insert(point);
        }
    }
    assert!(found);

    let dist = point.0.abs() + point.1.abs();

    Ok(dist.to_string())
}

type Point = (i32, i32);

struct Commands<'a> {
    words: Vec<&'a str>,
    index: usize,
}

impl<'a> Commands<'a> {
    fn new(str: &'a str) -> Self {
        let words = str.split(", ").collect();
        Self {words, index: 0}
    }

    fn read_next_command(&mut self) -> Result<Option<(bool ,usize)>, &str> {
        if self.index >= self.words.len() {
            return Ok(None)
        }

        let next_str = self.words[self.index];
        if next_str.len() < 2 {
            return Err(ERR_INPUT_MALFORMED)
        }
        self.index += 1;

        let direction = match next_str.chars().next().expect("String was tested to be len>=2") {
            'L' => false,
            'R' => true,
            _ => return Err(ERR_INPUT_MALFORMED),
        };

        let steps = next_str[1..].parse().map_err(|_| ERR_INPUT_MALFORMED)?;

        Ok(Some((direction, steps)))
    }
}

impl<'a> Iterator for Commands<'a> {
    type Item = (bool, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.read_next_command();
        match tmp {
            Ok(None) => None,
            Ok(Some(next)) => Some(next),
            Err(e) => panic!("{}", e),
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn move_position(&self, point: &mut Point, steps: usize) {
        assert!(steps <= i32::MAX as usize);
        let steps = steps as i32;
        match self {
            Direction::North => point.0 += steps,
            Direction::East => point.1 += steps,
            Direction::South => point.0 -= steps,
            Direction::West => point.1 -= steps,
        }

    }
}

const ERR_INPUT_MALFORMED: &str = "Input is malformed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&["R2, L3".to_string()]), Ok("5".to_string()));
        assert_eq!(part_1(&["R2, R2, R2".to_string()]), Ok("2".to_string()));
        assert_eq!(part_1(&["R5, L5, R5, R3".to_string()]), Ok("12".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_01.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("209".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&["R8, R4, R4, R8".to_string()]), Ok("4".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_01.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("136".to_string()));
        Ok(())
    }
}
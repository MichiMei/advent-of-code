use std::collections::HashMap;
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected exactly one line containing the grid index.".to_string()))
    }
    let mut square_index = input[0].parse::<usize>()
        .map_err(|e| AoCError::BadInputFormat(format!(
            "Parsing grid index failed, expected number, found '{}'. {}", input[0], e)))?;
    if square_index == 0 {
        return Err(AoCError::BadInputFormat("Grid indices start from 1".to_string()))
    }
    square_index -= 1;
    let ring_index = calculate_ring_index(square_index);
    let ring_position = calculate_ring_position(square_index, ring_index);
    Ok((ring_index+ring_position).to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected exactly one line containing the grid index.".to_string()))
    }
    let max_value = input[0].parse::<usize>()
        .map_err(|e| AoCError::BadInputFormat(format!(
            "Parsing grid index failed, expected number, found '{}'. {}", input[0], e)))?;
    if max_value == 0 {
        return Err(AoCError::BadInputFormat("Grid indices start from 1".to_string()))
    }
    Ok(stress_test(max_value).to_string())
}

fn calculate_ring_index(square: usize) -> usize {
    for index in 0usize.. {
        let tmp = index*2+1;
        if square < tmp.saturating_mul(tmp) {
            return index
        }
    }
    panic!("This part should never be reached")
}

fn calculate_ring_position(square: usize, ring_index: usize) -> usize {
    if ring_index == 0 {
        return 0
    }
    let tmp = (ring_index-1)*2+1;
    let relative_pos = square-tmp*tmp;

    let side_pos = relative_pos%(ring_index*2);
    side_pos.abs_diff(ring_index-1)
}

fn stress_test(max_value: usize) -> usize {
    let mut values = HashMap::new();
    let mut point = (0, 0);
    let mut dir = Direction::new();
    values.insert(point, 1);

    loop {
        (dir, point) = dir.next(&point);
        let value = get_neighbors(&point).into_iter()
            .map(|p| *values.get(&p).unwrap_or(&0))
            .sum();
        if value > max_value {
            return value
        }
        values.insert(point, value);
    }

}

fn get_neighbors(p: &Point) -> Vec<Point> {
    vec![
        (p.0+1, p.1+1),
        (p.0+1, p.1  ),
        (p.0+1, p.1-1),
        (p.0  , p.1+1),
        (p.0  , p.1-1),
        (p.0-1, p.1+1),
        (p.0-1, p.1  ),
        (p.0-1, p.1-1),
    ]
}

type Point = (i32, i32);

#[derive(Copy, Clone)]
enum Direction {
    Up(usize, usize),
    Down(usize, usize),
    Right(usize, usize),
    Lef(usize, usize),
}

impl Direction {
    pub fn new() -> Self {
        Self::Right(1, 1)
    }

    pub fn next(&self, point: &Point) -> (Self, Point) {
        match self {
            Direction::Up(total, remaining) => {
                if *remaining == 0 {
                    let new_dir = Direction::Lef(total+1, total+1);
                    new_dir.next(point)
                } else {
                    let new_dir = Direction::Up(*total, remaining-1);
                    let new_point = (point.0, point.1-1);
                    (new_dir, new_point)
                }
            }
            Direction::Down(total, remaining) => {
                if *remaining == 0 {
                    let new_dir = Direction::Right(total+1, total+1);
                    new_dir.next(point)
                } else {
                    let new_dir = Direction::Down(*total, remaining-1);
                    let new_point = (point.0, point.1+1);
                    (new_dir, new_point)
                }
            }
            Direction::Right(total, remaining) => {
                if *remaining == 0 {
                    let new_dir = Direction::Up(*total, *total);
                    new_dir.next(point)
                } else {
                    let new_dir = Direction::Right(*total, remaining-1);
                    let new_point = (point.0+1, point.1);
                    (new_dir, new_point)
                }
            }
            Direction::Lef(total, remaining) => {
                if *remaining == 0 {
                    let new_dir = Direction::Down(*total, *total);
                    new_dir.next(point)
                } else {
                    let new_dir = Direction::Lef(*total, remaining-1);
                    let new_point = (point.0-1, point.1);
                    (new_dir, new_point)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_calculate_ring_index() {
        /*assert_eq!(calculate_ring_index(1), 0);
        assert_eq!(calculate_ring_index(9), 1);
        assert_eq!(calculate_ring_index(25), 2);
        assert_eq!(calculate_ring_index(49), 3);
        assert_eq!(calculate_ring_index(81), 4);
        assert_eq!(calculate_ring_index(121), 5);
        assert_eq!(calculate_ring_index(169), 6);
        assert_eq!(calculate_ring_index(225), 7);
        assert_eq!(calculate_ring_index(289), 8);
        assert_eq!(calculate_ring_index(361), 9);*/
        assert_eq!(calculate_ring_index(0), 0);
        assert_eq!(calculate_ring_index(8), 1);
        assert_eq!(calculate_ring_index(24), 2);
        assert_eq!(calculate_ring_index(48), 3);
        assert_eq!(calculate_ring_index(80), 4);
        assert_eq!(calculate_ring_index(120), 5);
        assert_eq!(calculate_ring_index(168), 6);
        assert_eq!(calculate_ring_index(224), 7);
        assert_eq!(calculate_ring_index(288), 8);
        assert_eq!(calculate_ring_index(360), 9);
    }

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["1".to_string()]), Ok("0".to_string()));
        assert_eq!(part_1(&vec!["12".to_string()]), Ok("3".to_string()));
        assert_eq!(part_1(&vec!["23".to_string()]), Ok("2".to_string()));
        assert_eq!(part_1(&vec!["1024".to_string()]), Ok("31".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_03.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("438".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["146".to_string()]), Ok("147".to_string()));
        assert_eq!(part_2(&vec!["147".to_string()]), Ok("304".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_03.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("266330".to_string()));
        Ok(())
    }
}
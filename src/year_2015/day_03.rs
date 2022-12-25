use std::collections::HashSet;

pub fn part_1(input: &Vec<String>) -> Result<String, &str> {
    if input.len() != 1 {
        return Err(ERR_VEC_LENGTH)
    }
    let mut point = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(point);
    let str = input.first().unwrap();

    for c in str.chars() {
        point = move_point(c, point)?;
        visited.insert(point);
    }
    Ok(visited.len().to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, &str> {
    if input.len() != 1 {
        return Err(ERR_VEC_LENGTH)
    }
    let mut santa = (0, 0);
    let mut roboter_santa = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(santa);
    let str = input.first().unwrap();
    let mut iter = str.chars();

    loop {
        if let Some(c) = iter.next() {
            santa = move_point(c, santa)?;
            visited.insert(santa);
        } else {
            break;
        }
        if let Some(c) = iter.next() {
            roboter_santa = move_point(c, roboter_santa)?;
            visited.insert(roboter_santa);
        } else {
            break;
        }
    }
    Ok(visited.len().to_string())
}

fn move_point(dir: char, point: (i32, i32)) -> Result<(i32, i32), &'static str> {
    Ok(match dir {
        '^' => (point.0+1, point.1),
        '>' => (point.0, point.1+1),
        'v' => (point.0-1, point.1),
        '<' => (point.0, point.1-1),
        _ => return Err(ERR_CHAR_UNSUPPORTED),
    })
}

const ERR_VEC_LENGTH: &str = "The input is expected to be exactly one line";
const ERR_CHAR_UNSUPPORTED: &str = "Unexpected character in input, \
        only '^', '>', 'v' and '<' allowed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec![">".to_string()]), Ok("2".to_string()));
        assert_eq!(part_1(&vec!["^>v<".to_string()]), Ok("4".to_string()));
        assert_eq!(part_1(&vec!["^v^v^v^v^v".to_string()]), Ok("2".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_03.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("2592".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["^v".to_string()]), Ok("3".to_string()));
        assert_eq!(part_2(&vec!["^>v<".to_string()]), Ok("3".to_string()));
        assert_eq!(part_2(&vec!["^v^v^v^v^v".to_string()]), Ok("11".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_03.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("2360".to_string()));
        Ok(())
    }
}
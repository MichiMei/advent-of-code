use crate::errors::{AoCError, AoCResult};

pub fn part_1(input: &[String]) -> AoCResult<String> {
    let grid = parse_input(input)?;
    let res = grid.iter()
        .filter(|row| check_row(row, None, false) || check_row_rev(row, false))
        .count();
    Ok(res.to_string())
}

fn parse_input(input: &[String]) -> AoCResult<Vec<Vec<i32>>> {
    input.iter()
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &str) -> AoCResult<Vec<i32>> {
    line.split_whitespace()
        .map(|elem| elem.parse::<i32>()).collect::<Result<Vec<_>, _>>()
        .map_err(|e| AoCError::BadInputFormat(format!("Expected Integer.\n{}", e)))
}

fn check_row(row: &[i32], prev: Option<i32>, can_remove: bool) -> bool {
    if row.is_empty() {
        return true
    }
    if can_remove && check_row(&row[1..], prev, false) {
        return true
    }
    if let Some(prev) = prev {
        if !compare(prev, row[0]) {
            return false
        }
        check_row(&row[1..], Some(row[0]), can_remove)
    } else {
        check_row(&row[1..], Some(row[0]), can_remove)
    }
}

fn compare(first: i32, second: i32) -> bool {
    if second <= first || second > first+3 {
        false
    } else {
        true
    }
}

fn check_row_rev(row: &[i32], can_remove: bool) -> bool {
    let mut v = row.to_vec();
    v.reverse();
    check_row(&v, None, can_remove)
}

pub fn part_2(input: &[String]) -> AoCResult<String> {
    let grid = parse_input(input)?;
    let res = grid.iter()
        .filter(|row| check_row(row, None, true) || check_row_rev(row, true))
        .count();
    Ok(res.to_string())
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ]
        /*vec![
            "48 46 47 49 51 54 56".to_string(),
            "1 1 2 3 4 5".to_string(),
            "1 2 3 4 5 5".to_string(),
            "5 1 2 3 4 5".to_string(),
            "1 4 3 2 1".to_string(),
            "1 6 7 8 9".to_string(),
            "1 2 3 4 3".to_string(),
            "9 8 7 6 7".to_string(),
            "7 10 8 10 11".to_string(),
            "29 28 27 25 26 25 22 20".to_string(),
        ]*/
    }

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&get_example_input()), Ok("2".to_string()));
    }

    #[test]
    fn check_input_part_1() -> AoCResult<()> {
        let input = get_input(2024, 2)?;
        assert_eq!(part_1(&input), Ok("534".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&get_example_input()), Ok("4".to_string()));
    }

    #[test]
    fn check_input_part_2() -> AoCResult<()> {
        let input = get_input(2024, 2)?;
        assert_eq!(part_2(&input), Ok("577".to_string()));
        Ok(())
    }
}
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let jumps = parse_numbers(input)?;
    Ok(execute_jumps_part_1(jumps).to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let jumps = parse_numbers(input)?;
    Ok(execute_jumps_part_2(jumps).to_string())
}

fn parse_numbers(input: &[String]) -> Result<Vec<i32>, AoCError<String>> {
    input.iter()
        .map(|line| line.parse()).collect::<Result<Vec<_>, _>>()
        .map_err(|e| AoCError::BadInputFormat(
            format!("Parsing input numbers failed. {}", e)))
}

fn execute_jumps_part_1(mut jumps: Vec<i32>) -> usize {
    let mut steps = 0;
    let mut index = 0i64;
    while index >= 0 && index < jumps.len() as i64 {
        steps += 1;
        let new_index = index + jumps[index as usize] as i64;
        jumps[index as usize] += 1;
        index = new_index;
    }
    steps
}

fn execute_jumps_part_2(mut jumps: Vec<i32>) -> usize {
    let mut steps = 0;
    let mut index = 0i64;
    while index >= 0 && index < jumps.len() as i64 {
        steps += 1;
        let new_index = index + jumps[index as usize] as i64;
        if jumps[index as usize] >= 3 {
            jumps[index as usize] -= 1;
        } else {
            jumps[index as usize] += 1;
        }
        index = new_index;
    }
    steps
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "0".to_string(),
            "3".to_string(),
            "0".to_string(),
            "1".to_string(),
            "-3".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let v = get_example_input();
        assert_eq!(part_1(&v), Ok("5".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 5)?;
        assert_eq!(part_1(&input), Ok("343467".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = get_example_input();
        assert_eq!(part_2(&v), Ok("10".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 5)?;
        assert_eq!(part_2(&input), Ok("24774780".to_string()));
        Ok(())
    }
}
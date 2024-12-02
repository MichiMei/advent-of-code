use std::collections::HashMap;
use crate::errors::{AoCError, AoCResult};

pub fn part_1(input: &Vec<String>) -> AoCResult<String> {
    let (mut first, mut second) = parse_input(input)?;
    let res = compare_ordered(&mut first, &mut second);
    Ok(res.to_string())
}

fn parse_input(input: &Vec<String>) -> AoCResult<(Vec<u32>, Vec<u32>)> {
    let mut first_res = Vec::with_capacity(input.len());
    let mut second_res = Vec::with_capacity(input.len());
    for line in input {
        let split = line.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(AoCError::BadInputFormat(
                format!("Expected lines with two integers, found {}", line)))
        }
        let first = split[0].parse::<u32>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing first int failed: {}\n{}", split[0], e)))?;
        first_res.push(first);
        let second = split[1].parse::<u32>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing second int failed: {}\n{}", split[0], e)))?;
        second_res.push(second)
    }
    Ok((first_res, second_res))
}

fn compare_ordered(first: &mut [u32], second: &mut [u32]) -> u32 {
    first.sort_unstable();
    second.sort_unstable();
    first.iter().zip(second.iter())
        .map(|(f, s)| f.abs_diff(*s))
        .sum()
}

pub fn part_2(input: &Vec<String>) -> AoCResult<String> {
    let (mut first, mut second) = parse_input(input)?;
    let first = account(&mut first);
    let second = account(&mut second);
    let res = compare_occurrences(&first, &second);
    Ok(res.to_string())
}

fn account(numbers: &mut [u32]) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    numbers.sort_unstable();
    for number in numbers {
        let number = *number as usize;
        let prev = map.get(&number).copied().unwrap_or(0);
        map.insert(number, prev+1);
    }
    map
}

fn compare_occurrences(first: &HashMap<usize, usize>, second: &HashMap<usize, usize>) -> usize {
    first.iter()
        .map(|(number, occurrences)|
            occurrences * number * second.get(number).copied().unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string()
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("11".to_string()));
    }

    #[test]
    fn check_input_part_1() -> AoCResult<()> {
        let input = get_input(2024, 1)?;
        assert_eq!(part_1(&input), Ok("1320851".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("31".to_string()));
    }

    #[test]
    fn check_input_part_2() -> AoCResult<()> {
        let input = get_input(2024, 1)?;
        assert_eq!(part_2(&input), Ok("26859182".to_string()));
        Ok(())
    }
}
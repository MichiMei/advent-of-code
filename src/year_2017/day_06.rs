use std::collections::HashMap;
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the starting block sizes.".to_string()))
    }
    let numbers = parse_numbers(&input[0])?;
    Ok(count_redistribution_loop(numbers).0.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the starting block sizes.".to_string()))
    }
    let numbers = parse_numbers(&input[0])?;
    Ok(count_redistribution_loop(numbers).1.to_string())
}

fn parse_numbers(line: &str) -> Result<Vec<usize>, AoCError<String>> {
    line.split_whitespace()
        .map(|word| word.parse())
        .collect::<Result<_, _>>()
        .map_err(|e| AoCError::BadInputFormat(
            format!("Parsing input numbers failed. {}", e)))
}

fn count_redistribution_loop(mut numbers: Vec<usize>) -> (usize, usize) {
    let mut cache = HashMap::new();
    while !cache.contains_key(&numbers) {
        cache.insert(numbers.clone(), cache.len());
        redistribute(&mut numbers);
    }
    let total_cycles = cache.len();
    let loop_start = *cache.get(&numbers).expect("Must be contained to exit while");
    (total_cycles, total_cycles-loop_start)
}

fn redistribute(numbers: &mut Vec<usize>) {
    let min = find_max(numbers);
    let len = numbers.len();
    let distribute_val = numbers[min]/len;
    let remainder = numbers[min]%len;
    numbers[min] = 0;
    for index in 1..=len {
        if index <= remainder {
            numbers[(min+index)%len] += distribute_val+1;
        } else {
            numbers[(min+index)%len] += distribute_val;
        }
    }
}

fn find_max(numbers: &[usize]) -> usize {
    let mut min_val = numbers[0];
    let mut min_index = 0;
    for (index, elem) in numbers.iter().enumerate() {
        if *elem > min_val {
            min_val = *elem;
            min_index = index;
        }
    }
    min_index
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["0 2 7 0".to_string()]), Ok("5".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 6)?;
        assert_eq!(part_1(&input), Ok("5042".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["0 2 7 0".to_string()]), Ok("4".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 6)?;
        assert_eq!(part_2(&input), Ok("1086".to_string()));
        Ok(())
    }
}
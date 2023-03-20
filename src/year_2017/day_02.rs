use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    calculate_checksum(input).map(|sum| sum.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    sum_divisible(input).map(|sum| sum.to_string())
}

fn calculate_checksum(input: &Vec<String>) -> Result<i32, AoCError<String>>{
    let mut sum = 0;
    for line in input {
        let numbers = parse_number(line)?;
        let min = numbers.iter().min().unwrap_or(&0);
        let max = numbers.iter().max().unwrap_or(&0);
        sum += max-min;
    }
    Ok(sum)
}

fn sum_divisible(input: &Vec<String>) -> Result<i32, AoCError<String>> {
    let mut sum = 0;
    for line in input {
        let numbers = parse_number(line)?;
        sum += get_divisible(&numbers)?;
    }
    Ok(sum)
}

fn parse_number(line: &str) -> Result<Vec<i32>, AoCError<String>> {
    line.split_whitespace()
        .map(|word| word.parse::<i32>()).collect::<Result<Vec<_>, _>>()
        .map_err(|e| AoCError::BadInputFormat(
            format!("Input can only contain integers. {}", e)))
}

fn get_divisible(numbers: &[i32]) -> Result<i32, AoCError<String>> {
    let mut res = None;
    for (index, num0) in numbers.iter().enumerate() {
        for num1 in numbers[index+1..].iter() {
            if num0%num1 == 0 {
                if res.is_none() {
                    res = Some(num0/num1);
                } else {
                    return Err(AoCError::NoSolutionFoundError(
                        "One line contains multiple divisible pairs.".to_string()))
                }
            } else if num1%num0 == 0 {
                if res.is_none() {
                    res = Some(num1/num0);
                } else {
                    return Err(AoCError::NoSolutionFoundError(
                        "One line contains multiple divisible pairs.".to_string()))
                }
            }
        }
    }
    res.ok_or_else(|| AoCError::NoSolutionFoundError(
        "One line contains no divisible pairs.".to_string()))
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "5 1 9 5".to_string(),
            "7 5 3".to_string(),
            "2 4 6 8".to_string(),
        ];

        assert_eq!(part_1(&v), Ok("18".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 2)?;
        assert_eq!(part_1(&input), Ok("41887".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = vec![
            "5 9 2 8".to_string(),
            "9 4 7 3".to_string(),
            "3 8 6 5".to_string(),
        ];
        assert_eq!(part_2(&v), Ok("9".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 2)?;
        assert_eq!(part_2(&input), Ok("226".to_string()));
        Ok(())
    }
}
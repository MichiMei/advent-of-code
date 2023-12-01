use std::string::ToString;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let values = input.iter()
        .map(|line| get_outer_digits(line))
        .map(|digits| digits.map(combine_digits))
        .collect::<Option<Vec<_>>>();
    sum_values(values)
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let values = input.iter()
        .map(|line| get_outer_digits(&transform_line(line)))
        .map(|digits| digits.map(combine_digits))
        .collect::<Option<Vec<_>>>();
    sum_values(values)
}

fn get_outer_digits(line: &str) -> Option<(u8, u8)> {
    let mut result = None;
    for char in line.chars() {
        if char.is_ascii_digit() {
            let digit = (char as u8) - b'0';
            if result.is_none() {
                result = Some((digit, digit));
            } else {
                result = result.map(|(f, _)| (f, digit));
            }
        }
    }
    result
}

fn combine_digits(digits: (u8, u8)) -> u32 {
    (digits.0 as u32) * 10 + (digits.1 as u32)
}

fn transform_line(line: &str) -> String {
    line.chars().enumerate()
        .filter_map(|(index, first_char)| {
            if first_char.is_ascii_digit() {
                Some(first_char)
            } else {
                starts_with_digit(&line[index..])
            }
        })
        .collect()
}

fn starts_with_digit(str: &str) -> Option<char> {
    for (pattern, char) in &DIGITS {
        if str.starts_with(pattern) {
            return Some(*char)
        }
    }
    None
}

const DIGITS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn sum_values(values: Option<Vec<u32>>) -> Result<String, AoCError<String>> {
    if let Some(values) = values {
        Ok(values.iter().sum::<u32>().to_string())
    } else {
        Err(AoCError::BadInputFormat("One of the lines contains no digit".to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let input = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];
        assert_eq!(part_1(&input), Ok("142".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 1)?;
        assert_eq!(part_1(&input), Ok("54331".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];
        assert_eq!(part_2(&input), Ok("281".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 1)?;
        assert_eq!(part_2(&input), Ok("54518".to_string()));
        Ok(())
    }
}
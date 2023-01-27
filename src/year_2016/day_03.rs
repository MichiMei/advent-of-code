use crate::errors::AoCError;
use crate::errors::AoCError::{BadInputFormat, UnexpectedInputLength};

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let mut count = 0;
    for line in input {
        let side_lengths = parse_line(line)?;
        if valid_triangle(side_lengths) {
            count += 1;
        }
    }

    Ok(count.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let mut count = 0;
    let iter = input.chunks(3);
    for triple in iter {
        for side_lengths in parse_triple(triple)? {
            if valid_triangle(side_lengths) {
                count += 1;
            }
        }
    }

    Ok(count.to_string())
}

fn parse_line(line: &str) -> Result<[usize; 3], AoCError<String>> {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.len() != 3 {
        return Err(BadInputFormat(format!("Expected 3 numbers per line, found {}.", words.len())))
    }
    Ok([
        words[0].parse().map_err(|e| BadInputFormat(format!("Expected only numbers, \
            '{}' could not be parsed:\n{}", words[0], e)))?,
        words[1].parse().map_err(|e| BadInputFormat(format!("Expected only numbers, \
            '{}' could not be parsed:\n{}", words[1], e)))?,
        words[2].parse().map_err(|e| BadInputFormat(format!("Expected only numbers, \
            '{}' could not be parsed:\n{}", words[2], e)))?,
    ])
}

fn parse_triple(triple: &[String]) -> Result<[[usize; 3]; 3], AoCError<String>> {
    if triple.len() != 3 {
        return Err(UnexpectedInputLength(format!("The line count should be divisible by 3")))
    }
    let side_lengths = triple.iter()
        .map(|line| parse_line(line))
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(side_lengths.len(), 3);
    assert_eq!(side_lengths[0].len(), 3);
    assert_eq!(side_lengths[1].len(), 3);
    assert_eq!(side_lengths[2].len(), 3);
    Ok([
        [side_lengths[0][0], side_lengths[1][0], side_lengths[2][0]],
        [side_lengths[0][1], side_lengths[1][1], side_lengths[2][1]],
        [side_lengths[0][2], side_lengths[1][2], side_lengths[2][2]],
    ])
}

fn valid_triangle(mut side_lengths: [usize; 3]) -> bool {
    side_lengths.sort_unstable();
    side_lengths[0] + side_lengths[1] > side_lengths[2]
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "15 5 25".to_string(),
            "15 30 25".to_string(),
        ];
        assert_eq!(part_1(&v), Ok("1".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_03.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("917".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_03.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("1649".to_string()));
        Ok(())
    }
}
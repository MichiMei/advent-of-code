use crate::errors::AoCError;
use crate::year_2017::lib_2017::knot_hash::KnotHash;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing a list of hash lengths.".to_string()))
    }
    let numbers = parse_numbers(&input[0])?;
    let mut knot_hash = KnotHash::new(255);
    knot_hash.execute_list(&numbers);
    knot_hash.get_start_product()
        .map(|res| res.to_string())
        .ok_or_else(|| AoCError::NoSolutionFoundError("Hash-Length < 2".to_string()))
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing a list of hash lengths.".to_string()))
    }
    let mut knot_hash = KnotHash::new(255);
    knot_hash.complete_hash(&input[0]);
    Ok(knot_hash.get_dense_hash())
}

fn parse_numbers(line: &str) -> Result<Vec<usize>, AoCError<String>> {
    line.split(',')
        .map(|word| word.parse()
        .map_err(|e| AoCError::BadInputFormat(format!(
            "Parsing input numbers failed. {}", e))))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let mut knot_hash = KnotHash::new(4);
        knot_hash.execute_list(&vec![3, 4, 1, 5]);
        assert_eq!(knot_hash.get_start_product(), Some(12));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 10)?;
        assert_eq!(part_1(&input), Ok("13760".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["".to_string()]),
                   Ok("a2582a3a0e66e6e86e3812dcb672a272".to_string()));
        assert_eq!(part_2(&vec!["AoC 2017".to_string()]),
                   Ok("33efeb34ea91902bb2f59c9920caa6cd".to_string()));
        assert_eq!(part_2(&vec!["1,2,3".to_string()]),
                   Ok("3efbe78a8d82f29979031a4aa0b16a9d".to_string()));
        assert_eq!(part_2(&vec!["1,2,4".to_string()]),
                   Ok("63960835bcdc130f0b66d7ff4f6a5a8e".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 10)?;
        assert_eq!(part_2(&input), Ok("2da93395f1a6bb3472203252e3b17fe5".to_string()));
        Ok(())
    }
}
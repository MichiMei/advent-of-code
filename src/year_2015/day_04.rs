use crate::errors::AoCError;
use crate::md5_collision::find_hash_collision_parallel;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Expected 1 line, found {} lines", input.len()))
        )
    }
    let input = input.first().unwrap();
    find_hash_collision_parallel(input, 0, 5)?
        .ok_or(AoCError::NoSolutionFoundError(String::new()))
        .map(|t| t.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Expected 1 line, found {} lines", input.len()))
        )
    }
    let input = input.first().unwrap();
    find_hash_collision_parallel(input, 0, 6)?
        .ok_or(AoCError::NoSolutionFoundError(String::new()))
        .map(|t| t.to_string())
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&["abcdef".to_string()]), Ok("609043".to_string()));
        assert_eq!(part_1(&["pqrstuv".to_string()]), Ok("1048970".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 4)?;
        assert_eq!(part_1(&input), Ok("282749".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 4)?;
        assert_eq!(part_2(&input), Ok("9962624".to_string()));
        Ok(())
    }
}
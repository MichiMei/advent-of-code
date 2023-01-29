use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Input should be 1 line, found {} lines", input.len())
        ))
    }
    let mut counter = 0;
    let line = input.first().unwrap();
    for char in line.chars() {
        match char {
            '(' => counter += 1,
            ')' => counter -= 1,
            c => {
                return Err(AoCError::BadInputFormat(
                    format!("Unexpected char '{}'. Only '(' and ')' allowed", c)
                ))
            },
        }
    }
    Ok(counter.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Input should be 1 line, found {} lines", input.len())
        ))
    }
    let mut height_counter = 0;
    let mut position_counter = 1usize;
    let line = input.first().unwrap();
    for char in line.chars() {
        match char {
            '(' => height_counter += 1,
            ')' => height_counter -= 1,
            c => {
                return Err(AoCError::BadInputFormat(
                    format!("Unexpected char '{}'. Only '(' and ')' allowed", c)
                ))
            },
        }
        if height_counter < 0 {
            assert_eq!(height_counter, -1);
            return Ok(position_counter.to_string())
        }
        position_counter += 1;
    }
    Err(AoCError::NoSolutionFoundError(
        "Santa never reaches the basement".to_string()
    ))
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert!(part_1(&["(())".to_string()]) == Ok("0".to_string()));
        assert!(part_1(&["()()".to_string()]) == Ok("0".to_string()));

        assert!(part_1(&["(((".to_string()]) == Ok("3".to_string()));
        assert!(part_1(&["(()(()(".to_string()]) == Ok("3".to_string()));

        assert!(part_1(&["))(((((".to_string()]) == Ok("3".to_string()));

        assert!(part_1(&["())".to_string()]) == Ok("-1".to_string()));
        assert!(part_1(&["))(".to_string()]) == Ok("-1".to_string()));

        assert!(part_1(&[")))".to_string()]) == Ok("-3".to_string()));
        assert!(part_1(&[")())())".to_string()]) == Ok("-3".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_01.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert!(part_1(&input) == Ok("138".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert!(part_2(&[")".to_string()]) == Ok("1".to_string()));

        assert!(part_2(&["()())".to_string()]) == Ok("5".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_01.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert!(part_2(&input) == Ok("1771".to_string()));
        Ok(())
    }
}
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    todo!()
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["input".to_string()]), Ok("expected".to_string())); // TODO
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 6)?;
        assert_eq!(part_1(&input), Ok("expected".to_string())); // TODO
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["input".to_string()]), Ok("expected".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 6)?;
        assert_eq!(part_2(&input), Ok("expected".to_string())); // TODO
        Ok(())
    }
}
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected only a single line containing a digit string".to_string()))
    }
    let sum = sum_repeated_digits(&input[0])?;
    Ok(sum.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected only a single line containing a digit string".to_string()))
    }
    let sum = sum_halfway_around_digits(&input[0])?;
    Ok(sum.to_string())
}

fn sum_repeated_digits(line: &str) -> Result<u32, AoCError<String>> {
    if line.len() < 2 {
        return Ok(0)
    }
    let mut iter = line.chars();
    let first = iter.next().expect("String cannot be empty");
    let mut prev = first;
    let mut count = 0;
    for next in iter {
        if next == prev  {
            count += next.to_digit(10)
                .ok_or_else(|| AoCError::BadInputFormat(format!(
                    "Input contains non digit character: '{}'.", next)))?;
        }
        prev = next;
    }
    if prev == first {
        count += prev.to_digit(10)
            .ok_or_else(|| AoCError::BadInputFormat(format!(
                "Input contains non digit character: '{}'.", prev)))?;
    }
    Ok(count)
}

fn sum_halfway_around_digits(line: &str) -> Result<u32, AoCError<String>> {
    if line.len()%2 != 0 {
        return Err(AoCError::BadInputFormat(
            "String contains odd number of characters".to_string()));
    }
    let index = line.len()/2;
    let fist_half = line[0..index].chars();
    let second_half = line[index..].chars();
    let iter = fist_half.zip(second_half);
    let mut count = 0;
    for (c0, c1) in iter {
        if c0 == c1 {
            count += 2 * c0.to_digit(10)
                .ok_or_else(|| AoCError::BadInputFormat(format!(
                    "Input contains non digit character: '{}'.", c0)))?;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["1122".to_string()]), Ok("3".to_string()));
        assert_eq!(part_1(&vec!["1111".to_string()]), Ok("4".to_string()));
        assert_eq!(part_1(&vec!["1234".to_string()]), Ok("0".to_string()));
        assert_eq!(part_1(&vec!["91212129".to_string()]), Ok("9".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_01.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("1102".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["1212".to_string()]), Ok("6".to_string()));
        assert_eq!(part_2(&vec!["1221".to_string()]), Ok("0".to_string()));
        assert_eq!(part_2(&vec!["123425".to_string()]), Ok("4".to_string()));
        assert_eq!(part_2(&vec!["123123".to_string()]), Ok("12".to_string()));
        assert_eq!(part_2(&vec!["12131415".to_string()]), Ok("4".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_01.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("1076".to_string()));
        Ok(())
    }
}
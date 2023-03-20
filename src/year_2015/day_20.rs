use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(format!(
            "Input is expected to be exactly one line. Found: {}", input.len()
        )))
    }
    let target = input[0].parse::<usize>().map_err(|e| AoCError::BadInputFormat(
        format!("Could not parse the input number. Found: {}\n{}", input[0], e)))?;

    let vec = calculate_array(target/10);
    let index = find_first_bigger(&vec, target/10).ok_or_else(
        || AoCError::NoSolutionFoundError(String::new()))?;

    Ok(index.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(format!(
            "Input is expected to be exactly one line. Found: {}", input.len()
        )))
    }
    let target = input[0].parse::<usize>().map_err(|e| AoCError::BadInputFormat(
        format!("Could not parse the input number. Found: {}\n{}", input[0], e)))?;

    let vec = calculate_array_50(target/11+1);
    let index = find_first_bigger_with_factor(&vec, target, 11).ok_or_else(
        || AoCError::NoSolutionFoundError(String::new()))?;

    Ok(index.to_string())
}

fn calculate_array(max_house: usize) -> Vec<usize> {
    let mut vec = vec![1; max_house];
    for elf in 2..=max_house {
        for house in (elf..=max_house).step_by(elf) {
            vec[house-1] += elf;
        }
        if vec[elf-1] >= max_house {
            break
        }
    }
    vec
}

fn calculate_array_50(max_house: usize) -> Vec<usize> {
    let mut vec = vec![1; max_house];
    for elf in 2..=max_house {
        for house in 1..=50 {
            let index = house*elf;
            if index < vec.len() {
                vec[index - 1] += elf;
            }
        }
    }
    vec
}

fn find_first_bigger(vec: &[usize], target: usize) -> Option<usize> {
    for (index, val) in vec.iter().enumerate() {
        if *val >= target {
            return Some(index+1)
        }
    }
    None
}

fn find_first_bigger_with_factor(vec: &[usize], target: usize, factor: usize) -> Option<usize> {
    for (index, val) in vec.iter().enumerate() {
        if *val*factor >= target {
            return Some(index+1)
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&["10".to_string()]), Ok("1".to_string()));
        assert_eq!(part_1(&["70".to_string()]), Ok("4".to_string()));
        assert_eq!(part_1(&["150".to_string()]), Ok("8".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 20)?;
        assert_eq!(part_1(&input), Ok("665280".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&["100".to_string()]), Ok("6".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 20)?;
        assert_eq!(part_2(&input), Ok("705600".to_string()));
        Ok(())
    }
}
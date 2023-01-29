pub fn part_1(input: &[String]) -> Result<String, &str> {
    if input.len() != 1 {
        return Err(ERR_INPUT_MALFORMED)
    }
    let target = input[0].parse::<usize>().map_err(|_| ERR_INPUT_MALFORMED)?;

    let vec = calculate_array(target/10);
    let index = find_first_bigger(&vec, target/10).ok_or(ERR_INPUT_MALFORMED)?;

    Ok(index.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, &str> {
    if input.len() != 1 {
        return Err(ERR_INPUT_MALFORMED)
    }
    let target = input[0].parse::<usize>().map_err(|_| ERR_INPUT_MALFORMED)?;

    let vec = calculate_array_50(target/11+1);
    let index = find_first_bigger_with_factor(&vec, target, 11).ok_or(ERR_INPUT_MALFORMED)?;

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

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&["10".to_string()]), Ok("1".to_string()));
        assert_eq!(part_1(&["70".to_string()]), Ok("4".to_string()));
        assert_eq!(part_1(&["150".to_string()]), Ok("8".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_20.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("665280".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&["100".to_string()]), Ok("6".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_20.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("705600".to_string()));
        Ok(())
    }
}
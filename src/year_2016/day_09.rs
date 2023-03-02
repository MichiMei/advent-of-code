use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Expected one line of input. Found {} lines", input.len())))
    }

    Ok(decompress_length(&input[0], false)?.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Expected one line of input. Found {} lines", input.len())))
    }

    Ok(decompress_length(&input[0], true)?.to_string())
}

/// Calculates the length of the decompressed string.
fn decompress_length(str: &str, recursive: bool) -> Result<usize, AoCError<String>> {
    let mut remaining = str;
    let mut char_count = 0;

    while !remaining.is_empty() {
        char_count += move_to_bracket(&mut remaining);
        if remaining.is_empty() {
            break;
        }
        if let Some((length, repetitions)) = extract_marker(&mut remaining) {
            if remaining.len() < length {
                return Err(AoCError::BadInputFormat("A marker is out of bounds. It either exceeds \
                    the input of the length of a parent marker.".to_string()));
            }
            let split = remaining.split_at(length);
            let repetition_str = split.0;
            remaining = split.1;
            let marker_length = if recursive {
                decompress_length(repetition_str, recursive)?
            } else {
                repetition_str.len()
            };
            char_count += marker_length * repetitions;
        } else {
            char_count += 1;
            remaining = &remaining[1..];
        }
    }

    Ok(char_count)
}

/// Removes all characters from the given slice until a '(' is found.
/// Returns the number of removed chars
fn move_to_bracket(str: &mut &str) -> usize {
    let length;
    if let Some(index) = str.find('(') {
        length = index;
        *str = &str[index..];
    } else {
        length = str.len();
        *str = &str[0..0];
    }
    length
}

/// Removes the marker (in the beginning of the given slice) from the slice and parses it.
/// If no marker is found or the marker is malformed None is returned and the slice remains
/// unchanged.
fn extract_marker(str: &mut &str) -> Option<(usize, usize)> {
    let close_index = str.find(')')?;

    let (marker, remaining) = str.split_at(close_index+1);

    if let Some(marker) = parse_marker(marker) {
        *str = remaining;
        Some(marker)
    } else {
        None
    }
}

/// Parses a marker from the given slice.
/// Returns  None if the slice is malformed.
fn parse_marker(str: &str) -> Option<(usize, usize)> {

    let (bracket_open, remaining) = str.split_at(1);
    let (marker, bracket_close) = remaining.split_at(str.len()-2);

    if bracket_open != "(" || bracket_close != ")" {
        return None
    }
    let values = marker.split('x')
        .map(|int_str| int_str.parse::<usize>().ok())
        .collect::<Option<Vec<_>>>()?;

    if values.len() != 2 {
        return None
    }

    Some((values[0], values[1]))
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["ADVENT".to_string()]), Ok("6".to_string()));
        assert_eq!(part_1(&vec!["A(1x5)BC".to_string()]), Ok("7".to_string()));
        assert_eq!(part_1(&vec!["(3x3)XYZ".to_string()]), Ok("9".to_string()));
        assert_eq!(part_1(&vec!["A(2x2)BCD(2x2)EFG".to_string()]), Ok("11".to_string()));
        assert_eq!(part_1(&vec!["(6x1)(1x3)A".to_string()]), Ok("6".to_string()));
        assert_eq!(part_1(&vec!["X(8x2)(3x3)ABCY".to_string()]), Ok("18".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_09.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("138735".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["(3x3)XYZ".to_string()]), Ok("9".to_string()));
        assert_eq!(part_2(&vec!["X(8x2)(3x3)ABCY".to_string()]), Ok("20".to_string()));
        assert_eq!(part_2(&vec!["(27x12)(20x12)(13x14)(7x10)(1x12)A".to_string()]), Ok("241920".to_string()));
        assert_eq!(part_2(&vec!["(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_string()]), Ok("445".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_09.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("11125026826".to_string()));
        Ok(())
    }
}
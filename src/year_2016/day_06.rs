use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    if input.is_empty() {
        return Err(AoCError::UnexpectedInputLength(
            "Input needs to have at least one line.".to_string()
        ))
    }
    let mut iter = input.iter();
    let first = iter.next().expect("Is possible as length >= 1");
    let mut error_corrector = ErrorCorrector::from_line(first)?;
    for line in iter {
        error_corrector.add_line(line)?;
    }
    if let Some(code) = error_corrector.get_most_frequent_code() {
        Ok(code)
    } else {
        Err(AoCError::NoSolutionFoundError(
            "No solution was found. At least one char is not unique.".to_string()
        ))
    }
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    if input.is_empty() {
        return Err(AoCError::UnexpectedInputLength(
            "Input needs to have at least one line.".to_string()
        ))
    }
    let mut iter = input.iter();
    let first = iter.next().expect("Is possible as length >= 1");
    let mut error_corrector = ErrorCorrector::from_line(first)?;
    for line in iter {
        error_corrector.add_line(line)?;
    }
    if let Some(code) = error_corrector.get_least_frequent_code() {
        Ok(code)
    } else {
        Err(AoCError::NoSolutionFoundError(
            "No solution was found. At least one char is not unique.".to_string()
        ))
    }
}

struct ErrorCorrector {
    chars: Vec<CharCounter>,
}

impl ErrorCorrector {
    fn from_line(line: &str) -> Result<Self, AoCError<String>> {
        let length = line.len();
        let mut new = Self::new(length);
        new.add_line(line)?;
        Ok(new)
    }

    fn new(length: usize) -> Self {
        let chars = vec![Default::default(); length];
        Self{chars}
    }

    fn add_line(&mut self, line: &str) -> Result<(), AoCError<String>> {
        if self.chars.len() != line.len() {
            return Err(AoCError::BadInputFormat(
                format!("A input line has the wrong size ({}), expected {}. \
                    All lines must have equal length.", line.len(), self.chars.len())
            ))
        }
        for (index, char) in line.chars().enumerate() {
            self.chars[index].add_char(char)?;
        }
        Ok(())
    }

    fn get_most_frequent_code(&self) -> Option<String> {
        self.chars.iter().map(|c| c.get_most_frequent()).collect()
    }

    fn get_least_frequent_code(&self) -> Option<String> {
        self.chars.iter().map(|c| c.get_least_frequent()).collect()
    }
}

#[derive(Debug, Clone, Default)]
struct CharCounter {
    counts: [usize; 26],
    max_count: usize,
    max_char: Option<char>,
}

impl CharCounter {
    fn add_char(&mut self, char: char) -> Result<(), AoCError<String>> {
        let index = Self::char_to_usize(char)
            .ok_or_else(|| AoCError::BadInputFormat(
                format!("Unexpected character '{}'. Only a-z allowed.", char)
            ))?;
        assert!(index < self.counts.len());
        let new_count = self.counts[index] + 1;
        self.counts[index] = new_count;
        if new_count == self.max_count {
            self.max_char = None;
        }
        if new_count > self.max_count {
            self.max_count = new_count;
            self.max_char = Some(char);
        }
        Ok(())
    }

    fn get_most_frequent(&self) -> Option<char> {
        self.max_char
    }

    fn get_least_frequent(&self) -> Option<char> {
        let mut min_count = None;
        let mut min_index = None;
        for (index, count) in self.counts.iter().enumerate() {
            if *count == 0 {
                continue
            }
            if min_count.is_none() || count < min_count.unwrap() {
                min_count = Some(count);
                min_index = Some(index);
            } else if count == min_count.unwrap() {
                min_index = None;
            }
        }
        min_index.and_then(Self::usize_to_char)
    }

    fn char_to_usize(char: char) -> Option<usize> {
        if !('a'..='z').contains(&char) {
            return None
        }
        let index = (char as usize) - ('a' as usize);
        assert!(index < 26);
        Some(index)
    }

    fn usize_to_char(index: usize) -> Option<char> {
        if index >= 26 {
            return None
        }
        let tmp = index + ('a' as usize);
        assert!(tmp <= u8::MAX as usize);
        let char = tmp as u8 as char;
        assert!(('a'..='z').contains(&char));
        Some(char)
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "eedadn".to_string(),
            "drvtee".to_string(),
            "eandsr".to_string(),
            "raavrd".to_string(),
            "atevrs".to_string(),
            "tsrnev".to_string(),
            "sdttsa".to_string(),
            "rasrtv".to_string(),
            "nssdts".to_string(),
            "ntnada".to_string(),
            "svetve".to_string(),
            "tesnvt".to_string(),
            "vntsnd".to_string(),
            "vrdear".to_string(),
            "dvrsen".to_string(),
            "enarar".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let v = get_example_input();

        assert_eq!(part_1(&v), Ok("easter".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 6)?;
        assert_eq!(part_1(&input), Ok("ygjzvzib".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = get_example_input();

        assert_eq!(part_2(&v), Ok("advent".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 6)?;
        assert_eq!(part_2(&input), Ok("pdesmnoz".to_string()));
        Ok(())
    }
}
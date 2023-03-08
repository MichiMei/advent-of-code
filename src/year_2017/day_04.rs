use std::collections::HashSet;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    Ok(count_valid_passphrases(input, not_contains_equal_words).to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    Ok(count_valid_passphrases(input, not_contains_anagrams).to_string())
}

fn count_valid_passphrases(input: &[String], validity_function: fn(&str) ->bool) -> usize {
    input.iter()
        .map(|line| validity_function(line))
        .filter(|b| *b)
        .count()
}

fn not_contains_equal_words(line: &str) -> bool {
    let mut count = 0;
    let mut set = HashSet::new();
    for word in line.split_whitespace() {
        count += 1;
        set.insert(word);
    }
    count == set.len()
}

fn not_contains_anagrams(line: &str) -> bool {
    let mut count = 0;
    let mut set = HashSet::new();
    for word in line.split_whitespace() {
        count += 1;
        let mut sorted_chars = word.chars().collect::<Vec<_>>();
        sorted_chars.sort_unstable();
        set.insert(sorted_chars);
    }
    count == set.len()
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "aa bb cc dd ee".to_string(),
            "aa bb cc dd aa".to_string(),
            "aa bb cc dd aaa".to_string(),
        ];
        assert_eq!(part_1(&v), Ok("2".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_04.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("337".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = vec![
            "abcde fghij".to_string(),
            "abcde xyz ecdab".to_string(),
            "a ab abc abd abf abj".to_string(),
            "iiii oiii ooii oooi oooo".to_string(),
            "oiii ioii iioi iiio".to_string(),
        ];
        assert_eq!(part_2(&v), Ok("3".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_04.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("231".to_string()));
        Ok(())
    }
}
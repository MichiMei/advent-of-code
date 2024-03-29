use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut count = 0;
    for line in input {
        if is_nice_part_1(line) {
            count += 1;
        }
    }
    Ok(count.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut count = 0;
    for line in input {
        if is_nice_part_2(line) {
            count += 1;
        }
    }
    Ok(count.to_string())
}

fn is_nice_part_1(str: &str) -> bool {
    contains_3_vowels(str) && contains_double(str) && !contains_bad_strings(str)
}

fn contains_3_vowels(str: &str) -> bool {
    let mut count = 0;
    for c in str.chars() {
        match c {
            'a' => count += 1,
            'e' => count += 1,
            'i' => count += 1,
            'o' => count += 1,
            'u' => count += 1,
            _ => {}
        }
    }
    count >= 3
}

fn contains_double(str: &str) -> bool {
    let mut chars = str.chars();
    let mut prev = match chars.next() {
        None => return false,
        Some(val) => val,
    };
    for next in chars {
        if prev == next {
            return true
        }
        prev = next;
    }
    false
}

fn contains_bad_strings(str: &str) -> bool {
    if str.contains("ab") {
        return true
    }
    if str.contains("cd") {
        return true
    }
    if str.contains("pq") {
        return true
    }
    if str.contains("xy") {
        return true
    }
    false
}

fn is_nice_part_2(str: &str) -> bool {
    contains_duplicate_pair(str) && contains_blank_letter_repetition(str)
}

fn contains_duplicate_pair(str: &str) -> bool {
    for index in 0..str.len()-1 {
        if str[index+2..].contains(&str[index..index+2]) {
            return true
        }
    }
    false
}

fn contains_blank_letter_repetition(str: &str) -> bool {
    let mut chars = str.chars();
    let mut pre_prev = match chars.next() {
        None => return false,
        Some(val) => val,
    };
    let mut prev = match chars.next() {
        None => return false,
        Some(val) => val
    };
    for next in chars {
        if next == pre_prev {
            return true
        }
        pre_prev = prev;
        prev = next;
    }
    false
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&["ugknbfddgicrmopn".to_string()]), Ok("1".to_string()));
        assert_eq!(part_1(&["aaa".to_string()]), Ok("1".to_string()));
        assert_eq!(part_1(&["jchzalrnumimnmhp".to_string()]), Ok("0".to_string()));
        assert_eq!(part_1(&["haegwjzuvuyypxyu".to_string()]), Ok("0".to_string()));
        assert_eq!(part_1(&["dvszwmarrgswjxmb".to_string()]), Ok("0".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 5)?;
        assert_eq!(part_1(&input), Ok("238".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&["qjhvhtzxzqqjkmpb".to_string()]), Ok("1".to_string()));
        assert_eq!(part_2(&["xxyxx".to_string()]), Ok("1".to_string()));
        assert_eq!(part_2(&["uurcxstgmygtbstg".to_string()]), Ok("0".to_string()));
        assert_eq!(part_2(&["ieodomkazucvgmuy".to_string()]), Ok("0".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 5)?;
        assert_eq!(part_2(&input), Ok("69".to_string()));
        Ok(())
    }
}
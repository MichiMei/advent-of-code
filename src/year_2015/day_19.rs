use std::collections::HashSet;
use std::mem::swap;

pub fn part_1(input: &Vec<String>) -> Result<String, &str> {
    if input.len() < 3 || !input[input.len()-2].is_empty() {
        return Err(ERR_INPUT_MALFORMED)
    }
    let replacements = parse_replacements(&input[0..input.len()-2])?;
    let start_str = &input[input.len()-1];

    let mut result_str = HashSet::new();
    for repl in replacements.iter() {
        result_str.extend(replace(start_str, repl));
    }

    Ok(result_str.len().to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, &str> {
    /*if input.len() < 3 || !input[input.len()-2].is_empty() {
        return Err(ERR_INPUT_MALFORMED)
    }
    let replacements = parse_replacements(&input[0..input.len()-2])?;
    let goal = &input[input.len()-1];
    let mut string_set = HashSet::new();
    string_set.insert(goal.to_string());
    let mut tmp = HashSet::new();
    let mut count = 0;

    while !string_set.contains("e") {
        for str in string_set.iter() {
            for repl in replacements.iter() {
                tmp.extend(replace_reverse(&str, &repl));
            }
        }
        swap(&mut string_set, &mut tmp);
        tmp.clear();
        count += 1;
        println!("{} -> {}", count, string_set.len());
    }

    Ok(count.to_string())*/
    unimplemented!()
}

fn parse_replacements(input: &[String]) -> Result<Vec<Replacement>, &str> {
    let mut replacements = vec![];
    for line in input {
        replacements.push(Replacement::from(line)?);
    }
    Ok(replacements)
}

fn replace(start_str: &str, repl: &Replacement) -> HashSet<String> {
    let mut res = HashSet::new();
    let mut last = 0;
    while let Some(location) = start_str[last..].find(&repl.pattern) {
        let location = last+location;
        let s0 = start_str[0..location].to_string();
        let s1 = &repl.replacement[..];
        let s2 = &start_str[location+repl.pattern.len()..];
        let new = String::from(s0+s1+s2);
        res.insert(new);
        last = location+1;
    }
    res
}

fn replace_reverse(start_str: &str, repl: &Replacement) -> HashSet<String> {
    let mut res = HashSet::new();
    let mut last = 0;
    while let Some(location) = start_str[last..].find(&repl.replacement) {
        let location = last+location;
        let s0 = start_str[0..location].to_string();
        let s1 = &repl.pattern[..];
        let s2 = &start_str[location+repl.replacement.len()..];
        let new = String::from(s0+s1+s2);
        res.insert(new);
        last = location+1;
    }
    res
}

struct Replacement {
    pattern: String,
    replacement: String,
}

impl Replacement {
    fn from(str: &str) -> Result<Self, &str> {
        let words: Vec<&str> = str.split(" => ").collect();
        if words.len() != 2 {
            return Err(ERR_INPUT_MALFORMED)
        }
        let pattern = words[0].to_string();
        let replacement = words[1].to_string();
        if pattern.is_empty() || replacement.is_empty() {
            return Err(ERR_INPUT_MALFORMED)
        }
        Ok(Self{pattern, replacement})
    }
}

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v0 = vec![
            "H => HO".to_string(),
            "H => OH".to_string(),
            "O => HH".to_string(),
            "".to_string(),
            "HOH".to_string()
        ];
        assert_eq!(part_1(&v0), Ok("4".to_string()));

        let v1 = vec![
            "H => HO".to_string(),
            "H => OH".to_string(),
            "O => HH".to_string(),
            "".to_string(),
            "HOHOHO".to_string()
        ];
        assert_eq!(part_1(&v1), Ok("7".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_19.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("535".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v0 = vec![
            "e => H".to_string(),
            "e => O".to_string(),
            "H => HO".to_string(),
            "H => OH".to_string(),
            "O => HH".to_string(),
            "".to_string(),
            "HOH".to_string()
        ];
        assert_eq!(part_2(&v0), Ok("3".to_string()));

        let v1 = vec![
            "e => H".to_string(),
            "e => O".to_string(),
            "H => HO".to_string(),
            "H => OH".to_string(),
            "O => HH".to_string(),
            "".to_string(),
            "HOHOHO".to_string()
        ];
        assert_eq!(part_2(&v1), Ok("6".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_19.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("expected".to_string())); // TODO
        Ok(())
    }
}
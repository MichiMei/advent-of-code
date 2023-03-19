use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::mem::swap;
use crate::errors::AoCError;
use crate::output::bool_slice_to_string;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let rules = parse_rules(input)?;
    let mut picture = Picture::create(create_start_pattern());
    extend_picture(&mut picture, &rules, 5)
        .map(|sum| sum.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let rules = parse_rules(input)?;
    let mut picture = Picture::create(create_start_pattern());
    extend_picture(&mut picture, &rules, 18)
        .map(|sum| sum.to_string())
}

fn parse_rules(input: &[String]) -> Result<Rules, AoCError<String>> {
    let mut res = HashMap::new();
    for line in input {
        let rules = parse_rule(line)?;
        res.extend(rules)
    }
    Ok(res)
}

fn parse_rule(line: &str) -> Result<Vec<(Pattern, Pattern)>, AoCError<String>> {
    let parts = line.split(" => ").collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(AoCError::BadInputFormat(format!(
            "Malformed rule, expected '<pattern> => <replacement>', found '{}'.", line)))
    }
    let pattern = parse_pattern(parts[0])?;
    let replacement = parse_pattern(parts[1])?;
    let patterns = get_modified_patterns(pattern);
    Ok(patterns.into_iter().map(|pattern| (pattern, replacement.clone())).collect())
}

fn parse_pattern(str: &str) -> Result<Pattern, AoCError<String>> {
    let words = str.split('/').collect::<Vec<_>>();
    let size = words.len();
    if !(2..=4).contains(&size) {
        return Err(AoCError::BadInputFormat(format!(
            "Only patterns of sizes 2x2, 3x3 and 4x4 supported. Pattern '{}' has {} lines."
            , str, size)))
    }
    let mut data = vec![];
    for word in words {
        if word.len() != size {
            return Err(AoCError::BadInputFormat(format!(
                "Only patterns of sizes 2x2, 3x3 and 4x4 supported. Pattern '{}' has {} columns."
                , str, word.len())))
        }
        let line = word.chars()
            .map(|c| {
                match c {
                    '.' => Ok(false),
                    '#' => Ok(true),
                    _ => Err(AoCError::BadInputFormat(format!(
                        "Malformed pattern, only chars '.' and '#' supported, found '{}'.", c)))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        data.push(line);
    }
    Ok(data)
}

fn get_modified_patterns(pattern: Pattern) -> Vec<Pattern> {
    let mut res = get_reflections(transpose(&pattern));
    res.extend(get_reflections(pattern));
    res
}

fn get_reflections(pattern: Pattern) -> Vec<Pattern> {
    let vertical = reflection_v(&pattern);
    let horizontal = reflection_h(&pattern);
    let point = reflection_h(&vertical);
    vec![pattern, vertical, horizontal, point]
}

fn reflection_h(pattern: &Pattern) -> Pattern {
    let mut res = pattern.clone();
    res.iter_mut().for_each(|line| line.reverse());
    res
}

fn reflection_v(pattern: &Pattern) -> Pattern {
    let mut res = pattern.clone();
    res.reverse();
    res
}

fn transpose(pattern: &Pattern) -> Pattern {
    let mut data = vec![vec![false; pattern[0].len()]; pattern.len()];
    for (line_index, line) in pattern.iter().enumerate() {
        for (col_index, elem) in line.iter().enumerate() {
            if *elem {
                data[col_index][line_index] = true;
            }
        }
    }
    data
}

fn create_start_pattern() -> Pattern {
    vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true]]
}

fn extend_picture(picture: &mut Picture, rules: &Rules, iterations: usize)
    -> Result<usize, AoCError<String>> {
    for _ in 0..iterations {
        picture.extend(rules)?;
    }
    Ok(picture.count_trues())
}

struct Picture {
    patterns: HashMap<Pattern, usize>,
}

impl Picture {
    fn create(pattern: Pattern) -> Self {
        let mut patterns = HashMap::new();
        patterns.insert(pattern, 1);
        Self{patterns}
    }

    fn extend(&mut self, rules: &Rules) -> Result<(), AoCError<String>> {
        let mut old_patterns: HashMap<Pattern, usize>  = HashMap::new();
        swap(&mut old_patterns, &mut self.patterns);

        for (pattern, count) in old_patterns.iter() {
            let (pattern_split, split_count) = split_pattern(pattern);
            let patterns_replaced = replace_views(pattern_split, rules)?;
            if patterns_replaced[0].len() == 3 && patterns_replaced.len() == 9 {
                // only independent 3x3 patterns -> dont recombine
                for pattern in patterns_replaced {
                    assert_eq!(pattern.len(), 3);
                    self.add_pattern(pattern.clone(), *count);
                }
            } else {
                // inter-dependent patterns -> recombine
                let pattern_combined =
                    recombine_pattern(patterns_replaced, split_count);
                self.add_pattern(pattern_combined, *count);
            }
        }
        Ok(())
    }

    fn add_pattern(&mut self, pattern: Pattern, count: usize) {
        let old_count = *self.patterns.get(&pattern).unwrap_or(&0);
        self.patterns.insert(pattern, old_count+count);
    }

    fn count_trues(&self) -> usize {
        self.patterns.iter()
            .map(|(pattern, count)| count*count_trues(pattern))
            .sum()
    }
}

impl Display for Picture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (pattern, count) in self.patterns.iter() {
            writeln!(f, "{}:", count)?;
            for line in pattern.iter() {
                writeln!(f, "\t{}", bool_slice_to_string(line))?;
            }
        }
        writeln!(f,)
    }
}

/// Splits a pattern into multiple patterns.
/// If side-length is evenly divisible  by 2, returns views of size 2x2, otherwise 3x3
fn split_pattern(pattern: &Pattern) -> (Vec<Pattern>, usize) {
    let split_size = if pattern.len() % 2 == 0 {
        2
    } else if pattern.len() % 3 == 0 {
        3
    } else {
        panic!("All patterns should be divisible by 2 or 3");
    };
    let mut res = vec![];
    for line_chunk in pattern.chunks_exact(split_size) {
        for start in (0..line_chunk[0].len()).step_by(split_size) {
            let data = line_chunk.iter().map(|line| line[start..start+split_size].to_vec()).collect::<Vec<_>>();
            assert_eq!(data.len(), split_size);
            data.iter().for_each(|line| assert_eq!(line.len(), split_size));
            res.push(data)
        }
    }
    (res, pattern.len()/split_size)
}

fn replace_views(patterns: Vec<Pattern>, rules: &Rules) -> Result<Vec<&Pattern>, AoCError<String>> {
    patterns.iter()
        .map(|pattern| rules.get(pattern))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| AoCError::NoSolutionFoundError(
            "A pattern was not found in the replacement rules.".to_string()))
}

fn recombine_pattern(views: Vec<&Pattern>, split_count: usize) -> Pattern {
    assert_eq!(split_count*split_count, views.len());
    let mut data = vec![];
    for chunk in views.chunks_exact(split_count) {
        for line_index in 0..chunk[0].len() {
            let line = chunk.iter()
                .flat_map(|pattern| pattern[line_index].iter().copied())
                .collect();
            data.push(line);
        }
    }
    data
}

fn count_trues(pattern: &Pattern) -> usize {
    pattern.iter()
        .flat_map(|line| line.iter())
        .filter(|b| **b)
        .count()
}

type Pattern = Vec<Vec<bool>>;
type Rules = HashMap<Pattern, Pattern>;

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() -> Result<(), AoCError<String>> {
        let v = vec![
            "../.# => ##./#../...".to_string(),
            ".#./..#/### => #..#/..../..../#..#".to_string(),
        ];
        let rules = parse_rules(&v)?;
        let mut picture = Picture::create(create_start_pattern());
        assert_eq!(extend_picture(&mut picture, &rules, 2), Ok(12));
        Ok(())
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_21.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("142".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_21.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("1879071".to_string()));
        Ok(())
    }
}
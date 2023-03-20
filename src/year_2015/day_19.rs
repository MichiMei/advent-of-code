use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::ops::{Add, Sub};
use std::str::Chars;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() < 3 || !input[input.len()-2].is_empty() {
        return Err(AoCError::UnexpectedInputLength("The input has to be at least 3 lines long and \
            the penultimate line needs to be empty.".to_string()))
    }
    let replacements = parse_replacements(&input[0..input.len()-2])?;
    let start_str = &input[input.len()-1];

    let mut result_str = HashSet::new();
    for repl in replacements.iter() {
        result_str.extend(replace(start_str, repl));
    }

    Ok(result_str.len().to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let elems = Elements::from(&input[0..input.len()-2])?;
    let rules = parse_rules(&input[0..input.len()-2], &elems)?;
    let goal = Molecule::from_string(&input[input.len()-1], &elems)?;
    let start = Molecule::from_string("e", &elems)?;

    let rules_vec: Vec<Molecule> = rules.into_iter().collect();
    let res = build_molecule_rule_wise(&start, &rules_vec, &goal, &elems,
                                       0, goal.count(), "")
        .ok_or_else(|| AoCError::BadInputFormat(
            "The replacement rules could not be parsed".to_string()))?;

    Ok(res.to_string())
}

fn parse_replacements(input: &[String]) -> Result<Vec<Replacement>, AoCError<String>> {
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
        let new = s0 + s1 + s2;
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
    fn from(str: &str) -> Result<Self, AoCError<String>> {
        let words: Vec<&str> = str.split(" => ").collect();
        if words.len() != 2 {
            return Err(AoCError::BadInputFormat(format!(
                "Replacement line malformed. Expected '<pattern> => <replacement>.\nFound: '{}'",
                str
            )))
        }
        let pattern = words[0].to_string();
        let replacement = words[1].to_string();
        if pattern.is_empty() || replacement.is_empty() {
            return Err(AoCError::BadInputFormat(format!(
                "Replacement line malformed. Expected '<pattern> => <replacement>.\nFound: '{}'",
                str
            )))
        }
        Ok(Self{pattern, replacement})
    }
}

fn build_molecule_rule_wise(molecule: &Molecule, rules: &[Molecule], goal: &Molecule,
                            elems: &Elements, r_count: usize, max: usize, dbg: &str)
        -> Option<usize> {
    if rules.is_empty() {
        return if molecule == goal {
            let m_count =
                goal.count_intermediates(elems) - molecule.count_intermediates(elems);
            Some(m_count + r_count)
        } else {
            None
        }
    }
    let rule = &rules[0];

    let mut min = build_molecule_rule_wise(molecule, &rules[1..], goal, elems,
                                           r_count, max, &format!("{} 0", dbg));
    let mut new_molecule = molecule.clone();
    for c in 1..=max {
        new_molecule = &new_molecule + rule;
        if !new_molecule.is_valid(goal) {
            break;
        }
        let tmp = build_molecule_rule_wise(&new_molecule, &rules[1..], goal,
                                           elems, r_count+c, max,
                                           &format!("{} {}", dbg, c));
        if min.is_none() || (tmp.is_some() && min.unwrap() < tmp.unwrap()) {
            min = tmp;
        }
    }
    min
}

fn parse_rules(input: &[String], elems: &Elements) -> Result<HashSet<Molecule>, AoCError<String>> {
    let mut rules = HashSet::new();
    for line in input.iter() {
        let tmp = Molecule::from_rule(line, elems)?;
        rules.insert(tmp);
    }
    Ok(rules)
}

#[derive(Debug)]
struct Elements {
    elements: Vec<String>,
    terminal: Vec<bool>,
    index: HashMap<String, usize>,
    template_intermediate: Option<usize>,
}

impl Elements {
    fn new() -> Self {
        Self {
            elements: vec![],
            terminal: vec![],
            index: HashMap::new(),
            template_intermediate: None,
        }
    }

    fn from(input: &[String]) -> Result<Self, AoCError<String>> {
        let mut elems = Elements::new();

        for line in input.iter() {
            let words: Vec<&str> = line.split(" => ").collect();
            if words.len() != 2 {
                return Err(AoCError::BadInputFormat(format!(
                    "Replacement line malformed. Expected '<pattern> => <replacement>.\
                    \nFound: '{}'", line
                )))
            }
            let mut chars = words[0].chars().peekable();
            let tmp = parse_elem(&mut chars)
                .ok_or_else(|| AoCError::BadInputFormat(
                    "The replacement rules could not be parsed".to_string()))?;
            if parse_elem(&mut chars).is_some() {
                return Err(AoCError::BadInputFormat(format!(
                    "Replacement line malformed. Expected '<pattern> => <replacement>.\n\
                    Found: '{}'", line
                )))
            }
            let index = elems.add(tmp);
            elems.set_intermediate(index);

            let mut chars = words[1].chars().peekable();
            while let Some(tmp) = parse_elem(&mut chars) {
                elems.add(tmp);
            }
        }
        Ok(elems)
    }

    fn add(&mut self, name: String) -> usize {
        if let Some(index) = self.index.get(&name) {
            *index
        } else {
            let index = self.elements.len();
            self.elements.push(name.clone());
            self.terminal.push(true);
            self.index.insert(name, index);
            index
        }
    }

    fn set_intermediate(&mut self, index: usize) -> bool {
        if index >= self.elements.len() {
            false
        } else {
            self.terminal[index] = false;
            if self.template_intermediate.is_none() {
                self.template_intermediate = Some(index);
            }
            true
        }
    }

    fn get_index(&self, name: &str) -> Option<usize> {
        let tmp = self.index.get(name).copied();
        if tmp.is_some() && !self.terminal[tmp.unwrap()] {
            assert!(self.template_intermediate.is_some());
            return self.template_intermediate
        }
        tmp
    }

    fn is_terminal(&self, index: usize) -> bool {
        self.terminal[index]
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Molecule {
    counts: Vec<i32>,
}

impl Molecule {
    fn from_rule(str: &str, elems: &Elements) -> Result<Self, AoCError<String>> {
        let words: Vec<&str> = str.split(" => ").collect();
        if words.len() != 2 {
            return Err(AoCError::BadInputFormat(format!(
                "Replacement line malformed. Expected '<pattern> => <replacement>.\nFound: '{}'",
                str
            )))
        }
        let negative = Self::from_string(words[0], elems)?;
        let positive = Self::from_string(words[1], elems)?;
        Ok(positive - negative)
    }

    fn from_string(str: &str, elems: &Elements) -> Result<Self, AoCError<String>> {
        let mut counts = vec![0; elems.elements.len()];
        let mut chars = str.chars().peekable();
        while let Some(e) = Self::parse_elem(&mut chars, elems) {
            counts[e?] += 1;
        }
        Ok(Self{counts})
    }

    fn parse_elem(chars: &mut Peekable<Chars>, elems: &Elements) -> Option<Result<usize, AoCError<String>>> {
        let res = parse_elem(chars)?;
        Some(elems.get_index(&res).ok_or_else(|| AoCError::BadInputFormat(
            "The replacement rules could not be parsed".to_string()))
        )
    }

    fn is_valid(&self, goal: &Self) -> bool {
        for (s, g) in self.counts.iter().zip(goal.counts.iter()) {
            if s > g {
                return false
            }
        }
        true
    }

    fn count_intermediates(&self, elems: &Elements) -> usize {
        let mut count = 0;
        for (index, val) in self.counts.iter().enumerate() {
            if !elems.is_terminal(index) {
                count += val;
            }
        }
        assert!(count >= 0);
        count as usize
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for val in self.counts.iter() {
            count += val;
        }
        assert!(count >= 0);
        count as usize
    }
}

fn parse_elem<I>(chars: &mut Peekable<I>) -> Option<String>
    where I: Iterator<Item = char> {
    chars.peek()?;
    let mut res = format!("{}", chars.next().unwrap());
    if chars.peek().is_some() && chars.peek().unwrap().is_lowercase() {
        res = format!("{}{}", res, chars.next().unwrap());
    }
    Some(res)
}

impl Add for &Molecule {
    type Output = Molecule;

    fn add(self, rhs: Self) -> Self::Output {
        let counts = self.counts.iter()
            .zip(rhs.counts.iter())
            .map(|(x, y)| *x + *y)
            .collect();
        Molecule{counts}
    }
}

impl Sub for Molecule {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let counts = self.counts.iter()
            .zip(rhs.counts.iter())
            .map(|(x, y)| *x - *y)
            .collect();
        Self{counts}
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
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
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 19)?;
        assert_eq!(part_1(&input), Ok("535".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 19)?;
        assert_eq!(part_2(&input), Ok("212".to_string()));
        Ok(())
    }
}
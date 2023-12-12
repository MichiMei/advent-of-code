use std::collections::HashMap;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut records = input.iter()
        .map(|line| Record::parse(line))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(records.iter_mut()
        .map(|record| record.count_possibilities())
        .sum::<usize>()
        .to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut records = input.iter()
        .map(|line| Record::parse(line))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(records.iter_mut()
        .map(|record| {
            record.unfold();
            record.count_possibilities()
        })
        .sum::<usize>()
        .to_string())
}

struct Record {
    springs: Vec<SpringCondition>,
    groups: Vec<usize>,
}

impl Record {
    fn parse(line: &str) -> Result<Self, AoCError<String>> {
        let split = line.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(AoCError::BadInputFormat(
                format!("Unexpected input line. Expected '<Spring conditions> \
                <comma separated list of groups>'. Found '{}'", line)))
        }
        let springs = Self::parse_springs(split[0])?;
        let groups = Self::parse_groups(split[1])?;
        Ok(Self {
            springs,
            groups,
        })
    }

    fn parse_springs(str: &str) -> Result<Vec<SpringCondition>, AoCError<String>> {
        str.chars()
            .map(SpringCondition::parse)
            .collect()
    }

    fn parse_groups(str: &str) -> Result<Vec<usize>, AoCError<String>> {
        str.split(',')
            .map(|num| num.parse()
                .map_err(|e| AoCError::BadInputFormat(
                    format!("Parsing group number '{}' failed. {}", num, e))))
            .collect()
    }

    fn count_possibilities(&mut self) -> usize {
        let missing = self.count_missing_disrepair();
        //self.rec_count_possibilities(0, missing)
        let mut cache = HashMap::new();
        self.rec_count_possibilities(0, missing, 0, None, &mut cache)
    }

    fn rec_count_possibilities
    (
        &mut self,
        start: usize,
        missing: usize,
        mut current_group: usize,
        mut group_remaining: Option<usize>,
        cache: &mut HashMap<(usize, usize, Option<usize>), usize>
    )
        -> usize
    {
        // Recursion end when all springs checked
        if start == self.springs.len() {
            return if missing == 0 {
                assert!(group_remaining.is_none() || group_remaining == Some(0));
                assert!(current_group == self.groups.len() || current_group == self.groups.len() - 1);
                1
            } else {
                0
            }
        }

        // Store current spring condition as backup
        let backup = self.springs[start];

        // If all springs in disrepair found, unknown springs need to be operational
        if missing == 0 && self.springs[start] == SpringCondition::Unknown {
            self.springs[start] = SpringCondition::Operational;
        }

        let res = match self.springs[start] {
            SpringCondition::Unknown => {
                self.springs[start] = SpringCondition::Operational;
                let operational = self.rec_count_possibilities(start, missing, current_group, group_remaining, cache);
                self.springs[start] = SpringCondition::Disrepair;
                let disrepair = self.rec_count_possibilities(start, missing-1, current_group, group_remaining, cache);
                operational+disrepair
            }
            sc => {
                match self.get_state(start, current_group, group_remaining) {
                    State::Opening => {
                        assert_eq!(group_remaining, None);
                        assert_eq!(sc, SpringCondition::Disrepair);
                        assert!(current_group < self.groups.len());
                        group_remaining = Some(self.groups[current_group]-1);
                    }
                    State::Closing => {
                        assert_eq!(group_remaining, Some(0));
                        assert_eq!(sc, SpringCondition::Operational);
                        group_remaining = None;
                        current_group += 1;
                    }
                    State::StillOpen => {
                        assert!(group_remaining.is_some());
                        assert!(group_remaining.unwrap() > 0);
                        assert_eq!(sc, SpringCondition::Disrepair);
                        group_remaining = Some(group_remaining.unwrap()-1);
                    }
                    State::StillClosed => {
                        assert_eq!(group_remaining, None);
                        assert_eq!(sc, SpringCondition::Operational);
                    }
                    State::Bad => {
                        cache.insert((start, current_group, group_remaining), 0);
                        return 0
                    },
                }
                // Check cache (end on hit)
                if let Some(result) = cache.get(&(start, current_group, group_remaining)) {
                    return *result
                }

                self.rec_count_possibilities(start+1, missing, current_group, group_remaining, cache)
            }
        };
        cache.insert((start, current_group, group_remaining), res);
        self.springs[start] = backup;
        res
    }

    fn get_state
    (
        &self,
        start: usize,
        current_group: usize,
        group_remaining: Option<usize>,
    )
        -> State
    {
        match (self.springs[start], group_remaining) {
            (SpringCondition::Operational, None) => State::StillClosed,
            (SpringCondition::Operational, Some(0)) => State::Closing,
            (SpringCondition::Disrepair, None) => {
                if current_group == self.groups.len() {
                    State::Bad
                } else {
                    State::Opening
                }
            }
            (SpringCondition::Disrepair, Some(x)) if x > 0 => State::StillOpen,
            _ => State::Bad,
        }
    }

    fn count_missing_disrepair(&self) -> usize {
        let disrepair = self.groups.iter().sum::<usize>();
        let present = self.springs.iter()
            .filter(|sc| **sc == SpringCondition::Disrepair)
            .count();
        disrepair-present
    }

    fn unfold(&mut self) {
        let pattern = self.springs.clone();
        for _ in 0..4 {
            self.springs.push(SpringCondition::Unknown);
            self.springs.append(&mut pattern.clone());
        }

        let pattern = self.groups.clone();
        for _ in 0..4 {
            self.groups.append(&mut pattern.clone());
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Opening,
    Closing,
    StillOpen,
    StillClosed,
    Bad,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum SpringCondition {
    Operational,
    Disrepair,
    Unknown,
}

impl SpringCondition {
    fn parse(c: char) -> Result<Self, AoCError<String>> {
        match c {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Disrepair),
            '?' => Ok(Self::Unknown),
            c => Err(AoCError::BadInputFormat(
                format!("Unexpected spring condition, expected '.', '#' or '?'. Found '{}'", c)))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "???.### 1,1,3".to_string(),
            ".??..??...?##. 1,1,3".to_string(),
            "?#?#?#?#?#?#?#? 1,3,1,6".to_string(),
            "????.#...#... 4,1,1".to_string(),
            "????.######..#####. 1,6,5".to_string(),
            "?###???????? 3,2,1".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("21".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 12)?;
        assert_eq!(part_1(&input), Ok("7705".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("525152".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 12)?;
        assert_eq!(part_2(&input), Ok("50338344809230".to_string()));
        Ok(())
    }
}
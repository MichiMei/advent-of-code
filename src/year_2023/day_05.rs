use std::cmp::{max, min};
use std::collections::HashMap;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let almanac = Almanac::parse(input, SeedType::SeedList)?;
    let locations = almanac.get_final_destinations()?;
    locations.iter()
        .map(Range::get_start)
        .min()
        .ok_or_else(|| AoCError::NoSolutionFoundError("Calculated solutions empty".to_string()))
        .map(|value| value.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let almanac = Almanac::parse(input, SeedType::SeedRanges)?;
    let locations = almanac.get_final_destinations()?;
    locations.iter()
        .map(Range::get_start)
        .min()
        .ok_or_else(|| AoCError::NoSolutionFoundError("Calculated solutions empty".to_string()))
        .map(|value| value.to_string())
}

enum SeedType {
    SeedList,
    SeedRanges,
}

struct Almanac {
    seeds: Vec<Range>,
    maps: HashMap<String, Map>,
}

impl Almanac {
    fn parse(mut input: &[String], seed_type: SeedType) -> Result<Self, AoCError<String>> {
        if input.len() < 3 {
            return Err(AoCError::BadInputFormat("Input too short, has to include the seeds line, \
                    a blank line and at least one map.".to_string()));
        }
        let seeds = match seed_type {
            SeedType::SeedList => Self::parse_seed_list(&input[0]),
            SeedType::SeedRanges => Self::parse_seed_ranges(&input[0]),
        }?;
        if !input[1].trim().is_empty() {
            return Err(AoCError::BadInputFormat("Second line should be blank.".to_string()))
        }
        input = &input[2..];

        let maps = input.split(|line| line.trim().is_empty())
            .map(|slice| Map::parse(slice)
                .map(|map| (map.source_name.clone(), map)))
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok(Self {
            seeds,
            maps,
        })
    }

    fn parse_seed_list(line: &str) -> Result<Vec<Range>, AoCError<String>> {
        if !line.starts_with("seeds: ") {
            return Err(AoCError::BadInputFormat(
                format!("Input is expected to starts with 'seeds: <list of numbers>'. \
                            Found: '{}'", line)))
        }
        line[7..].split_whitespace()
            .map(|str| str.parse()
                .map(|start| Range::new(start, 1)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing seeds failed. {}", e)))
    }

    fn parse_seed_ranges(line: &str) -> Result<Vec<Range>, AoCError<String>> {
        if !line.starts_with("seeds: ") {
            return Err(AoCError::BadInputFormat(
                format!("Input is expected to starts with 'seeds: <list of numbers>'. \
                            Found: '{}'", line)))
        }
        let numbers = line[7..].split_whitespace()
            .map(|str| str.parse::<i64>()).collect::<Result<Vec<_>, _>>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing seeds failed. {}", e)))?;
        let chunks = numbers.chunks_exact(2);
        if !chunks.remainder().is_empty() {
            return Err(AoCError::BadInputFormat(
                "Odd seed number not supported in ranged mode".to_string()))
        }
        Ok(chunks.map(|c| Range::new(c[0], c[1]))
            .collect())
    }

    fn get_final_destinations(self) -> Result<Vec<Range>, AoCError<String>> {
        let mut current_type = "seed".to_string();
        let mut current_values = self.seeds;
        while current_type != "location" {
            let map = self.maps.get(&current_type)
                .ok_or_else(|| AoCError::NoSolutionFoundError(
                    format!("Map with source {} not found.", current_type)))?;
            current_type = map.get_destination();
            current_values = map.map(current_values);
        }
        Ok(current_values)
    }
}

struct Map {
    source_name: String,
    destination_name: String,
    mappings: Vec<Mapping>,
}

impl Map {
    fn parse(input: &[String]) -> Result<Self, AoCError<String>> {
        let name_end = input[0].find(' ')
            .ok_or_else(|| AoCError::BadInputFormat(
                format!("Expected '<src>-to-<dest> map:', found {}", input[0])))?;
        let title = &(&input[0])[..name_end];
        let split = title.split('-').collect::<Vec<_>>();
        if split.len() != 3 {
            return Err(AoCError::BadInputFormat(
                format!("Expected '<src>-to-<dest> map:', found {}", input[0])))
        }
        let source_name = split[0].to_string();
        let destination_name = split[2].to_string();
        let mappings = input[1..].iter().
            map(|line| Mapping::parse(line))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            source_name,
            destination_name,
            mappings,
        })
    }

    fn map(&self, mut ranges: Vec<Range>) -> Vec<Range> {
        let mut mapped_values = vec![];
        for mapping in self.mappings.iter() {
            ranges = ranges.into_iter()
                .flat_map(|range| {
                    let (mapped, remaining) = mapping.map(&range);
                    if let Some(mapped) = mapped {
                        mapped_values.push(mapped);
                    }
                    remaining.into_iter()
                })
                .collect();
        }
        mapped_values.append(&mut ranges);
        mapped_values
    }

    fn get_destination(&self) -> String {
        self.destination_name.clone()
    }
}

struct Mapping {
    range: Range,
    modifier: i64,
}

impl Mapping {
    fn new(range: Range, modifier: i64) -> Self {
        Self {
            range,
            modifier,
        }
    }

    fn parse(line: &str) -> Result<Self, AoCError<String>> {
        let split = line.split_whitespace().collect::<Vec<_>>();
        if split.len() != 3 {
            return Err(AoCError::BadInputFormat(
                format!("MapEntry malformed, expected '<dest> <src> <range>', found '{}'", line)));
        }
        let destination_id = split[0].parse::<i64>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing destination failed. '{}': {}", line, e)))?;
        let source_id = split[1].parse()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing source failed. '{}': {}", line, e)))?;
        let range = split[2].parse()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing range failed. '{}': {}", line, e)))?;
        let range = Range::new(source_id, range);
        Ok(Self::new(range, destination_id - source_id))
    }

    fn map(&self, range: &Range) -> (Option<Range>, Vec<Range>) {
        let (overlap, remainder) = self.range.split_overlap(range);
        (overlap.map(|o| o.add(self.modifier)), remainder)
    }
}

#[derive(Copy, Clone)]
struct Range {
    start: i64,
    range: i64,
}

impl Range {
    fn new(start: i64, range: i64) -> Self {
        Self {
            start,
            range,
        }
    }

    fn from_end_exclusive(start: i64, end: i64) -> Option<Self> {
        if end <= start {
            return None
        }
        Some(Self {
            start,
            range: end - start,
        })
    }

    fn split_overlap(&self, split_range: &Self) -> (Option<Self>, Vec<Self>) {
        if !self.has_overlap(split_range) {
            return (None, vec![*split_range])
        }
        if self.contains(split_range) {
            return (Some(*split_range), vec![])
        }
        let fixed_start = self.start;
        let fixed_end = self.get_end_exclusive();
        let split_start = split_range.start;
        let split_end = split_range.get_end_exclusive();

        let first_cut = max(split_start, fixed_start);
        let second_cut = min(split_end, fixed_end);

        let prefix = Range::from_end_exclusive(split_start, first_cut);
        let center = Range::from_end_exclusive(first_cut, second_cut);
        assert!(center.is_some(), "Center should always be some, otherwise had no overlap");
        let suffix = Range::from_end_exclusive(second_cut, split_end);

        let mut remainder = vec![];
        if let Some(prefix) = prefix {
            remainder.push(prefix);
        }
        if let Some(suffix) = suffix {
            remainder.push(suffix);
        }
        assert!(!remainder.is_empty(), "Remainder should not be empty, otherwise was contained");

        (center, remainder)
    }

    fn has_overlap(&self, other: &Self) -> bool {
        if self.start >= other.get_end_exclusive() {
            return false
        }
        if other.start >= self.get_end_exclusive() {
            return false
        }
        true
    }

    fn contains(&self, other: &Self) -> bool {
        if other.start >= self.start && other.get_end_exclusive() <= self.get_end_exclusive() {
            return true
        }
        false
    }

    fn add(mut self, modifier: i64) -> Self {
        self.start += modifier;
        self
    }

    fn get_start(&self) -> i64 {
        self.start
    }

    fn get_end_exclusive(&self) -> i64 {
        self.start+self.range
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            "".to_string(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            "".to_string(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("35".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 5)?;
        assert_eq!(part_1(&input), Ok("486613012".to_string()));
        Ok(())
    }
    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("46".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 5)?;
        assert_eq!(part_2(&input), Ok("56931769".to_string()));
        Ok(())
    }
}
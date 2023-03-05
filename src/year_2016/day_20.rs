use std::cmp::max;
use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let ranges = input.iter()
        .map(|line| IPRange::parse(line))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(find_smallest_allowed(ranges).to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let ranges = input.iter()
        .map(|line| IPRange::parse(line))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(count_allowed(ranges).to_string())
}

fn find_smallest_allowed(mut ranges: Vec<IPRange>) -> u32 {
    if ranges.is_empty() {
        return 0
    }
    ranges.sort_unstable_by(|x0, x1| x0.lower_bound.cmp(&x1.lower_bound));
    let mut iter = ranges.into_iter();
    let mut combined = iter.next().expect("List of ranges is not empty");
    if combined.lower_bound > 0 {
        return 0
    }
    for range in iter {
        if let Some(new) = combined.combine(&range) {
            combined = new;
        } else {
            break
        }
    }
    combined.upper_bound+1

}

fn count_allowed(ranges: Vec<IPRange>) -> u32 {
    if ranges.is_empty() {
        return u32::MAX
    }
    let combined = combine(ranges);
    let mut iter = combined.iter();
    let mut current = iter.next().expect("List of ranges is not empty");
    let mut count = current.lower_bound;
    for next in iter {
        count += next.lower_bound-current.upper_bound-1;
        current = next;
    }
    count += u32::MAX-current.upper_bound;
    count
}

fn combine(mut ranges: Vec<IPRange>) -> Vec<IPRange> {
    ranges.sort_unstable_by(|x0, x1| x0.lower_bound.cmp(&x1.lower_bound));
    let mut res = vec![];
    let mut iter = ranges.into_iter().peekable();
    while iter.peek().is_some() {
        let mut combined = iter.next().expect("Was tested to be some");
        while let Some(next) = iter.peek() {
            if let Some(new) = combined.combine(next) {
                combined = new;
                iter.next().expect("Peek returned result");
            } else {
                break
            }
        }
        res.push(combined);
    }
    res
}

#[derive(Debug)]
struct IPRange {
    lower_bound: u32,
    upper_bound: u32,
}

impl IPRange {
    pub fn new(lower_bound: u32, upper_bound: u32) -> Self {
        assert!(lower_bound <= upper_bound);
        Self{lower_bound, upper_bound}
    }

    pub fn parse(line: &str) -> Result<Self, AoCError<String>> {
        let words = line.split('-').collect::<Vec<_>>();
        if words.len() != 2 {
            return Err(AoCError::BadInputFormat(format!(
                "Parsing range failed, expected '<lower-bound>-<upper-bound>', found '{}'", line)))
        }
        let lower_bound = words[0].parse()
            .map_err(|e| AoCError::BadInputFormat(format!(
                "Parsing lower bound failed, expected a number, found '{}'. {}", words[0], e)))?;
        let upper_bound = words[1].parse()
            .map_err(|e| AoCError::BadInputFormat(format!(
                "Parsing upper bound failed, expected a number, found '{}'. {}", words[1], e)))?;
        Ok(Self::new(lower_bound, upper_bound))
    }

    pub fn combine(&self, other: &Self) -> Option<Self> {
        assert!(self.lower_bound <= other.lower_bound);
        if self.upper_bound.saturating_add(1) < other.lower_bound {
            return None
        }
        let lower_bound = self.lower_bound;
        let upper_bound = max(self.upper_bound, other.upper_bound);
        Some(IPRange{lower_bound, upper_bound})
    }
}

impl Display for IPRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.lower_bound, self.upper_bound)
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "5-8".to_string(),
            "0-2".to_string(),
            "4-7".to_string(),
        ];

        assert_eq!(part_1(&v), Ok("3".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_20.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("23923783".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_20.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("125".to_string()));
        Ok(())
    }
}
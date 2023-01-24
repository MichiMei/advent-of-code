use std::cmp::min;

pub fn part_1(input: &Vec<String>) -> Result<String, &str> {
    let weights = parse_input(input)?;

    let res = check_distributions(&weights, 3).ok_or(ERR_NO_SOLUTION)?;

    Ok(res.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, &str> {
    let weights = parse_input(input)?;

    let res = check_distributions(&weights, 4).ok_or(ERR_NO_SOLUTION)?;

    Ok(res.to_string())
}

fn check_distributions(weights: &Vec<u128>, bucket_count: usize) -> Option<u128> {
    let sum: u128 = weights.iter().sum();
    assert_eq!(sum%(bucket_count as u128), 0);
    let goal = sum/(bucket_count as u128);
    let count = weights.len();
    let distribution = Distribution::new(weights.clone());
    for size in 1..=count/bucket_count {
        if !distribution.size_is_possible(size, goal) {
            continue
        }
        if let Some(quantum) =
            check_distributions_for_size_rec(size, goal, distribution.clone(), 0,
                                             bucket_count) {
            return Some(quantum)
        }
    }
    None
}

fn check_distributions_for_size_rec(size: usize, goal: u128, mut distribution: Distribution,
                                    current: usize, bucket_count: usize) -> Option<u128> {
    if distribution.count == size {
        let res = distribution.quantum;
        if distribution.sum == goal {
            return if distribution.splittable(bucket_count - 1) {
                Some(res)
            } else {
                None
            }
        }
    }
    if distribution.sum >= goal {
        return None
    }
    if current >= distribution.weights.len() {
        return None
    }
    if !distribution.still_possible(current, size, goal) {
        return None
    }

    let mut active = distribution.clone();
    active.add(current);
    let res_active =
        check_distributions_for_size_rec(size, goal, active, current+1,
                                         bucket_count);
    let res_inactive =
        check_distributions_for_size_rec(size, goal, distribution, current+1, bucket_count);

    if res_active.is_none() {
        return res_inactive
    }
    if res_inactive.is_none() {
        return res_active
    }
    Some(min(res_active.unwrap(), res_inactive.unwrap()))
}

#[derive(Clone)]
struct Distribution {
    weights: Vec<u128>,
    active: Vec<bool>,
    count: usize,
    sum: u128,
    quantum: u128,
}

impl Distribution {
    fn new(mut weights: Vec<u128>) -> Self {
        weights.sort();
        weights.reverse();
        let active = vec![false; weights.len()];
        Self {
            weights,
            active,
            count: 0,
            sum: 0,
            quantum: 1,
        }
    }

    fn add(&mut self, index: usize) {
        assert!(index < self.weights.len());
        assert_eq!(self.active[index], false);

        self.active[index] = true;
        self.count += 1;
        self.sum += self.weights[index];
        self.quantum *= self.weights[index];
    }

    fn splittable(&mut self, bucket_count: usize) -> bool {
        if bucket_count == 1 {
            return true
        }
        let goal = self.sum;
        self.sum = 0;
        self.splittable_rec(goal, 0, bucket_count)
    }

    fn splittable_rec(&mut self, goal: u128, index: usize, bucket_count: usize) -> bool {
        if self.sum == goal {
            return self.splittable(bucket_count-1)
        }
        if self.sum > goal || index >= self.weights.len() {
            return false
        }
        if self.active[index] {
            return self.splittable_rec(goal, index+1, bucket_count);
        }

        let mut active = self.clone();
        active.add(index);
        active.splittable_rec(goal, index+1, bucket_count)
            || self.splittable_rec(goal, index+1, bucket_count)
    }

    fn size_is_possible(&self, size: usize, goal: u128) -> bool {
        self.weights.iter().take(size).sum::<u128>() >= goal
    }

    fn still_possible(&self, index: usize, size: usize, goal: u128) -> bool {
        self.sum+self.weights[index]*((size-self.count) as u128) >= goal
    }
}



fn parse_input(input: &Vec<String>) -> Result<Vec<u128>, &str> {
    let mut res = vec![];
    for line in input.iter() {
        res.push(line.parse().map_err(|_| ERR_INPUT_MALFORMED)?)
    }
    Ok(res)
}

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";
const ERR_NO_SOLUTION: &str = "No solution was found";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "11".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let v = get_example();

        assert_eq!(part_1(&v), Ok("99".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_24.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("11266889531".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = get_example();

        assert_eq!(part_2(&v), Ok("44".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_24.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("77387711".to_string()));
        Ok(())
    }
}
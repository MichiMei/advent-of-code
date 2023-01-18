use std::collections::HashSet;

pub fn part_1(input: &Vec<String>) -> Result<String, &str> {
    let mut weights = parse_input(input)?;

    unimplemented!()
}

pub fn part_2(input: &Vec<String>) -> Result<String, &str> {
    unimplemented!()
}

fn calc_optimal_distribution(weights: &Vec<u128>) -> Option<u128> {
    let mut sorted = weights.clone();
    sorted.sort_unstable();
    sorted.reverse();
    let min = min_set_size(weights);
    let max = weights.iter().count()/3;
    for size in min..=max {
        if let Some(res) = calc_optimal_distribution_for_size(weights, size) {
            return Some(res)
        }
    }
    None
}

fn calc_optimal_distribution_for_size(weights: &Vec<u128>, size: usize) -> Option<u128> {
    let weight_sum = sum_weights(weights);
    assert_eq!(weight_sum % 3, 0);
    let goal = weight_sum/3;

    let mut solutions = find_first_set_rec();
    solutions.sort_by(|x1, x2| {
        let y1 = calculate_quantum_entanglement(x1, weights);
        let y2 = calculate_quantum_entanglement(x2, weights);
        y1.cmp(&y2)
    });
    solutions.sort_by(|x1, x2| {
        let y1 = calculate_count(x1);
        let y2 = calculate_count(x2);
        y1.cmp(&y2)
    });

    for solution in solutions {
        if is_valid(&solution, &weights) {
            return Some(calculate_quantum_entanglement(&solution, &weights))
        }
    }

    None
}

fn find_first_set_rec(weights: &Vec<u128>, solution: &Vec<bool>, index: usize, remaining: usize, sum: u128, goal: u128) -> Vec<Vec<bool>> {
    let mut res = vec![];
    if remaining == 0 || sum >= goal || index >= solution.len() {
        if sum == goal {
            res.push(solution.clone());
        }
        return res
    }

    let mut new_solution = solution.clone();
    new_solution[index] = true;
    let new_sum = sum + weights[index];
    res.extend(find_first_set_rec(weights, &new_solution, index+1, remaining-1, new_sum, goal));
    res.extend(find_first_set_rec(weights, solution, index+1, remaining, sum, goal));

    res
}

fn parse_input(input: &Vec<String>) -> Result<Vec<u128>, &str> {
    let mut res = vec![];
    for line in input.iter() {
        res.push(line.parse().map_err(|_| ERR_INPUT_MALFORMED)?)
    }
    Ok(res)
}

fn sum_weights(weights: &Vec<u128>) -> u128 {
    weights.iter().sum()
}

fn min_set_size(weights: &Vec<u128>) -> usize {
    let sum = sum_weights(weights);
    assert_eq!(sum % 3, 0);
    let goal = sum/3;
    let mut sum = 0;
    for (index, elem) in weights.iter().enumerate() {
        sum += *elem;
        if sum >= goal {
            return index+1
        }
    }
    panic!();
}

fn calculate_count(used: &Vec<bool>) -> usize {
    used.iter().filter(|y| **y).count()
}

fn calculate_quantum_entanglement(used: &Vec<bool>, weight: &Vec<u128>) -> u128 {
    used.iter().zip(weight.iter())
        .filter(|(b, _)| **b)
        .map(|(_, w)| *w)
        .product()
}

fn is_valid(weights: &Vec<u128>, used: &Vec<bool>) -> bool {



    unimplemented!()
}









fn get_all_possibilities(weights: &Vec<u128>) -> Vec<Vec<bool>> {
    let sum: u128 = weights.iter().sum();
    assert_eq!(sum % 3, 0);
    let goal = sum/3;
    get_all_possibilities_rec(weights, 0, &vec![false; weights.len()], goal, 0)
}

fn get_all_possibilities_rec(weights: &Vec<u128>, index: usize, used: &Vec<bool>, goal: u128, sum: u128) -> Vec<Vec<bool>> {
    if sum == goal {
        return vec![used.clone()]
    }
    if sum > goal || index >= used.len() {
        return vec![]
    }

    let mut res = vec![];

    res.extend(get_all_possibilities_rec(weights, index+1, used, goal, sum));

    let mut new_used = used.clone();
    new_used[index] = true;
    let new_weight = sum+weights[index];
    res.extend(
        get_all_possibilities_rec(weights, index+1, &new_used, goal, new_weight)
    );

    res
}

fn combine_possibilities(possibilities: Vec<Vec<bool>>, weights: &Vec<u128>) -> HashSet<Vec<Location>> {
    let mut res = HashSet::new();
    for i0 in 0..possibilities.len() {
        let p0 = &possibilities[i0];
        for i1 in i0+1..possibilities.len() {
            let p1 = &possibilities[i1];
            if check_compatible(p0, p1) {
                res.insert(get_combination(p0, p1, weights));
            }
        }
    }
    res
}

fn check_compatible(p0: &Vec<bool>, p1: &Vec<bool>) -> bool {
    if p0.len() != p1.len() {
        return false
    }
    for (elem0, elem1) in p0.iter().zip(p1.iter()) {
        if *elem0 && *elem1 {
            return false
        }
    }
    true
}

fn get_combination(p0: &Vec<bool>, p1: &Vec<bool>, weights: &Vec<u128>) -> Vec<Location> {
    let order = get_order(p0, p1, weights);
    let mut res = vec![order[2]; p0.len()];
    for (index, (elem0, elem1)) in p0.iter().zip(p1.iter()).enumerate() {
        assert!( !(*elem0 && *elem1) );
        if *elem0 {
            res[index] = order[0];
        }
        if *elem1 {
            res[index] = order[1];
        }
    }
    res
}

fn get_order(p0: &Vec<bool>, p1: &Vec<bool>, weights: &Vec<u128>) -> Vec<Location> {
    let mut p2 = vec![false; p0.len()];
    for (index, (elem0, elem1)) in p0.iter().zip(p1.iter()).enumerate() {
        if !elem0 && !elem1 {
            p2[index] = true;
        }
    }
    let used = [p0, p1, &p2];

    let mut order = vec![0, 1, 2];

    // sort by quantum entanglement
    let quantum_entanglements: Vec<u128> = used.iter()
        .map(|x| calculate_quantum_entanglement(*x, weights)).collect();
    assert_eq!(quantum_entanglements.len(), 3);
    order.sort_by(|x0, x1| {
        quantum_entanglements[*x0].cmp(&quantum_entanglements[*x1])
    });

    // sort by item counts
    let counts: Vec<usize> =
        used.iter().map(|x| calculate_count(x)).collect();
    assert_eq!(counts.len(), 3);
    order.sort_by(|x0, x1| {
        counts[*x0].cmp(&counts[*x1])
    });

    let mut res = vec![Location::Left; 3];
    for (index, elem) in order.iter().enumerate() {
        res[*elem] = Location::from(index);
    }

    res
}

fn count(locs: &Vec<Location>) -> usize {
    locs.iter().filter(|x| **x == Location::Passenger).count()
}

fn quantum_entanglement(locs: &Vec<Location>, weights: &Vec<u128>) -> u128 {
    locs.iter().zip(weights.iter())
        .filter(|(l, _)| **l == Location::Passenger)
        .map(|(_, w)| *w)
        .product()
}

fn print(split: &Vec<Location>, weights: &Vec<u128>) {
    for (loc, weight) in split.iter().zip(weights.iter()) {
        if *loc == Location::Passenger {
            print!("{} ", weight);
        }
    }
    print!("| ");
    for (loc, weight) in split.iter().zip(weights.iter()) {
        if *loc == Location::Left {
            print!("{} ", weight);
        }
    }
    print!("| ");
    for (loc, weight) in split.iter().zip(weights.iter()) {
        if *loc == Location::Right {
            print!("{} ", weight);
        }
    }
    println!()
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Location {
    Passenger,
    Left,
    Right,
}

impl Location {
    fn from(index: usize) -> Self {
        match index {
            0 => Self::Passenger,
            1 => Self::Left,
            2 => Self::Right,
            _ => panic!(),
        }
    }
}

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
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
        ];

        assert_eq!(part_1(&v), Ok("99".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_24.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("expected".to_string())); // TODO
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["input".to_string()]), Ok("expected".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_XX.txt";    // TODO
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("expected".to_string())); // TODO
        Ok(())
    }
}
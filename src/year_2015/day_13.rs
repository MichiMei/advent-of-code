use std::cmp::max;
use std::collections::{HashMap, HashSet};

pub fn part_1(input: &[String]) -> Result<String, &str> {
    let matrix = parse_happiness_matrix(input)?;
    let table = Table::new(matrix.len());
    let res = find_optimal_sitting(&matrix, &table, false);
    Ok(res.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, &str> {
    let matrix = parse_happiness_matrix(input)?;
    let table = Table::new(matrix.len());
    let res = find_optimal_sitting(&matrix, &table, true);
    Ok(res.to_string())
}

fn parse_happiness_matrix(input: &[String]) -> Result<Vec<Vec<i32>>, &str> {
    let mut index = HashMap::new();
    for line in input.iter() {
        let name = line.split(' ').next().ok_or(ERR_INPUT_MALFORMED)?;
        if !index.contains_key(name) {
            let i = index.len();
            index.insert(name, i);
        }
    }

    let mut matrix = vec![vec![0; index.len()]; index.len()];
    for line in input {
        let words: Vec<&str> = line.split(' ').collect();
        if words.len() != 11 {
            return Err(ERR_INPUT_MALFORMED)
        }
        let src = &words[0];
        let dest = &words[10];
        let dest = &dest[0..dest.len()-1];
        let negative = match words[2] {
            "gain" => false,
            "lose" => true,
            _ => return Err(ERR_INPUT_MALFORMED),
        };
        let mut val = words[3].parse::<i32>().map_err(|_| ERR_INPUT_MALFORMED)?;
        if negative {
            val = -val;
        }

        let x = index.get(src).ok_or(ERR_INPUT_MALFORMED)?;
        let y = index.get(dest).ok_or(ERR_INPUT_MALFORMED)?;

        matrix[*x][*y] = val;
    }

    Ok(matrix)
}

fn find_optimal_sitting(matrix: &Vec<Vec<i32>>, table: &Table, seat_self: bool) -> i32 {
    if table.remaining.is_empty() {
        let first = table.order.first().unwrap();
        let last = table.order.last().unwrap();
        let mut res = table.happiness;
        if !seat_self {
            res += matrix[*first][*last] + matrix[*last][*first];
        }
        return res
    }

    let mut maximum = 0;

    for rem in table.remaining.iter() {
        let mut new_table = table.clone();
        new_table.remaining.remove(rem);
        if !table.order.is_empty() {
            new_table.happiness += matrix[*table.order.last().unwrap()][*rem];
            new_table.happiness += matrix[*rem][*table.order.last().unwrap()];
        }
        new_table.order.push(*rem);
        let tmp = find_optimal_sitting(matrix, &new_table, seat_self);
        maximum = max(maximum, tmp);
    }

    maximum
}

#[derive(Clone)]
struct Table {
    remaining: HashSet<usize>,
    order: Vec<usize>,
    happiness: i32,
}

impl Table {
    fn new(count: usize) -> Self {
        let remaining = (0..count).collect();
        let mut order = vec![];
        order.reserve(count);
        Self{remaining, order, happiness: 0}
    }
}

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/example_day_13.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("330".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_13.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("618".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_13.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("601".to_string()));
        Ok(())
    }
}
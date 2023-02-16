use std::cmp::Ordering;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let containers = parse_containers(input)?;
    let mut filled = vec![false; containers.len()];
    let count = calc_possibilities(&containers, &mut filled, 0, 0, 150);
    Ok(count.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let containers = parse_containers(input)?;
    let mut filled = vec![false; containers.len()];
    let (count, _) =
        calc_minimal_possibilities(&containers, &mut filled, 0, 0, 150)
            .ok_or_else(|| AoCError::NoSolutionFoundError(
                "No possible distribution found".to_string()
            ))?;
    Ok(count.to_string())
}

fn parse_containers(input: &[String]) -> Result<Vec<u16>, AoCError<String>> {
    let mut res = vec![];
    for line in input {
        res.push(line.parse().map_err(|e| AoCError::BadInputFormat(
            format!("Parsing number failed, found '{}'.\n{}", line, e)
        ))?);
    }
    Ok(res)
}

fn calc_possibilities(containers: &Vec<u16>, filled: &mut Vec<bool>, current: usize, amount: usize,
                      total: usize) -> usize {
    if current >= containers.len() {
        return usize::from(amount == total)
    }

    let try0 = calc_possibilities(containers, filled, current+1, amount, total);

    filled[current] = true;
    let new_amount = amount + containers[current] as usize;
    let try1 = calc_possibilities(containers, filled, current+1,
                                  new_amount, total);
    filled[current] = false;

    try0+try1
}

fn calc_minimal_possibilities(containers: &Vec<u16>, filled: &mut Vec<bool>, current: usize, amount: usize,
                       total: usize) -> Option<(usize, usize)> {
    if current >= containers.len() {
        return if amount == total {
            Some((1, filled.iter().filter(|x| **x).count()))
        } else {
            None
        }
    }

    let try0 = calc_minimal_possibilities(containers, filled, current+1, amount, total);

    filled[current] = true;
    let new_amount = amount + containers[current] as usize;
    let try1 = calc_minimal_possibilities(containers, filled, current+1,
                                  new_amount, total);
    filled[current] = false;

    if try0.is_none() {
        return try1
    }
    if try1.is_none() {
        return try0
    }

    let try0 = try0.unwrap();
    let try1 = try1.unwrap();

    match try0.1.cmp(&try1.1) {
        Ordering::Less => Some(try0),
        Ordering::Equal => Some((try0.0+try1.0, try0.1)),
        Ordering::Greater => Some(try1),
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() -> Result<(), AoCError<String>> {
        let v = vec![
            "20".to_string(),
            "15".to_string(),
            "10".to_string(),
            "5".to_string(),
            "5".to_string()
        ];
        let containers = parse_containers(&v)?;
        let mut filled = vec![false; containers.len()];
        let count = calc_possibilities(&containers, &mut filled, 0, 0, 25);
        assert_eq!(count, 4);
        Ok(())
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_17.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("1304".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() -> Result<(), AoCError<String>> {
        let v = vec![
            "20".to_string(),
            "15".to_string(),
            "10".to_string(),
            "5".to_string(),
            "5".to_string()
        ];
        let containers = parse_containers(&v)?;
        let mut filled = vec![false; containers.len()];
        let (count, _) =
            calc_minimal_possibilities(&containers, &mut filled, 0, 0, 25)
                .ok_or_else(|| AoCError::NoSolutionFoundError(
                    "No possible distribution found".to_string()
                ))?;
        assert_eq!(count, 3);
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_17.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("18".to_string()));
        Ok(())
    }
}
use std::collections::{HashMap, HashSet};

pub fn part_1(input: &Vec<String>) -> Result<String, &str> {
    let distances = parse_input(input)?;
    let remaining: HashSet<usize> = (0..distances.len()).collect();

    let res = find_shortest_path_rec(&distances, &remaining, 0, None, None);

    if res.is_none() {
        Err(ERR_NO_PATH_FOUND)
    } else {
        Ok(res.unwrap().to_string())
    }
}

pub fn part_2(input: &Vec<String>) -> Result<String, &str> {
    let distances = parse_input(input)?;
    let remaining: HashSet<usize> = (0..distances.len()).collect();

    let res = find_longest_path_rec(&distances, &remaining, 0, None);

    if res.is_none() {
        Err(ERR_NO_PATH_FOUND)
    } else {
        Ok(res.unwrap().to_string())
    }
}

fn parse_input(input: &Vec<String>) -> Result<Vec<Vec<u16>>, &str> {
    let mut tmp = vec![];
    for line in input {
        tmp.push(parse_line(line)?);
    }

    let mut index = HashMap::new();
    let mut count = 0;
    for (s, d, _) in tmp.iter() {
        if !index.contains_key(s) {
            assert!(index.insert(s, count).is_none());
            count += 1;
        }
        if !index.contains_key(d) {
            assert!(index.insert(d, count).is_none());
            count += 1;
        }
    }

    let mut res = vec![vec![0; index.len()]; index.len()];
    for (s, d, val) in tmp.iter() {
        let x = index.get(&s).unwrap();
        let y = index.get(&d).unwrap();
        res[*x][*y] = *val;
        res[*y][*x] = *val;
    }

    Ok(res)
}

fn parse_line(str: &str) -> Result<(String, String, u16), &str> {
    let words: Vec<&str> = str.split(" ").collect();
    if words.len() != 5 {
        return Err(ERR_INPUT_MALFORMED)
    }
    let s = words[0].to_string();
    let d = words[2].to_string();
    let v = words[4].parse().map_err(|_| ERR_INPUT_MALFORMED)?;

    Ok((s, d, v))
}

fn find_shortest_path_rec(distances: &Vec<Vec<u16>>, remaining: &HashSet<usize>, length: u16,
                          current: Option<usize>, shortest: Option<u16>) -> Option<u16> {
    if shortest.is_some() && length >= shortest.unwrap() {
        return None
    }
    if remaining.is_empty() {
        return if shortest.is_none() || shortest.unwrap() > length {
            Some(length)
        } else {
            None
        }
    }

    let mut new_shortest = shortest;

    let mut remaining_new = remaining.clone();
    for elem in remaining.iter() {
        assert!(remaining_new.remove(elem));
        let length_new = if current.is_some() {
            length + distances[current.unwrap()][*elem]
        } else {
            length
        };
        let res = find_shortest_path_rec(distances, &remaining_new, length_new,
                                         Some(*elem), new_shortest);
        if res.is_some() && (new_shortest.is_none() || new_shortest.unwrap() > res.unwrap()) {
            new_shortest = res;
        }
        assert!(remaining_new.insert(*elem));
    }

    if new_shortest == shortest {
        None
    } else {
        new_shortest
    }
}

fn find_longest_path_rec(distances: &Vec<Vec<u16>>, remaining: &HashSet<usize>, length: u16,
                         current: Option<usize>) -> Option<u16> {
    if remaining.is_empty() {
        return Some(length)
    }

    let mut longest = None;

    let mut remaining_new = remaining.clone();
    for elem in remaining.iter() {
        assert!(remaining_new.remove(elem));
        let length_new = if current.is_some() {
            length + distances[current.unwrap()][*elem]
        } else {
            length
        };
        let res = find_longest_path_rec(distances, &remaining_new, length_new,
                                         Some(*elem));
        if res.is_some() && (longest.is_none() || longest.unwrap() < res.unwrap()) {
            longest = res;
        }
        assert!(remaining_new.insert(*elem));
    }

    longest
}

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";
const ERR_NO_PATH_FOUND: &str = "Could not calculate a path";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let input = vec![
            "London to Dublin = 464".to_string(),
            "London to Belfast = 518".to_string(),
            "Dublin to Belfast = 141".to_string()
        ];
        assert_eq!(part_1(&input), Ok("605".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_09.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("207".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = vec![
            "London to Dublin = 464".to_string(),
            "London to Belfast = 518".to_string(),
            "Dublin to Belfast = 141".to_string()
        ];
        assert_eq!(part_2(&input), Ok("982".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_09.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("804".to_string()));
        Ok(())
    }
}
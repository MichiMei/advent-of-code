use std::collections::HashMap;
use std::mem::swap;
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let map = Map::parse(input)?;
    let steps = map.follow_from_to("AAA", "ZZZ")?;
    Ok(steps.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let map = Map::parse(input)?;
    let starts = map.get_a_nodes();
    match map.calculate_z_reach(&starts) {
        Ok(value) => Ok(value.to_string()),
        Err(e) => {
            println!("Fallback to brute force: {}", e);
            map.follow_to_z_nodes(starts).map(|v| v.to_string())
        }
    }

}

struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Map {
    fn parse(input: &Vec<String>) -> Result<Self, AoCError<String>> {
        if input.len() < 3 {
            return Err(AoCError::UnexpectedInputLength(
                "Input needs to be at least 3 lines: directions, empty, node".to_string()))
        }
        let directions = input[0].chars()
            .map(Direction::parse)
            .collect::<Result<_, _>>()?;
        let nodes = input[2..].iter()
            .map(|line| Self::parse_node(line))
            .collect::<Result<_, _>>()?;
        Ok(Self {
            directions,
            nodes,
        })
    }

    fn parse_node(line: &str) -> Result<(String, (String, String)), AoCError<String>> {
        let split = line.split_whitespace().collect::<Vec<_>>();
        if split.len() != 4 {
            return Err(AoCError::BadInputFormat(
                format!("Bad node format, expected: '<start> = (<left>, <right>)'. \
                Found '{}'", line)))
        }
        if split[1] != "=" {
            return Err(AoCError::BadInputFormat(
                format!("Bad node format, expected: '<start> = (<left>, <right>)'. \
                Found '{}'", line)))
        }
        let start = split[0].to_string();
        if !split[2].starts_with('(') || !split[2].ends_with(',') {
            return Err(AoCError::BadInputFormat(
                format!("Bad node format, expected: '<start> = (<left>, <right>)'. \
                Found '{}'", line)))
        }
        let left = split[2][1..(split[2].len()-1)].to_string();
        if !split[3].ends_with(')') {
            return Err(AoCError::BadInputFormat(
                format!("Bad node format, expected: '<start> = (<left>, <right>)'. \
                Found '{}'", line)))
        }
        let right = split[3][..(split[3].len()-1)].to_string();
        Ok((start, (left, right)))
    }

    fn follow_from_to(&self, start: &str, end: &str) -> Result<usize, AoCError<String>> {
        let mut current = start;
        let mut count = 0;
        for direction in self.directions.iter().cycle() {
            if current == end {
                break
            }
            let node = self.nodes.get(current)
                .ok_or_else(|| AoCError::NoSolutionFoundError(
                    format!("Could not find node '{}'", current)))?;
            current = match direction {
                Direction::Left => &node.0,
                Direction::Right => &node.1,
            };
            count += 1;
        }
        Ok(count)
    }

    fn get_a_nodes(&self) -> Vec<&str> {
        self.nodes.keys()
            .filter(|node| node.ends_with('A'))
            .map(|node| &node[..])
            .collect()
    }

    fn follow_to_z_nodes<'a>(&'a self, mut starts: Vec<&'a str>) -> Result<usize, AoCError<String>> {
        let mut count = 0;
        for direction in self.directions.iter().cycle() {
            assert!(count < 100000000);
            let mut end = true;
            for current in starts.iter_mut() {
                if !current.ends_with('Z') {
                    end = false;
                }
                let node = self.nodes.get(*current)
                    .ok_or_else(|| AoCError::NoSolutionFoundError(
                        format!("Could not find node '{}'", current)))?;
                *current = match direction {
                    Direction::Left => &node.0,
                    Direction::Right => &node.1,
                };
            }
            if end {
                break
            }
            count += 1;
        }
        Ok(count)
    }

    /// Calculates the 'first' time all starting values reach an end point.
    /// For this the functions calculates the finishing loops and calculates a LCM, therefore this
    /// solution only works, iff all paths only reach a unique endpoint before and during their
    /// loops.
    fn calculate_z_reach(&self, starts: &[&str]) -> Result<usize, AoCError<String>> {
        let loops = starts.iter()
            .map(|start| self.get_finish_loop(start))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(multi_lcm(loops))
    }

    fn get_finish_loop(&self, start: &str) -> Result<(usize, usize), AoCError<String>> {
        let mut current = start;
        let mut first_finish = None;
        for (steps, (index, direction)) in self.directions.iter().enumerate().cycle().enumerate() {
            if current.ends_with('Z') {
                if let Some((last_steps, last_index)) = first_finish {
                    return if last_index == index {
                        Ok((steps, steps - last_steps))
                    } else {
                        Err(AoCError::NoSolutionFoundError(
                            "Solution expects each path to only reach a unique finishing loop."
                                .to_string()))
                    }
                } else {
                    first_finish = Some((steps, index));
                }
            }
            let node = self.nodes.get(current)
                .ok_or_else(|| AoCError::NoSolutionFoundError(
                    format!("Could not find node '{}'", current)))?;
            current = match direction {
                Direction::Left => &node.0,
                Direction::Right => &node.1,
            };
        }
        panic!("Unreachable")
    }
}

fn multi_lcm(mut loops: Vec<(usize, usize)>) -> usize {
    if loops.is_empty() {
        return 0
    }
    if loops.len() == 1 {
        return loops[0].0;
    }
    let mut main = loops.pop().expect("Loops is not empty");
    while let Some(other) = loops.pop() {
        main = single_lcm(main, other);
    }
    main.0
}

fn single_lcm(mut first: (usize, usize), mut second: (usize, usize)) -> (usize, usize) {
    if first.0 < second.0 {
        swap(&mut first, &mut second)
    }
    assert!(first.0 >= second.0);
    let mut count = 0;
    let mut tmp = first.0 - second.0;
    while tmp%second.1 != 0 {
        tmp += first.1;
        count += 1;
    }
    let start = first.0 + first.1*count;
    let loop_size = lcm(first.1, second.1);
    (start, loop_size)
}

fn lcm(f: usize, s: usize) -> usize {
    f/gcd(f, s)*s
}

fn gcd(f: usize, s: usize) -> usize {
    let mut a = f;
    let mut b = s;
    while b != 0 {
        let h = a%b;
        a = b;
        b = h;
    }
    a
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(c: char) -> Result<Self, AoCError<String>> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            c => Err(AoCError::BadInputFormat(
                format!("Directions string contains unsupported char '{}'. \
                Only 'L' and 'R' supported.", c)))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_1_input() -> Vec<String> {
        vec![
            "RL".to_string(),
            "".to_string(),
            "AAA = (BBB, CCC)".to_string(),
            "BBB = (DDD, EEE)".to_string(),
            "CCC = (ZZZ, GGG)".to_string(),
            "DDD = (DDD, DDD)".to_string(),
            "EEE = (EEE, EEE)".to_string(),
            "GGG = (GGG, GGG)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ]
    }

    fn get_example_2_input() -> Vec<String> {
        vec![
            "LLR".to_string(),
            "".to_string(),
            "AAA = (BBB, BBB)".to_string(),
            "BBB = (AAA, ZZZ)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_1_input();
        assert_eq!(part_1(&input), Ok("2".to_string()));
        let input = get_example_2_input();
        assert_eq!(part_1(&input), Ok("6".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 8)?;
        assert_eq!(part_1(&input), Ok("21883".to_string()));
        Ok(())
    }

    fn get_example_3_input() -> Vec<String> {
        vec![
            "LR".to_string(),
            "".to_string(),
            "11A = (11B, XXX)".to_string(),
            "11B = (XXX, 11Z)".to_string(),
            "11Z = (11B, XXX)".to_string(),
            "22A = (22B, XXX)".to_string(),
            "22B = (22C, 22C)".to_string(),
            "22C = (22Z, 22Z)".to_string(),
            "22Z = (22B, 22B)".to_string(),
            "XXX = (XXX, XXX)".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_3_input();
        assert_eq!(part_2(&input), Ok("6".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 8)?;
        assert_eq!(part_2(&input), Ok("12833235391111".to_string()));
        Ok(())
    }
}
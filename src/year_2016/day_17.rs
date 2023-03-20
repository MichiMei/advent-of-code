use std::collections::VecDeque;
use crate::errors::AoCError;
use crate::md5_collision::hash;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the passcode".to_string()))
    }
    let res = find_shortest_path((0,0), (3,3), &input[0])?;
    Ok(res)
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the passcode".to_string()))
    }
    let res = find_longest_path((0,0), (3,3), &input[0])?;
    Ok(res.to_string())
}

type Point = (u8, u8);

fn find_shortest_path(start: Point, end: Point, passcode: &str)
    -> Result<String, AoCError<String>> {
    let mut queue = VecDeque::new();
    queue.push_back((start, "".to_string()));
    while !queue.is_empty() {
        let (current, path) = queue.pop_front().expect("Queue checked by while");
        if current == end {
            return Ok(path)
        }
        for (neighbor, path) in get_neighbors(current, passcode, &path) {
            queue.push_back((neighbor, path));
        }
    }
    Err(AoCError::NoSolutionFoundError(
        format!("No possible path from {:?} to {:?} was found.", start, end)))
}

fn find_longest_path(start: Point, end: Point, passcode: &str)
    -> Result<usize, AoCError<String>> {
    let mut queue = VecDeque::new();
    queue.push_back((start, "".to_string()));
    let mut length = None;
    while !queue.is_empty() {
        let (current, path) = queue.pop_front().expect("Queue checked by while");
        if current == end {
            if length.is_none() || length.unwrap() < path.len() {
                length = Some(path.len());
            }
            continue
        }
        for (neighbor, path) in get_neighbors(current, passcode, &path) {
            queue.push_back((neighbor, path));
        }
    }
    length.ok_or_else(|| AoCError::NoSolutionFoundError(
        format!("No possible path from {:?} to {:?} was found.", start, end)))
}

fn get_neighbors(p: Point, passcode: &str, path: &str) -> Vec<(Point, String)> {
    let hash = hash(&format!("{}{}", passcode, path))
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    let mut res = vec![];
    if p.0 > 0 && is_door_open(&hash, Direction::Left) {
        res.push(((p.0-1, p.1), format!("{}{}", path, Direction::Left.get_char())));
    }
    if p.1 > 0 && is_door_open(&hash, Direction::Up) {
        res.push(((p.0, p.1-1), format!("{}{}", path, Direction::Up.get_char())));
    }
    if p.0 < 3 && is_door_open(&hash, Direction::Right) {
        res.push(((p.0+1, p.1), format!("{}{}", path, Direction::Right.get_char())));
    }
    if p.1 < 3 && is_door_open(&hash, Direction::Down) {
        res.push(((p.0, p.1+1), format!("{}{}", path, Direction::Down.get_char())));
    }
    res
}

fn is_door_open(hash: &str, dir: Direction) -> bool {
    assert!(hash.is_ascii());
    let bytes = hash.as_bytes();
    match bytes[dir.get_index()] as char {
        '0'..='9' | 'a' => false,
        'b'..='f' => true,
        _ => panic!("The string should only contain hex chars"),
    }
}

enum Direction {
    Up, Down, Right, Left
}

impl Direction {
    pub fn get_index(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Right => 3,
            Direction::Left => 2,
        }
    }

    pub fn get_char(&self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Right => 'R',
            Direction::Left => 'L',
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert!(part_1(&vec!["hijkl".to_string()]).is_err());
        assert_eq!(part_1(&vec!["ihgpwlah".to_string()]), Ok("DDRRRD".to_string()));
        assert_eq!(part_1(&vec!["kglvqrro".to_string()]), Ok("DDUDRLRRUDRD".to_string()));
        assert_eq!(part_1(&vec!["ulqzkmiv".to_string()]),
                   Ok("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 17)?;
        assert_eq!(part_1(&input), Ok("DDRRUDLRRD".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["ihgpwlah".to_string()]), Ok("370".to_string()));
        assert_eq!(part_2(&vec!["kglvqrro".to_string()]), Ok("492".to_string()));
        assert_eq!(part_2(&vec!["ulqzkmiv".to_string()]), Ok("830".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 17)?;
        assert_eq!(part_2(&input), Ok("488".to_string()));
        Ok(())
    }
}
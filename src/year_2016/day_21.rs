use std::mem::swap;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let instructions = parse_instructions(input)?;
    let password = scramble(&instructions, "abcdefgh")?;
    Ok(password)
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let instructions = parse_instructions(input)?;
    let password = unscramble(&instructions, "fbgdceah")?;
    Ok(password)
}

fn parse_instructions(input: &[String]) -> Result<Vec<Instruction>, AoCError<String>> {
    input.iter().map(|line| Instruction::parse(line)).collect()
}

fn scramble(instructions: &[Instruction], password: &str) -> Result<String, AoCError<String>> {
    let mut password= password.to_string();
    for instruction in instructions.iter() {
        password = instruction.execute(&password, false)?;
    }
    Ok(password)
}

fn unscramble(instructions: &[Instruction], password: &str) -> Result<String, AoCError<String>> {
    let mut password= password.to_string();
    for instruction in instructions.iter().rev() {
        password = instruction.execute(&password, true)?;
    }
    Ok(password)
}

enum Instruction {
    SwapPosition(usize, usize),
    SwapLetters(char, char),
    RotateSteps(Direction, usize),
    RotateCharBased(char),
    Reverse(usize, usize),
    MoveChar(usize, usize),
}

impl Instruction {
    pub fn parse(line: &str) -> Result<Self, AoCError<String>> {
        if line.starts_with("swap position ") {
            let words = line.split(' ').collect::<Vec<_>>();
            if words.len() != 6 {
                return Err(AoCError::BadInputFormat(format!(
                    "Malformed 'swap position', expected 'swap position <src> with position \
                    <dest>', found '{}'", line)))
            }
            let src = words[2].parse().map_err(|e| AoCError::BadInputFormat(
                format!("Parsing 'swap position' src failed, expected number, found '{}'. {}",
                        words[2], e)))?;
            let dest = words[5].parse().map_err(|e| AoCError::BadInputFormat(
                format!("Parsing 'swap position' src failed, expected number, found '{}'. {}",
                        words[5], e)))?;
            return Ok(Self::SwapPosition(src, dest))
        }
        if line.starts_with("swap letter ") {
            let words = line.split(' ').collect::<Vec<_>>();
            if words.len() != 6 {
                return Err(AoCError::BadInputFormat(format!(
                    "Malformed 'swap position', expected 'swap letter <char_x> with letter \
                    <char_y>', found '{}'", line)))
            }
            let char_x = words[2].chars().next().ok_or_else(|| AoCError::BadInputFormat(
                format!("Parsing 'swap letter' char_x failed, expected char, found '{}'",
                        words[2])))?;
            let char_y = words[5].chars().next().ok_or_else(|| AoCError::BadInputFormat(
                format!("Parsing 'swap letter' char_y failed, expected char, found '{}'",
                        words[5])))?;
            return Ok(Self::SwapLetters(char_x, char_y))
        }
        if line.starts_with("rotate based on position of letter ") {
            let words = line.split(' ').collect::<Vec<_>>();
            if words.len() != 7 {
                return Err(AoCError::BadInputFormat(format!(
                    "Malformed 'rotate based', expected 'rotate based on position of letter \
                    <char>', found '{}'", line)))
            }
            let char = words[6].chars().next().ok_or_else(|| AoCError::BadInputFormat(
                format!("Parsing 'swap letter' char failed, expected char, found '{}'",
                        words[6])))?;
            return Ok(Self::RotateCharBased(char))
        }
        if line.starts_with("rotate ") {
            let words = line.split(' ').collect::<Vec<_>>();
            if words.len() != 4 {
                return Err(AoCError::BadInputFormat(format!(
                    "Malformed 'rotate [left|right]', expected 'rotate [left|right] <steps> \
                    steps', found '{}'", line)))
            }
            let dir = Direction::parse(words[1])?;
            let steps = words[2].parse().map_err(|e| AoCError::BadInputFormat(
                format!("Parsing 'rotate [left|right]' steps failed, expected number, found '{}'. \
                {}", words[2], e)))?;
            return Ok(Self::RotateSteps(dir, steps))
        }
        if line.starts_with("reverse positions ") {
            let words = line.split(' ').collect::<Vec<_>>();
            if words.len() != 5 {
                return Err(AoCError::BadInputFormat(format!(
                    "Malformed 'reverse positions', expected 'reverse positions <index_start> \
                    through <index_end>', found '{}'", line)))
            }
            let index_start = words[2].parse().map_err(|e| AoCError::BadInputFormat(
                format!("Parsing 'reverse positions' index_start failed, expected number, found \
                '{}'. {}", words[2], e)))?;
            let index_end = words[4].parse().map_err(|e| AoCError::BadInputFormat(
                format!("Parsing 'reverse positions' index_end failed, expected number, found \
                '{}'. {}", words[4], e)))?;
            return Ok(Self::Reverse(index_start, index_end))
        }
        if line.starts_with("move position ") {
            let words = line.split(' ').collect::<Vec<_>>();
            if words.len() != 6 {
                return Err(AoCError::BadInputFormat(format!(
                    "Malformed 'move position', expected 'move position <src> to position <dest>', \
                    found '{}'", line)))
            }
            let src = words[2].parse().map_err(|e| AoCError::BadInputFormat(
                format!("Parsing 'move position' src failed, expected number, found '{}'. {}",
                        words[2], e)))?;
            let dest = words[5].parse().map_err(|e| AoCError::BadInputFormat(
                format!("Parsing 'move position' dest failed, expected number, found '{}'. {}",
                        words[5], e)))?;
            return Ok(Self::MoveChar(src, dest))
        }
        Err(AoCError::BadInputFormat(format!("Unknown instruction '{}", line)))
    }

    pub fn execute(&self, str: &str, unscramble: bool) -> Result<String, AoCError<String>> {
        match self {
            Instruction::SwapPosition(src, dest) => {
                if unscramble {
                    swap_positions(str, *dest, *src)
                } else {
                    swap_positions(str, *src, *dest)
                }
            }
            Instruction::SwapLetters(char_x, char_y) =>
                Ok(swap_letters(str, *char_x, *char_y)),
            Instruction::RotateSteps(dir, steps) => {
                if unscramble {
                    Ok(rotate_steps(str, dir.reverse(), *steps))
                } else {
                    Ok(rotate_steps(str, *dir, *steps))
                }
            }
            Instruction::RotateCharBased(char) => {
                if unscramble {
                    reverse_rotate_char_based(str, *char)
                } else {
                    rotate_char_based(str, *char)
                }
            }
            Instruction::Reverse(index_start, index_end) =>
                reverse(str, *index_start, *index_end),
            Instruction::MoveChar(src, dest) => {
                if unscramble {
                    move_char(str, *dest, *src)
                } else {
                    move_char(str, *src, *dest)
                }
            }
        }
    }
}

fn swap_positions(str: &str, mut src: usize, mut dest: usize) -> Result<String, AoCError<String>> {
    if src > dest {
        swap(&mut src, &mut dest);
    }
    if src >= str.len() {
        return Err(AoCError::BadInputFormat(format!("Swap positions src index out of bounds. \
        Password length {}, index {}", str.len(), src)))
    }
    if dest >= str.len() {
        return Err(AoCError::BadInputFormat(format!("Swap positions dest index out of bounds. \
        Password length {}, index {}", str.len(), dest)))
    }
    Ok(format!("{}{}{}{}{}",
            &str[0..src],
            &str[dest..dest+1],
            &str[src+1..dest],
            &str[src..src+1],
            &str[dest+1..]))
}

fn swap_letters(str: &str, char_x: char, char_y: char) -> String {
    let mut pattern = "#".to_string();
    while str.contains(&pattern) {
        pattern = format!("{}#", pattern);
    }
    let mut res = str.replace(char_x, &pattern);
    res = res.replace(char_y, &char_x.to_string());
    res.replace(&pattern, &char_y.to_string())
}

fn rotate_steps(str: &str, dir: Direction, mut steps: usize) -> String {
    steps %= str.len();
    if dir == Direction::Right {
        steps = (str.len() - steps) % str.len();
    }

    format!("{}{}", &str[steps..], &str[0..steps])
}

fn rotate_char_based(str: &str, char: char) -> Result<String, AoCError<String>> {
    let index = str.find(char)
        .ok_or_else(|| AoCError::BadInputFormat(format!("Char {} for char based rotating is not \
        contained in the password", char)))?;
    let steps = calculate_rotate_steps(index);
    Ok(rotate_steps(str, Direction::Right, steps))
}

fn reverse(str: &str, index_start: usize, index_end: usize) -> Result<String, AoCError<String>> {
    if index_start >= str.len() {
        return Err(AoCError::BadInputFormat(format!("Reverse index_start out of bounds. \
        Password length {}, index {}", str.len(), index_start)))
    }
    if index_end >= str.len() {
        return Err(AoCError::BadInputFormat(format!("Reverse index_end out of bounds. \
        Password length {}, index {}", str.len(), index_end)))
    }
    Ok(format!("{}{}{}",
               &str[0..index_start],
               &str[index_start..=index_end].chars().rev().collect::<String>(),
               &str[index_end+1..]))
}

fn move_char(str: &str, src: usize, dest: usize) -> Result<String, AoCError<String>> {
    if src >= str.len() {
        return Err(AoCError::BadInputFormat(format!("Move src index out of bounds. \
        Password length {}, index {}", str.len(), src)))
    }
    if dest >= str.len() {
        return Err(AoCError::BadInputFormat(format!("Move dest index out of bounds. \
        Password length {}, index {}", str.len(), dest)))
    }

    let mut chars = str.chars().collect::<Vec<_>>();
    let char = chars.remove(src);
    chars.insert(dest, char);
    Ok(chars.iter().collect())
}

fn reverse_rotate_char_based(str: &str, char: char) -> Result<String, AoCError<String>> {
    let char_index = str.find(char)
        .ok_or_else(|| AoCError::BadInputFormat(format!("Char {} for char based rotating is not \
        contained in the password", char)))?;
    let mut steps = None;
    for try_index in 0..str.len() {
        let try_steps = calculate_rotate_steps(try_index);
        if (try_index+ try_steps) % str.len() == char_index {
            if steps.is_none() {
                steps = Some(try_steps);
            } else {
                return Err(AoCError::BadInputFormat(
                    "Char based rotating could not be reversed".to_string()))
            }
        }
    }
    if let Some(steps) = steps {
        Ok(rotate_steps(str, Direction::Left, steps))
    } else {
        Err(AoCError::BadInputFormat(
            "Reversing char based rotating is impossible".to_string()))
    }
}

fn calculate_rotate_steps(char_index: usize) -> usize {
    if char_index >= 4 {
        char_index+2
    } else {
        char_index+1
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn parse(str: &str) -> Result<Direction, AoCError<String>> {
        match str {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err(AoCError::BadInputFormat(format!("Parsing direction failed, expected 'left' or \
                    'right', found {}", str))),
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_swap_positions() -> Result<(), AoCError<String>> {
        let v = vec![
            "swap position 4 with position 0".to_string(),
        ];
        let i = parse_instructions(&v)?;
        let p = "abcde";
        let r = unscramble(&i, &scramble(&i, p)?)?;
        assert_eq!(p, r);
        Ok(())
    }

    #[test]
    fn check_swap_letters() -> Result<(), AoCError<String>> {
        let v = vec![
            "swap letter d with letter b".to_string(),
        ];
        let i = parse_instructions(&v)?;
        let p = "abcde";
        let r = unscramble(&i, &scramble(&i, p)?)?;
        assert_eq!(p, r);
        Ok(())
    }

    #[test]
    fn check_rotate_steps() -> Result<(), AoCError<String>> {
        let v = vec![
            "rotate left 1 step".to_string(),
        ];
        let i = parse_instructions(&v)?;
        let p = "abcde";
        let r = unscramble(&i, &scramble(&i, p)?)?;
        assert_eq!(p, r);
        Ok(())
    }

    #[test]
    fn check_rotate_char_based() -> Result<(), AoCError<String>> {
        let v = vec![
            "rotate based on position of letter h".to_string(),
        ];
        let i = parse_instructions(&v)?;
        let p = "abcdefgh";
        let s = scramble(&i, p)?;
        println!("{}", s);
        let r = unscramble(&i, &s)?;
        assert_eq!(p, r);
        Ok(())
    }

    #[test]
    fn check_reverse() -> Result<(), AoCError<String>> {
        let v = vec![
            "reverse positions 0 through 4".to_string(),
        ];
        let i = parse_instructions(&v)?;
        let p = "abcde";
        let r = unscramble(&i, &scramble(&i, p)?)?;
        assert_eq!(p, r);
        Ok(())
    }

    #[test]
    fn check_reverse_complex() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_21.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        let i = parse_instructions(&input).unwrap();
        let p = "abdcefgh";

        let s = scramble(&i, p).unwrap();
        println!("{}", s);
        let u = unscramble(&i, &s).unwrap();

        assert_eq!(p, &u);
        Ok(())
    }

    #[test]
    fn check_move_char() -> Result<(), AoCError<String>> {
        let v = vec![
            "move position 1 to position 4".to_string(),
        ];
        let i = parse_instructions(&v)?;
        let p = "abcde";
        let r = unscramble(&i, &scramble(&i, p)?)?;
        assert_eq!(p, r);
        Ok(())
    }

    #[test]
    fn check_examples_part_1() -> Result<(), AoCError<String>> {
        let v = vec![
            "swap position 4 with position 0".to_string(),
            "swap letter d with letter b".to_string(),
            "reverse positions 0 through 4".to_string(),
            "rotate left 1 step".to_string(),
            "move position 1 to position 4".to_string(),
            "move position 3 to position 0".to_string(),
            "rotate based on position of letter b".to_string(),
            "rotate based on position of letter d".to_string(),
        ];
        let instructions = parse_instructions(&v)?;
        assert_eq!(scramble(&instructions, "abcde"), Ok("decab".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_21.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("gbhcefad".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_21.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("gahedfcb".to_string()));
        Ok(())
    }
}
use crate::errors::AoCError;
use crate::string_manipulation::*;

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
            let dir = parse_direction(words[1])?;
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

pub fn parse_direction(str: &str) -> Result<Direction, AoCError<String>> {
    match str {
        "left" => Ok(Direction::Left),
        "right" => Ok(Direction::Right),
        _ => Err(AoCError::BadInputFormat(format!("Parsing direction failed, expected 'left' or \
        'right', found {}", str))),
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
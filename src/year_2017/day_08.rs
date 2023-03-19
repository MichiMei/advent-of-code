use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let instructions = parse_instructions(input)?;
    Ok(execute_instructions(&instructions).0.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let instructions = parse_instructions(input)?;
    Ok(execute_instructions(&instructions).1.to_string())
}

fn parse_instructions(input: &[String]) -> Result<Vec<Instruction>, AoCError<String>> {
    input.iter().map(|line| Instruction::parse(line)).collect()
}

fn execute_instructions(instructions: &[Instruction]) -> (i32, i32) {
    let mut registers = HashMap::new();
    let mut max = 0;
    for instruction in instructions.iter() {
        let res = instruction.execute(&mut registers);
        if let Some(res) = res {
            if res > max {
                max = res;
            }
        }
    }
    (*registers.values().max().unwrap_or(&0), max)
}

enum Instruction<'a> {
    Increase(&'a str, i32, Condition<'a>),
    Decrease(&'a str, i32, Condition<'a>),
}

impl<'a> Instruction<'a> {
    fn parse(line: &'a str) -> Result<Self, AoCError<String>> {
        let parts = line.split(" if ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(AoCError::BadInputFormat(format!(
                "Parsing instruction failed, expected '<reg> [inc|dec] <val> if <reg> <condition> \
                <val>', found '{}'.", line)))
        }
        let condition = Condition::parse(parts[1])?;
        let words = parts[0].split_whitespace().collect::<Vec<_>>();
        if words.len() != 3 {
            return Err(AoCError::BadInputFormat(format!(
                "Parsing instruction failed, expected '<reg> [inc|dec] <val> if <reg> <condition> \
                <val>', found '{}'.", line)))
        }
        let register = words[0];
        let value = words[2].parse()
            .map_err(|e| AoCError::BadInputFormat(format!(
                "Parsing condition value failed. Expected number, found '{}'. {}", words[2], e)))?;
        Ok(match words[1] {
            "inc" => Self::Increase(register, value, condition),
            "dec" => Self::Decrease(register, value, condition),
            x => return Err(AoCError::BadInputFormat(
                format!("Unexpected instruction '{}'. Only 'inc' or 'dec' supported.", x)))
        })
    }

    fn execute(&self, registers: &mut HashMap<&'a str, i32>) -> Option<i32> {
        match self {
            Instruction::Increase(reg, val, condition) => {
                if condition.check_condition(registers) {
                    let prev = *registers.get(reg).unwrap_or(&0);
                    registers.insert(reg, prev+val);
                    return Some(prev+val)
                }
                None
            }
            Instruction::Decrease(reg, val, condition) => {
                if condition.check_condition(registers) {
                    let prev = *registers.get(reg).unwrap_or(&0);
                    registers.insert(reg, prev-val);
                    return Some(prev-val)
                }
                None
            }
        }
    }
}

impl<'a> Display for Instruction<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Increase(reg, val, cond) =>
                write!(f, "{} inc {} if {}", reg, val, cond),
            Instruction::Decrease(reg, val, cond) =>
                write!(f, "{} dec {} if {}", reg, val, cond),
        }
    }
}

enum Condition<'a> {
    Equals(&'a str, i32),
    NotEquals(&'a str, i32),
    Greater(&'a str, i32),
    GreaterOrEqual(&'a str, i32),
    Less(&'a str, i32),
    LessOrEqual(&'a str, i32),
}

impl<'a> Condition<'a> {
    fn parse(str: &'a str) -> Result<Self, AoCError<String>> {
        let words = str.split_whitespace().collect::<Vec<_>>();
        let register = words[0];
        let value = words[2].parse()
            .map_err(|e| AoCError::BadInputFormat(format!(
                "Parsing condition value failed. Expected number, found '{}'. {}", words[2], e)))?;
        Ok(match words[1] {
            "==" => Self::Equals(register, value),
            "!=" => Self::NotEquals(register, value),
            ">" => Self::Greater(register, value),
            ">=" => Self::GreaterOrEqual(register, value),
            "<" => Self::Less(register, value),
            "<=" => Self::LessOrEqual(register, value),
            x => return Err(AoCError::BadInputFormat(
                format!("Unexpected condition '{}'. Only '==', '!=', '>', '>=', '<' and '<=' \
                supported.", x)))
        })
    }

    fn check_condition(&self, registers: &HashMap<&str, i32>) -> bool {
        match self {
            Condition::Equals(reg, val) =>
                registers.get(reg).unwrap_or(&0) == val,
            Condition::NotEquals(reg, val) =>
                registers.get(reg).unwrap_or(&0) != val,
            Condition::Greater(reg, val) =>
                registers.get(reg).unwrap_or(&0) > val,
            Condition::GreaterOrEqual(reg, val) =>
                registers.get(reg).unwrap_or(&0) >= val,
            Condition::Less(reg, val) =>
                registers.get(reg).unwrap_or(&0) < val,
            Condition::LessOrEqual(reg, val) =>
                registers.get(reg).unwrap_or(&0) <= val,
        }
    }
}

impl<'a> Display for Condition<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::Equals(reg, val) => write!(f, "{} == {}", reg, val),
            Condition::NotEquals(reg, val) => write!(f, "{} != {}", reg, val),
            Condition::Greater(reg, val) => write!(f, "{} > {}", reg, val),
            Condition::GreaterOrEqual(reg, val) => write!(f, "{} >= {}", reg, val),
            Condition::Less(reg, val) => write!(f, "{} < {}", reg, val),
            Condition::LessOrEqual(reg, val) => write!(f, "{} <= {}", reg, val),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "b inc 5 if a > 1".to_string(),
            "a inc 1 if b < 5".to_string(),
            "c dec -10 if a >= 1".to_string(),
            "c inc -20 if c == 10".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let v = get_example_input();
        assert_eq!(part_1(&v), Ok("1".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_08.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("6012".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = get_example_input();
        assert_eq!(part_2(&v), Ok("10".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_08.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("6369".to_string()));
        Ok(())
    }
}
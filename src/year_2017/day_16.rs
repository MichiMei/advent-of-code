use std::collections::HashMap;
use crate::errors::AoCError;
use crate::string_manipulation::{Direction, rotate_steps, swap_letters, swap_positions};

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the comma-separated list of instructions."
                .to_string()))
    }
    let instructions = parse_instructions(&input[0])?;
    let end = (b'a' + 16) as char;
    let input = ('a'..end).collect::<String>();
    execute_program_dance(&input, &instructions)
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the comma-separated list of instructions."
                .to_string()))
    }
    let instructions = parse_instructions(&input[0])?;
    repeat_dance(16, &instructions, 1000000000)
}

fn parse_instructions(line: &str) -> Result<Vec<Instruction>, AoCError<String>> {
    line.split(',').map(Instruction::parse).collect()
}

fn execute_program_dance(input: &str, instructions: &Vec<Instruction>)
    -> Result<String, AoCError<String>> {
    let mut input = input.to_string();
    for instruction in instructions {
        input = instruction.execute(&input)?;
    }
    Ok(input)
}

fn repeat_dance(program_count: u8, instructions: &Vec<Instruction>, repetitions: usize)
    -> Result<String, AoCError<String>> {
    let end = (b'a' + program_count) as char;
    let mut input = ('a'..end).collect::<String>();

    let (loop_start, loop_end, loop_str) =
        find_loop(&input, instructions, repetitions)?;

    let simulation_end = simulate_dance(loop_start, loop_end, repetitions);

    input = loop_str;
    for _ in simulation_end..repetitions {
        input = execute_program_dance(&input, instructions)?;
    }

    Ok(input)
}

fn find_loop(input: &str, instructions: &Vec<Instruction>, repetitions: usize) -> Result<(usize, usize, String), AoCError<String>> {
    let mut input = input.to_string();
    let mut cache = HashMap::new();
    for index in 0..repetitions {
        if let Some(start) = cache.get(&input) {
            return Ok((*start, index, input))
        }
        cache.insert(input.clone(), index);
        input = execute_program_dance(&input, instructions)?;
    }
    Ok((0, repetitions, input))
}

fn simulate_dance(loop_start: usize, loop_end: usize, repetitions: usize) -> usize {
    let loop_size = loop_end - loop_start;
    let remaining_repetitions = repetitions - loop_start;
    let loop_executions = remaining_repetitions / loop_size;

    loop_start + (loop_executions*loop_size)
}

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Instruction {
    fn parse(str: &str) -> Result<Self, AoCError<String>> {
        if let Some(remainder) = str.strip_prefix('s') {
            let steps = remainder.parse().map_err(|e| AoCError::BadInputFormat(
                format!("Parsing 'steps' from 's<steps>' failed. Expected number, found '{}'. {}",
                        remainder, e)))?;
            return Ok(Self::Spin(steps))
        }
        if let Some(remainder) = str.strip_prefix('x') {
            let words = remainder.split('/').collect::<Vec<_>>();
            let index0 = words[0].parse().map_err(|e| AoCError::BadInputFormat(
                format!("Parsing 'index0' from 'x<index0>/<index1>' failed. Expected number, found \
                '{}'. {}", words[0], e)))?;
            let index1 = words[1].parse().map_err(|e| AoCError::BadInputFormat(
                format!("Parsing 'index1' from 'x<index0>/<index1>' failed. Expected number, found \
                '{}'. {}", words[1], e)))?;
            return Ok(Self::Exchange(index0, index1))
        }
        if let Some(remainder) = str.strip_prefix('p') {
            if remainder.len() != 3 {
                return Err(AoCError::BadInputFormat(format!(
                    "'p<char0>/<char1>' malformed. Found '{}'.", str)))
            }
            let char0 = remainder.chars().next().expect("str has length 4");
            let char1 = remainder.chars().nth(2).expect("str has length 4");
            return Ok(Self::Partner(char0, char1))
        }
        Err(AoCError::BadInputFormat(format!(
            "Unknown instruction '{}'. Only 's<steps>', 'x<index0>/<index1>' and 'p<char>/<char>' \
            supported.", str)))
    }

    fn execute(&self, str: &str) -> Result<String, AoCError<String>> {
        match self {
            Instruction::Spin(steps) => Ok(rotate_steps(str, Direction::Right, *steps)),
            Instruction::Exchange(i0, i1) => swap_positions(str, *i0, *i1),
            Instruction::Partner(c0, c1) =>
                Ok(swap_letters(str, *c0, *c1)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() -> Result<(), AoCError<String>> {
        let instructions = parse_instructions("s1,x3/4,pe/b")?;
        assert_eq!(execute_program_dance("abcde", &instructions), Ok("baedc".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 16)?;
        assert_eq!(part_1(&input), Ok("olgejankfhbmpidc".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() -> Result<(), AoCError<String>> {
        let instructions = parse_instructions("s1,x3/4,pe/b")?;
        assert_eq!(repeat_dance(5, &instructions, 2),
                   Ok("ceadb".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 16)?;
        assert_eq!(part_2(&input), Ok("gfabehpdojkcimnl".to_string()));
        Ok(())
    }
}
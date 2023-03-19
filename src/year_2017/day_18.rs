use std::collections::{HashMap, VecDeque};
use crate::errors::AoCError;


pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let instructions = input.iter()
        .map(|line| Instruction::parse(line))
        .collect::<Result<Vec<_>, _>>()?;
    calculate_first_recover(&instructions)
        .map(|res| res.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let instructions = input.iter()
        .map(|line| Instruction::parse(line))
        .collect::<Result<Vec<_>, _>>()?;
    let (p0, p1) = create_programs(&instructions);
    run_programs(p0, p1)
        .map(|res| res.to_string())
}

fn calculate_first_recover(instructions: &Vec<Instruction>) -> Result<i64, AoCError<String>> {
    let mut registers = Registers::new(&[]);
    let mut played = None;
    let mut ip = 0;
    while ip < instructions.len() {
        match instructions[ip].execute(true, &mut registers, &mut played) {
            ExecutionResult::Successful => ip += 1,
            ExecutionResult::Jump(ip_change) => {
                let new_ip = ip as i128 + ip_change as i128;
                ip = usize::try_from(new_ip)
                    .map_err(|e| AoCError::NoSolutionFoundError(
                        format!("Execution ended without successful recover. {}", e)))?;
            }
            ExecutionResult::Wait => {
                return if let Some(prev_send) = played {
                    Ok(prev_send)
                } else {
                    Err(AoCError::NoSolutionFoundError(
                        "Execution ended without successful recover.".to_string()))
                }
            }
            ExecutionResult::Send(val) => {
                ip += 1;
                played = Some(val);
            }
        }
    }
    Err(AoCError::NoSolutionFoundError("Execution ended without successful recover.".to_string()))
}

fn create_programs(instructions: &[Instruction]) -> (Program, Program) {
    let program0 = Program::new(0, instructions);
    let program1 = Program::new(1, instructions);
    (program0, program1)
}

fn run_programs(mut program0: Program, mut program1: Program) -> Result<usize, AoCError<String>> {
    let mut p1_counter = 0;
    let mut p0_finished = false;
    let mut p0_is_wait = false;
    let mut p1_finished = false;
    let mut p1_is_wait = false;
    while !p0_finished || !p1_finished {
        if p0_is_wait && p1_is_wait {
            return Ok(p1_counter)
        }
        if !p0_finished && !p0_is_wait {
            match program0.run() {
                SystemCall::End => p0_finished = true,
                SystemCall::Send(value) => {
                    program1.add_input(value);
                    p1_is_wait = false;
                }
                SystemCall::Wait => p0_is_wait = true,
            }
        }
        if !p1_finished && !p1_is_wait {
            match program1.run() {
                SystemCall::End => p1_finished = true,
                SystemCall::Send(value) => {
                    program0.add_input(value);
                    p0_is_wait = false;
                    p1_counter += 1;
                }
                SystemCall::Wait => p1_is_wait = true,
            }
        }
    }
    Ok(p1_counter)
}

struct Program<'a> {
    instructions: &'a [Instruction],
    ip: usize,
    registers: Registers,
    input_queue: VecDeque<i64>,
}

impl<'a> Program<'a> {
    fn new(program_id: i64, instructions: &'a [Instruction]) -> Self {
        let ip = 0;
        let input_queue = VecDeque::new();
        let registers = Registers::new(&[('p', program_id)]);
        Self{instructions, ip, registers, input_queue}
    }

    fn run(&mut self) -> SystemCall {
        loop {
            if self.ip >= self.instructions.len() {
                return SystemCall::End
            }
            let mut input = self.input_queue.front().copied();
            match self.instructions[self.ip].execute(false, &mut self.registers, &mut input) {
                ExecutionResult::Successful => self.ip += 1,
                ExecutionResult::Jump(ip_change) => {
                    let new_ip = self.ip as i128 + ip_change as i128;
                    if let Ok(value) = usize::try_from(new_ip) {
                        self.ip = value;
                    } else {
                        self.ip = usize::MAX;
                        return SystemCall::End
                    }
                }
                ExecutionResult::Wait => return SystemCall::Wait,
                ExecutionResult::Send(value) => {
                    self.ip += 1;
                    return SystemCall::Send(value)
                }
            }
            if input.is_none() {
                self.input_queue.pop_front();
            }
        }
    }

    fn add_input(&mut self, input: i64) {
        self.input_queue.push_back(input)
    }
}

struct Registers {
    data: HashMap<char, i64>,
}

impl Registers {
    fn new(defaults: &[(char, i64)]) -> Self {
        let data = defaults.iter().copied().collect();
        Self{data}
    }

    fn set(&mut self, index: char, value: i64) {
        self.data.insert(index, value);
    }

    fn get(&self, index: char) -> i64 {
        *self.data.get(&index).unwrap_or(&0)
    }
}

enum SystemCall {
    End,
    Send(i64),
    Wait,
}

enum Instruction {
    Send(Parameter),
    Set(Parameter, Parameter),
    Addition(Parameter, Parameter),
    Multiplication(Parameter, Parameter),
    Modulo(Parameter, Parameter),
    Receive(Parameter),
    JumpGreaterZero(Parameter, Parameter),
}

impl Instruction {
    fn parse(line: &str) -> Result<Self, AoCError<String>> {
        let words = line.split_whitespace().collect::<Vec<_>>();
        match words[0] {
            "snd" => {
                if words.len() != 2 {
                    return Err(AoCError::BadInputFormat(format!(
                        "Malformed 'snd'. Expected 'snd <param>', found '{}'.", line)))
                }
                let param = Parameter::parse(words[1])?;
                Ok(Self::Send(param))
            }
            "set" => {
                if words.len() != 3 {
                    return Err(AoCError::BadInputFormat(format!(
                        "Malformed 'set'. Expected 'set <param0> <param1>', found '{}'.", line)))
                }
                let param0 = Parameter::parse(words[1])?;
                let param1 = Parameter::parse(words[2])?;
                Ok(Self::Set(param0, param1))
            }
            "add" => {
                if words.len() != 3 {
                    return Err(AoCError::BadInputFormat(format!(
                        "Malformed 'add'. Expected 'add <param0> <param1>', found '{}'.", line)))
                }
                let param0 = Parameter::parse(words[1])?;
                let param1 = Parameter::parse(words[2])?;
                Ok(Self::Addition(param0, param1))
            }
            "mul" => {
                if words.len() != 3 {
                    return Err(AoCError::BadInputFormat(format!(
                        "Malformed 'mul'. Expected 'mul <param0> <param1>', found '{}'.", line)))
                }
                let param0 = Parameter::parse(words[1])?;
                let param1 = Parameter::parse(words[2])?;
                Ok(Self::Multiplication(param0, param1))
            }
            "mod" => {
                if words.len() != 3 {
                    return Err(AoCError::BadInputFormat(format!(
                        "Malformed 'mod'. Expected 'mod <param0> <param1>', found '{}'.", line)))
                }
                let param0 = Parameter::parse(words[1])?;
                let param1 = Parameter::parse(words[2])?;
                Ok(Self::Modulo(param0, param1))
            }
            "rcv" => {
                if words.len() != 2 {
                    return Err(AoCError::BadInputFormat(format!(
                        "Malformed 'rcv'. Expected 'rcv <param>', found '{}'.", line)))
                }
                let param = Parameter::parse(words[1])?;
                Ok(Self::Receive(param))
            }
            "jgz" => {
                if words.len() != 3 {
                    return Err(AoCError::BadInputFormat(format!(
                        "Malformed 'jgz'. Expected 'jgz <param0> <param1>', found '{}'.", line)))
                }
                let param0 = Parameter::parse(words[1])?;
                let param1 = Parameter::parse(words[2])?;
                Ok(Self::JumpGreaterZero(param0, param1))
            }
            x => Err(AoCError::BadInputFormat(format!(
                "Unknown instruction '{}'. Only 'snd', 'set', 'add', 'mul', 'mod', 'rcv' and 'jgz' \
                supported.", x)))
        }
    }

    fn execute(&self, single: bool, registers: &mut Registers, input: &mut Option<i64>)
               -> ExecutionResult {
        match self {
            Instruction::Send(p) => {
                return ExecutionResult::Send(p.get_value(registers))
            }
            Instruction::Set(p0, p1) => {
                let value = p1.get_value(registers);
                p0.set_register(registers, value);
            }
            Instruction::Addition(p0, p1) => {
                let value = p0.get_value(registers) + p1.get_value(registers);
                p0.set_register(registers, value);
            }
            Instruction::Multiplication(p0, p1) => {
                let value = p0.get_value(registers) * p1.get_value(registers);
                p0.set_register(registers, value);
            }
            Instruction::Modulo(p0, p1) => {
                let value = p0.get_value(registers) % p1.get_value(registers);
                p0.set_register(registers, value);
            }
            Instruction::Receive(p) => {
                if single {
                    if p.get_value(registers) != 0 {
                        return ExecutionResult::Wait
                    }
                } else if let Some(input) = input.take() {
                    p.set_register(registers, input);
                } else {
                    return ExecutionResult::Wait
                }
            }
            Instruction::JumpGreaterZero(p0, p1) => {
                if p0.get_value(registers) > 0 {
                    return ExecutionResult::Jump(p1.get_value(registers))
                }
            }
        }
        ExecutionResult::Successful
    }
}

#[derive(Debug)]
enum ExecutionResult {
    Successful,
    Jump(i64),
    Wait,
    Send(i64),
}

enum Parameter {
    Value(i64),
    Register(char),
}

impl Parameter {
    fn parse(str: &str) -> Result<Self, AoCError<String>> {
        if let Ok(value) = str.parse() {
            Ok(Self::Value(value))
        } else if str.len() == 1 {
            let char = str.chars().next().expect("Length is 1");
            Ok(Self::Register(char))
        } else {
            Err(AoCError::BadInputFormat(format!(
                "Parsing parameter failed. Expected char or number, found '{}'.", str)))
        }
    }

    fn get_value(&self, registers: &Registers) -> i64 {
        match self {
            Parameter::Value(val) => *val,
            Parameter::Register(reg) => registers.get(*reg),
        }
    }

    fn set_register(&self, registers: &mut Registers, value: i64) -> bool {
        match self {
            Parameter::Value(_) => false,
            Parameter::Register(reg) => {
                registers.set(*reg, value);
                true
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "set a 1".to_string(),
            "add a 2".to_string(),
            "mul a a".to_string(),
            "mod a 5".to_string(),
            "snd a".to_string(),
            "set a 0".to_string(),
            "rcv a".to_string(),
            "jgz a -1".to_string(),
            "set a 1".to_string(),
            "jgz a -2".to_string(),
        ];
        assert_eq!(part_1(&v), Ok("4".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_18.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("8600".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = vec![
            "snd 1".to_string(),
            "snd 2".to_string(),
            "snd p".to_string(),
            "rcv a".to_string(),
            "rcv b".to_string(),
            "rcv c".to_string(),
            "rcv d".to_string(),
        ];
        assert_eq!(part_2(&v), Ok("3".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_18.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("7239".to_string()));
        Ok(())
    }
}
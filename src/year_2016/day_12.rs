use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut sim = AssembunnySimulator::from_input(input)?;
    let registers = sim.run();

    Ok(registers[0].to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut sim = AssembunnySimulator::from_input(input)?;
    sim.set_registers([0, 0, 1, 0]);
    let registers = sim.run();

    Ok(registers[0].to_string())
}

struct AssembunnySimulator {
    instructions: Vec<Instruction>,
    registers: [i32; 4],
    instruction_pointer: usize,
}

impl AssembunnySimulator {
    pub fn from_input(input: &[String]) -> Result<Self, AoCError<String>> {
        let instructions = input.iter()
            .map(|line| Instruction::parse(line))
            .collect::<Result<Vec<_>, _>>()?;
        let registers = [0; 4];
        let instruction_pointer = 0;
        Ok(Self{instructions, registers, instruction_pointer})
    }

    pub fn set_registers(&mut self, registers: [i32; 4]) {
        self.registers = registers;
    }

    pub fn run(&mut self) -> [i32; 4] {
        while self.instruction_pointer < self.instructions.len() {
            let ip_change =
                self.instructions[self.instruction_pointer].execute(&mut self.registers);
            let new_ip = self.instruction_pointer as i64 + ip_change as i64;
            self.instruction_pointer = if let Ok(val) = usize::try_from(new_ip) {
                val
            } else {
                break;
            };
        }
        self.registers
    }
}

enum Instruction {
    Copy(Parameter, Parameter),
    Increment(Parameter),
    Decrement(Parameter),
    JumpNotZero(Parameter, Parameter),
}

impl Instruction {
    pub fn parse(line: &str) -> Result<Self, AoCError<String>> {
        let words = line.split(' ').collect::<Vec<_>>();
        match words[0] {
            "cpy" => {
                if words.len() != 3 {
                    return Err(AoCError::BadInputFormat(format!(
                        "Instruction malformed, expected 'cpy <src> <dest>'. Found '{}'", line)))
                }
                Ok(Self::Copy(
                    Parameter::parse(words[1])?,
                    Parameter::parse_register(words[2])?
                ))
            }
            "inc" => {
                if words.len() != 2 {
                    return Err(AoCError::BadInputFormat(format!(
                        "Instruction malformed, expected 'inc <reg>'. Found '{}'", line)))
                }
                Ok(Self::Increment(
                    Parameter::parse_register(words[1])?
                ))
            }
            "dec" => {
                if words.len() != 2 {
                    return Err(AoCError::BadInputFormat(format!(
                        "Instruction malformed, expected 'dec <reg>'. Found '{}'", line)))
                }
                Ok(Self::Decrement(
                    Parameter::parse_register(words[1])?
                ))
            }
            "jnz" => {
                if words.len() != 3 {
                    return Err(AoCError::BadInputFormat(format!(
                        "Instruction malformed, expected 'jnz <cmp> <dist>'. Found '{}'", line)))
                }
                Ok(Self::JumpNotZero(
                    Parameter::parse(words[1])?,
                    Parameter::parse(words[2])?
                ))
            }
            x => {
                return Err(AoCError::BadInputFormat(format!(
                    "Unknown instruction, expected 'cpy', 'inc', 'dec' or 'jnz'. Found '{}'", x)))
            }
        }
    }

    /// Executes the instruction and returns the change to the instruction-pointer.
    /// Usually the change is 1, for jnz it may differ
    pub fn execute(&self, registers: &mut [i32; 4]) -> i32 {
        match self {
            Instruction::Copy(src, dest) => {
                match dest {
                    Parameter::Register(reg) => {
                        registers[*reg] = src.get_value(registers);
                    }
                    Parameter::Value(_) => panic!("Copy destination was a value"),
                }
            }
            Instruction::Increment(reg) => {
                match reg {
                    Parameter::Register(reg) => {
                        registers[*reg] += 1;
                    }
                    Parameter::Value(_) => panic!("Increment register was a value"),
                }
            }
            Instruction::Decrement(reg) => {
                match reg {
                    Parameter::Register(reg) => {
                        registers[*reg] -= 1;
                    }
                    Parameter::Value(_) => panic!("Increment register was a value"),
                }
            }
            Instruction::JumpNotZero(reg, dist) => {
                if reg.get_value(registers) != 0 {
                    return dist.get_value(registers)
                }
            }
        }
        1
    }
}

enum Parameter {
    Register(usize),
    Value(i32),
}

impl Parameter {
    pub fn parse(str: &str) -> Result<Self, AoCError<String>> {
        if let Ok(param) = Self::parse_register(str) { return Ok(param) }
        if let Ok(param) = Self::parse_value(str) { return Ok(param) }
        Err(AoCError::BadInputFormat(format!("Parsing parameter failed. Expected an Integer or one \
            of the registers 'a' - 'd'. Found '{}'", str)))
    }

    pub fn parse_register(str: &str) -> Result<Self, AoCError<String>> {
        match str {
            "a" => Ok(Self::Register(0)),
            "b" => Ok(Self::Register(1)),
            "c" => Ok(Self::Register(2)),
            "d" => Ok(Self::Register(3)),
            _ => Err(AoCError::BadInputFormat(format!(
                    "Expected one of the registers 'a' - 'd'. Found '{}'", str)))
        }
    }

    pub fn parse_value(str: &str) -> Result<Self, AoCError<String>> {
        str.parse()
            .map(Self::Value)
            .map_err(|_| AoCError::BadInputFormat(format!(
                "Expected a value (Integer). Found '{}'", str)))
    }

    pub fn get_value(&self, registers: &[i32; 4]) -> i32 {
        match self {
            Parameter::Register(reg) => registers[*reg],
            Parameter::Value(val) => *val,
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
            "cpy 41 a".to_string(),
            "inc a".to_string(),
            "inc a".to_string(),
            "dec a".to_string(),
            "jnz a 2".to_string(),
            "dec a".to_string(),
        ];

        assert_eq!(part_1(&v), Ok("42".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_12.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("318007".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_12.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("9227661".to_string()));
        Ok(())
    }
}
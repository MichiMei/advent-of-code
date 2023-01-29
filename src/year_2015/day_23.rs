pub fn part_1(input: &[String]) -> Result<String, &str> {
    let mut pc = SOTAComputer::from_input(input, 0)?;
    pc.run();
    Ok(pc.register_b.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, &str> {
    let mut pc = SOTAComputer::from_input(input, 1)?;
    pc.run();
    Ok(pc.register_b.to_string())
}

struct SOTAComputer {
    register_a: usize,
    register_b: usize,
    instructions: Vec<Instruction>,
    ip: Option<usize>,
}

impl SOTAComputer {
    fn from_input(input: &[String], register_a: usize) -> Result<Self, &'static str> {
        let register_b = 0;
        let mut instructions = vec![];
        for line in input.iter() {
            instructions.push(Instruction::from(line).ok_or(ERR_INPUT_MALFORMED)?);
        }
        let ip = Some(0);
        Ok(Self{register_a, register_b, instructions, ip})
    }

    fn run(&mut self) {
        while self.ip.is_some() {
            self.step()
        }
    }

    fn step(&mut self) {
        assert!(self.ip.is_some());

        match self.instructions[self.ip.unwrap()] {
            Instruction::Half(reg) => self.half(reg),
            Instruction::Triple(reg) => self.triple(reg),
            Instruction::Increment(reg) => self.increment(reg),
            Instruction::Jump(plus, offset) => self.jump(plus, offset),
            Instruction::JumpEven(reg, plus, offset) =>
                self.jump_even(reg, plus, offset),
            Instruction::JumpOne(reg, plus, offset) =>
                self.jump_one(reg, plus, offset),
        }
    }

    fn half(&mut self, reg: Register) {
        match reg {
            Register::A => self.register_a /= 2,
            Register::B => self.register_b /= 2,
        }
        self.move_ip_forwards(1);
    }

    fn triple(&mut self, reg: Register) {
        match reg {
            Register::A => self.register_a *= 3,
            Register::B => self.register_b *= 3,
        }
        self.move_ip_forwards(1);
    }

    fn increment(&mut self, reg: Register) {
        match reg {
            Register::A => self.register_a += 1,
            Register::B => self.register_b += 1,
        }
        self.move_ip_forwards(1);
    }

    fn jump(&mut self, plus: bool, offset: usize) {
        match plus {
            true => {
                self.move_ip_forwards(offset);
            }
            false => {
                self.move_ip_backwards(offset);
            }
        }
    }

    fn jump_even(&mut self, reg: Register, plus: bool, offset: usize) {
        let val = match reg {
            Register::A => self.register_a,
            Register::B => self.register_b,
        };
        if val % 2 == 0 {
            self.jump(plus, offset);
        } else {
            self.move_ip_forwards(1);
        }
    }

    fn jump_one(&mut self, reg: Register, plus: bool, offset: usize) {
        let val = match reg {
            Register::A => self.register_a,
            Register::B => self.register_b,
        };
        if val == 1 {
            self.jump(plus, offset);
        } else {
            self.move_ip_forwards(1);
        }
    }

    fn move_ip_forwards(&mut self, offset: usize) {
        self.ip = Some(self.ip.unwrap()+offset);
        if self.ip.unwrap() >= self.instructions.len() {
            self.ip = None;
        }
    }

    fn move_ip_backwards(&mut self, offset: usize) {
        if offset > self.ip.unwrap() {
            self.ip = None;
        } else {
            self.ip = Some(self.ip.unwrap() - offset);
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(bool, usize),
    JumpEven(Register, bool, usize),
    JumpOne(Register, bool, usize),
}

impl Instruction {
    fn from(line: &str) -> Option<Self> {
        let line = line.replace(',', "");
        let words: Vec<&str> = line.split(' ').collect();
        if words.len() < 2 || words.len() > 3 {
            return None
        }
        match words[0] {
            "hlf" => {
                if words.len() != 2 {
                    return None
                }
                let register = Register::from(words[1])?;
                Some(Self::Half(register))
            }
            "tpl" => {
                if words.len() != 2 {
                    return None
                }
                let register = Register::from(words[1])?;
                Some(Self::Triple(register))
            }
            "inc" => {
                if words.len() != 2 {
                    return None
                }
                let register = Register::from(words[1])?;
                Some(Self::Increment(register))
            }
            "jmp" => {
                if words.len() != 2 || words[1].len() < 2 {
                    return None
                }
                let plus = match words[1].chars().next().unwrap() {
                    '+' => true,
                    '-' => false,
                    _ => return None,
                };
                let offset = words[1][1..].parse().ok()?;
                Some(Self::Jump(plus, offset))
            }
            "jie" => {
                if words.len() != 3 || words[2].len() < 2 {
                    return None
                }
                let register = Register::from(words[1])?;
                let plus = match words[2].chars().next().unwrap() {
                    '+' => true,
                    '-' => false,
                    _ => return None,
                };
                let offset = words[2][1..].parse().ok()?;
                Some(Self::JumpEven(register, plus, offset))
            }
            "jio" => {
                if words.len() != 3 || words[2].len() < 2 {
                    return None
                }
                let register = Register::from(words[1])?;
                let plus = match words[2].chars().next().unwrap() {
                    '+' => true,
                    '-' => false,
                    _ => return None,
                };
                let offset = words[2][1..].parse().ok()?;
                Some(Self::JumpOne(register, plus, offset))
            }
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
enum Register {
    A,
    B,
}

impl Register {
    fn from(str: &str) -> Option<Self> {
        if str.len() != 1 {
            return None
        }
        Some(match str.chars().next().unwrap() {
            'a' => Self::A,
            'b' => Self::B,
            _ => return None
        })
    }
}

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() -> Result<(), &'static str> {
        let v = vec![
            "inc a".to_string(),
            "jio a, +2".to_string(),
            "tpl a".to_string(),
            "inc a".to_string(),
        ];

        let mut pc = SOTAComputer::from_input(&v, 0)?;
        pc.run();

        assert_eq!(pc.register_a, 2);
        Ok(())
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_23.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("184".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_23.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("231".to_string()));
        Ok(())
    }
}
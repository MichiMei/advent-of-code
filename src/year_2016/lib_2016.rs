pub mod assembunny {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};
    use std::hash::{Hash, Hasher};
    use crate::errors::AoCError;

    pub struct AssembunnySimulator {
        instructions: Vec<Instruction>,
        registers: [i32; 4],
        instruction_pointer: usize,
        output: Vec<i32>,
    }

    impl AssembunnySimulator {
        pub fn from_input(input: &[String]) -> Result<Self, AoCError<String>> {
            let instructions = input.iter()
                .map(|line| Instruction::parse(line))
                .collect::<Result<Vec<_>, _>>()?;
            let registers = [0; 4];
            let instruction_pointer = 0;
            let output = vec![];
            Ok(Self{instructions, registers, instruction_pointer, output})
        }

        pub fn set_registers(&mut self, registers: [i32; 4]) {
            self.registers = registers;
        }

        pub fn run(&mut self) -> [i32; 4] {
            while self.instruction_pointer < self.instructions.len() {
                if !self.step() {
                    break
                }
            }
            self.registers
        }

        pub fn run_until_loop(&mut self) -> ([i32; 4], &[i32], Option<usize>) {
            let mut state_cache = HashMap::new();

            while self.instruction_pointer < self.instructions.len() {
                let current_state = self.get_state();
                if let Some(prev_output_length) = state_cache.get(&current_state) {
                    return (self.registers, &self.output, Some(*prev_output_length))
                }
                state_cache.insert(current_state, self.output.len());

                if !self.step() {
                    break
                }
            }
            (self.registers, &self.output, None)
        }

        pub fn optimize(&mut self) {
            optimize_instructions(&mut self.instructions);
        }

        fn step(&mut self) -> bool {
            let (ip_change, toggle) =
                self.instructions[self.instruction_pointer]
                    .execute(&mut self.registers, &mut self.output);
            if let Some(toggle) = toggle {
                let toggle_index = self.instruction_pointer as i64 + toggle as i64;
                if let Ok(index) = usize::try_from(toggle_index) {
                    if index < self.instructions.len() {
                        self.instructions[index] = self.instructions[index].toggle();
                    }
                }
            }
            let new_ip = self.instruction_pointer as i64 + ip_change as i64;
            self.instruction_pointer = if let Ok(val) = usize::try_from(new_ip) {
                val
            } else {
                return false
            };
            true
        }

        fn get_state(&self) -> (u64, Vec<i32>, usize) {
            let mut hasher = DefaultHasher::new();
            self.instructions.hash(&mut hasher);
            let i_hash = hasher.finish();

            (i_hash, self.registers.to_vec(), self.instruction_pointer)
        }
    }

    fn optimize_instructions(instructions: &mut [Instruction]) {
        find_transfers(instructions);
        find_multiplies(instructions);
    }

    /// Searches for simple addition like optimizations.
    /// Transfers are a loop like: inc x, dec y, jnz y -2 (or inc, dec reversed).
    /// Replaces them by a simple instruction causing x+=y, y=0, ip+=3.
    fn find_transfers(instructions: &mut [Instruction]) {
        let mut optimizable = vec![];

        for (index, window) in instructions.windows(3).enumerate() {
            let jnz_params = if let Some(params) = window[2].is_jnz() {
                params
            } else {
                continue
            };

            let inc_param = if let Some(param) = window[0].is_inc() {
                param
            } else if let Some(param) = window[1].is_inc() {
                param
            } else {
                continue
            };

            let dec_param = if let Some(param) = window[0].is_dec() {
                param
            } else if let Some(param) = window[1].is_dec() {
                param
            } else {
                continue
            };

            if dec_param == jnz_params.0 {
                let transfer = Instruction::create_transfer(dec_param,
                                                            inc_param, 3);
                optimizable.push((index, transfer));
            }

        }

        for (index, transfer) in optimizable {
            instructions[index] = transfer;
        }
    }

    /// Searches for simple multiply like optimizations.
    /// Multiplies are a loop like: (dec can be also before transfer or cpy)
    /// 1. cpy a b
    /// 2. transfer b c 3
    /// 3. XXX
    /// 4. XXX
    /// 5. dec d
    /// 6. jnz d -5
    /// Replaces them by a simple instruction causing x+=y, y=0, ip+=3.
    fn find_multiplies(instructions: &mut [Instruction]) {
        let mut optimizable = vec![];

        for (index, window) in instructions.windows(6).enumerate() {
            let cpy_params = if let Some(params) = window[0].is_cpy() {
                params
            } else if let Some(params) = window[1].is_cpy() {
                params
            } else {
                continue
            };

            let transfer_params = if let Some(params) = window[1].is_transfer() {
                params
            } else if let Some(params) = window[2].is_transfer() {
                params
            } else {
                continue
            };

            let dec_param = if let Some(param) = window[0].is_dec() {
                param
            } else if let Some(param) = window[1].is_dec() {
                param
            } else if let Some(param) = window[4].is_dec() {
                param
            } else {
                continue
            };

            let jnz_params = if let Some(params) = window[5].is_jnz() {
                params
            } else {
                continue
            };

            if cpy_params.0 != cpy_params.1 &&
                cpy_params.1 == transfer_params.0 &&
                transfer_params.2 == 3 &&
                dec_param == jnz_params.0 &&
                cpy_params.0 != transfer_params.1 &&
                cpy_params.0 != dec_param &&
                cpy_params.1 != dec_param &&
                transfer_params.1 != dec_param {
                let multiply =
                    Instruction::create_multiply(cpy_params.0,dec_param,
                                                 transfer_params.1, 6);
                optimizable.push((index, multiply));
            }
        }

        for (index, multiply) in optimizable {
            instructions[index] = multiply;
        }
    }

    #[derive(Copy, Clone, Hash)]
    enum Instruction {
        Copy(Parameter, Parameter),
        Increment(Parameter),
        Decrement(Parameter),
        JumpNotZero(Parameter, Parameter),
        Toggle(Parameter),
        Transfer(Parameter, Parameter, i32),
        Multiply(Parameter, Parameter, Parameter, i32),
        Output(Parameter),
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
                "tgl" => {
                    if words.len() != 2 {
                        return Err(AoCError::BadInputFormat(format!(
                            "Instruction malformed, expected 'tgl <dist>'. Found '{}'", line)))
                    }
                    Ok(Self::Toggle(
                        Parameter::parse(words[1])?
                    ))
                }
                "out" => {
                    if words.len() != 2 {
                        return Err(AoCError::BadInputFormat(format!(
                            "Instruction malformed, expected 'out <param>'. Found '{}'", line)))
                    }
                    Ok(Self::Output(
                        Parameter::parse(words[1])?
                    ))
                }
                x => {
                    Err(AoCError::BadInputFormat(format!(
                        "Unknown instruction, expected 'cpy', 'inc', 'dec' or 'jnz'. Found '{}'", x)))
                }
            }
        }

        /// Executes the instruction and returns the change to the instruction-pointer as well as
        /// a possible toggle value.
        /// Usually the change is 1, for jnz it may differ.
        /// The toggle value is None for all instructions except toggle. toggle returns a change
        /// relative to the current instruction pointer, indicating which operation to toggle.
        pub fn execute(&self, registers: &mut [i32; 4], output: &mut Vec<i32>)
            -> (i32, Option<i32>) {
            let mut ip_change = 1;
            let mut toggle = None;
            match self {
                Instruction::Copy(src, dest) => {
                    match dest {
                        Parameter::Register(reg) => {
                            registers[*reg] = src.get_value(registers);
                        }
                        Parameter::Value(_) => {}
                    }
                }
                Instruction::Increment(reg) => {
                    match reg {
                        Parameter::Register(reg) => {
                            registers[*reg] += 1;
                        }
                        Parameter::Value(_) => {}
                    }
                }
                Instruction::Decrement(reg) => {
                    match reg {
                        Parameter::Register(reg) => {
                            registers[*reg] -= 1;
                        }
                        Parameter::Value(_) => {}
                    }
                }
                Instruction::JumpNotZero(reg, dist) => {
                    if reg.get_value(registers) != 0 {
                        ip_change = dist.get_value(registers)
                    }
                }
                Instruction::Toggle(dist) => {
                    toggle = Some(dist.get_value(registers))
                }
                Instruction::Transfer(src, dest, jump) => {
                    match dest {
                        Parameter::Register(dest_reg) => {
                            registers[*dest_reg] += src.get_value(registers);
                            match src {
                                Parameter::Register(src_reg) => {
                                    registers[*src_reg] = 0;
                                }
                                Parameter::Value(_) => {}
                            }
                        }
                        Parameter::Value(_) => {}
                    }
                    ip_change = *jump;
                }
                Instruction::Multiply(op0, op1, dest,
                                      jump) => {
                    match dest {
                        Parameter::Register(dest_reg) => {
                            registers[*dest_reg] +=
                                op0.get_value(registers)*op1.get_value(registers);
                        }
                        Parameter::Value(_) => {}
                    }
                    ip_change = *jump;
                }
                Instruction::Output(val) => {
                    output.push(val.get_value(registers));
                }
            }
            (ip_change, toggle)
        }

        /// Toggles the instruction and returns the new one.
        /// The following changes are made by toggle:
        /// 1. Increment -> Decrement
        /// 2. all other unary instructions -> Increment
        /// 3. JumpNotZero -> Copy
        /// 4. all other binary instructions -> JumpNotZero
        pub fn toggle(self) -> Self {
            match self {
                Instruction::Copy(p0, p1) =>
                    Instruction::JumpNotZero(p0, p1),
                Instruction::Increment(p) => Instruction::Decrement(p),
                Instruction::Decrement(p) => Instruction::Increment(p),
                Instruction::JumpNotZero(p0, p1) =>
                    Instruction::Copy(p0, p1),
                Instruction::Toggle(p) => Instruction::Increment(p),
                Instruction::Transfer(_, _, _) => unimplemented!("transfer cant be toggled"),
                Instruction::Multiply(_, _, _, _) => unimplemented!("transfer cant be toggled"),
                Instruction::Output(p) => Instruction::Increment(p),
            }
        }

        pub fn is_inc(&self) -> Option<Parameter> {
            match self {
                Instruction::Increment(p) => Some(*p),
                _ => None,
            }
        }

        pub fn is_dec(&self) -> Option<Parameter> {
            match self {
                Instruction::Decrement(p) => Some(*p),
                _ => None,
            }
        }

        pub fn is_jnz(&self) -> Option<(Parameter, Parameter)> {
            match self {
                Instruction::JumpNotZero(p0, p1) => Some((*p0, *p1)),
                _ => None,
            }
        }

        pub fn is_cpy(&self) -> Option<(Parameter, Parameter)> {
            match self {
                Instruction::Copy(p0, p1) => Some((*p0, *p1)),
                _ => None,
            }
        }

        pub fn is_transfer(&self) -> Option<(Parameter, Parameter, i32)> {
            match self {
                Instruction::Transfer(p0, p1, j) => Some((*p0, *p1, *j)),
                _ => None,
            }
        }

        pub fn create_transfer(src: Parameter, dest: Parameter, jump: i32) -> Self {
            Self::Transfer(src, dest, jump)
        }

        pub fn create_multiply(op0: Parameter, op1: Parameter, dest: Parameter, jump: i32) -> Self {
            Self::Multiply(op0, op1, dest, jump)
        }
    }

    impl Display for Instruction {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Instruction::Copy(p0, p1) => write!(f, "cpy {} {}", p0, p1),
                Instruction::Increment(p) => write!(f, "inc {}", p),
                Instruction::Decrement(p) => write!(f, "dec {}", p),
                Instruction::JumpNotZero(p0, p1) =>
                    write!(f, "jnz {} {}", p0, p1),
                Instruction::Toggle(p) => write!(f, "tgl {}", p),
                Instruction::Transfer(p0, p1, j) =>
                    write!(f, "TRANSFER {} {} {}", p0, p1, j),
                Instruction::Multiply(op0, op1, dest, j) =>
                    write!(f, "MULTIPLY {}*{}->{} {}", op0, op1, dest, j),
                Instruction::Output(p) => write!(f, "out {}", p),
            }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

    impl Display for Parameter {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Parameter::Register(reg) => {
                    match reg {
                        0 => write!(f, "a"),
                        1 => write!(f, "b"),
                        2 => write!(f, "c"),
                        3 => write!(f, "d"),
                        _ => panic!(),
                    }
                }
                Parameter::Value(val) => write!(f, "{}", val),
            }
        }
    }
}
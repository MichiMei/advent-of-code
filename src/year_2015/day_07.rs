use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{BitAnd, BitOr, Not, Shl, Shr};

pub fn part_1(input: &Vec<String>) -> Result<String, &str> {
    let mut variables = HashMap::new();
    let mut gates = vec![];
    for line in input {
        let gate = Gate::from(&line)?;
        if let Some((name, val)) = gate.is_input() {
            variables.insert(name, val);
        } else {
            gates.push(gate);
        }
    }

    while !gates.is_empty() {
        let mut unsolvable = vec![];
        for gate in gates {
            match gate.execute(&variables) {
                Ok((v_name, v_val)) => {
                     if let Some(prev)  = variables.insert(v_name, v_val) {
                         if prev != v_val {
                             return Err(ERR_VARIABLE_REASSIGNED)
                         }
                     }
                }
                Err(g) => unsolvable.push(g),
            }
        }
        gates = unsolvable;
    }

    if let Some(val) = variables.get("a") {
        Ok(val.to_string())
    } else {
        Err(ERR_VARIABLE_A_MISSING)
    }
}

pub fn part_2(input: &Vec<String>) -> Result<String, &str> {
    let mut variables = HashMap::new();
    let mut gates = vec![];
    for line in input {
        let gate = Gate::from(&line)?;
        if let Some((name, mut val)) = gate.is_input() {
            if &name == "b" {
                val = 956;
            }
            variables.insert(name, val);
        } else {
            gates.push(gate);
        }
    }

    while !gates.is_empty() {
        let mut unsolvable = vec![];
        for gate in gates {
            match gate.execute(&variables) {
                Ok((v_name, v_val)) => {
                    if let Some(prev)  = variables.insert(v_name.clone(), v_val) {
                        if prev != v_val {
                            println!("variable {} ({}) set to {}", v_name, prev, v_val);
                            return Err(ERR_VARIABLE_REASSIGNED)
                        }
                    }
                }
                Err(g) => unsolvable.push(g),
            }
        }
        gates = unsolvable;
    }

    if let Some(val) = variables.get("a") {
        Ok(val.to_string())
    } else {
        Err(ERR_VARIABLE_A_MISSING)
    }
}

enum Gate {
    Noop(Input, String),
    Not(Input, String),
    And(Input, Input, String),
    Or(Input, Input, String),
    LShift(Input, Input, String),
    RShift(Input, Input, String),
}

impl Gate {
    pub fn from(str: &str) -> Result<Self, &str> {
        let words: Vec<&str> = str.split(" ").collect();
        let operation = if words.len() == 3 {
            ""
        } else if words.len() == 4 {
            words[0]
        } else if words.len() == 5 {
            words[1]
        } else {
            return Err(ERR_INPUT_MALFORMED)
        };
        Ok(match operation {
            "" => {
                let in0 = Input::from(words[0]);
                let out = words[2].to_string();
                Self::Noop(in0?, out)
            }
            "NOT" => {
                let in0 = Input::from(words[1]);
                let out = words[3].to_string();
                Self::Not(in0?, out)
            }
            "AND" => {
                let in0 = Input::from(words[0]);
                let in1 = Input::from(words[2]);
                let out = words[4].to_string();
                Self::And(in0?, in1?, out)
            }
            "OR" => {
                let in0 = Input::from(words[0]);
                let in1 = Input::from(words[2]);
                let out = words[4].to_string();
                Self::Or(in0?, in1?, out)
            }
            "LSHIFT" => {
                let in0 = Input::from(words[0]);
                let in1 = Input::from(words[2]);
                let out = words[4].to_string();
                Self::LShift(in0?, in1?, out)
            }
            "RSHIFT" => {
                let in0 = Input::from(words[0]);
                let in1 = Input::from(words[2]);
                let out = words[4].to_string();
                Self::RShift(in0?, in1?, out)
            }
            _ => return Err(ERR_INPUT_MALFORMED),
        })
    }

    pub fn execute(self, variables: &HashMap<String, u16>) -> Result<(String, u16), Self> {
        match &self {
            Gate::Noop(in0, out) => {
                if let Some(val) = in0.get_value(variables) {
                    Ok((out.to_string(), val))
                } else {
                    Err(self)
                }
            }
            Gate::Not(in0, out) => {
                if let Some(val) = in0.get_value(variables) {
                    Ok((out.to_string(), val.not()))
                } else {
                    Err(self)
                }
            }
            Gate::And(in0, in1, out) => {
                let v0 = in0.get_value(variables);
                let v1 = in1.get_value(variables);
                if v0.is_some() && v1.is_some() {
                    Ok((out.to_string(), v0.unwrap().bitand(v1.unwrap())))
                } else {
                    Err(self)
                }
            }
            Gate::Or(in0, in1, out) => {
                let v0 = in0.get_value(variables);
                let v1 = in1.get_value(variables);
                if v0.is_some() && v1.is_some() {
                    Ok((out.to_string(), v0.unwrap().bitor(v1.unwrap())))
                } else {
                    Err(self)
                }
            }
            Gate::LShift(in0, in1, out) => {
                let v0 = in0.get_value(variables);
                let v1 = in1.get_value(variables);
                if v0.is_some() && v1.is_some() {
                    Ok((out.to_string(), v0.unwrap().shl(v1.unwrap())))
                } else {
                    Err(self)
                }
            }
            Gate::RShift(in0, in1, out) => {
                let v0 = in0.get_value(variables);
                let v1 = in1.get_value(variables);
                if v0.is_some() && v1.is_some() {
                    Ok((out.to_string(), v0.unwrap().shr(v1.unwrap())))
                } else {
                    Err(self)
                }
            }
        }
    }

    pub fn is_input(&self) -> Option<(String, u16)> {
        match self {
            Gate::Noop(Input::Value(v), o) => Some((o.to_string(), *v)),
            _ => None,
        }
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Gate::Noop(i, o) => write!(f, "{} -> {}", i, o),
            Gate::Not(i, o) => write!(f, "NOT {} -> {}", i, o),
            Gate::And(i0, i1, o) => write!(f, "{} AND {} -> {}", i0, i1, o),
            Gate::Or(i0, i1, o) => write!(f, "{} OR {} -> {}", i0, i1, o),
            Gate::LShift(i0, i1, o) => write!(f, "{} LSHIFT {} -> {}", i0, i1, o),
            Gate::RShift(i0, i1, o) => write!(f, "{} RSHIFT {} -> {}", i0, i1, o),
        }
    }
}

impl Debug for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Input {
    Variable(String),
    Value(u16),
}

impl Input {
    pub fn from(str: &str) -> Result<Self, &str> {
        if let Ok(val) = str.parse() {
            Ok(Self::Value(val))
        } else {
            if str.is_empty() {
                Err(ERR_INPUT_MALFORMED)
            } else {
                Ok(Self::Variable(str.to_string()))
            }
        }
    }

    pub fn get_value(&self, variables: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Input::Variable(name) => {
                if let Some(val) = variables.get(name) {
                    Some(*val)
                } else {
                    None
                }
            }
            Input::Value(val) => Some(*val),
        }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Variable(var) => write!(f, "{}", var),
            Input::Value(val) => write!(f, "{}", val),
        }
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

const ERR_INPUT_MALFORMED: &str = "Input is malformed";
const ERR_VARIABLE_REASSIGNED: &str = "A variable was assigned two different values";
const ERR_VARIABLE_A_MISSING: &str = "The value for 'a' could not be calculated";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let i0 = vec![
            "123 -> x".to_string(),
            "456 -> y".to_string(),
            "x AND y -> a".to_string()
        ];
        let i1 = vec![
            "123 -> x".to_string(),
            "456 -> y".to_string(),
            "x OR y -> a".to_string()
        ];
        let i2 = vec![
            "123 -> x".to_string(),
            "456 -> y".to_string(),
            "x LSHIFT 2 -> a".to_string()
        ];
        let i3 = vec![
            "123 -> x".to_string(),
            "456 -> y".to_string(),
            "y RSHIFT 2 -> a".to_string()
        ];
        let i4 = vec![
            "123 -> x".to_string(),
            "456 -> y".to_string(),
            "NOT x -> a".to_string()
        ];
        let i5 = vec![
            "123 -> x".to_string(),
            "456 -> y".to_string(),
            "NOT y -> a".to_string()
        ];

        assert_eq!(part_1(&i0), Ok("72".to_string()));
        assert_eq!(part_1(&i1), Ok("507".to_string()));
        assert_eq!(part_1(&i2), Ok("492".to_string()));
        assert_eq!(part_1(&i3), Ok("114".to_string()));
        assert_eq!(part_1(&i4), Ok("65412".to_string()));
        assert_eq!(part_1(&i5), Ok("65079".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_07.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("956".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_07.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("40149".to_string()));
        Ok(())
    }
}
use std::collections::{HashMap, VecDeque};
use std::ops::Not;
use crate::errors::{AoCError, AoCResult};

pub fn part_1(input: &[String]) -> AoCResult<String> {
    let mut modules = Module::parse_map(input)?;
    let mut sums = (0, 0);
    for _ in 0..1000 {
        let vals = press_button(&mut modules)?;
        sums.0 += vals.0;
        sums.1 += vals.1;
    }
    Ok((sums.0*sums.1).to_string())
}

#[allow(unused_variables)]
pub fn part_2(input: &Vec<String>) -> AoCResult<String> {
    todo!()
}

fn press_button(modules: &mut HashMap<String, Module>) -> AoCResult<(usize, usize)> {
    let mut priority_queue = VecDeque::new();
    priority_queue.push_back(("broadcaster".to_string(), Pulse::Low, "button".to_string()));

    let mut low_count = 0;
    let mut high_count = 0;
    while let Some((receiver, pulse, sender)) = priority_queue.pop_front() {
        if pulse == Pulse::Low {
            low_count += 1;
        } else {
            high_count += 1;
        }
        if let Some(module) = modules.get_mut(&receiver) {
            module.execute(pulse, &sender).into_iter()
                .for_each(|response| priority_queue.push_back(response));
        }
    }
    Ok((low_count, high_count))
}

struct Module {
    name: String,
    gate: Gate,
    outputs: Vec<String>,
    inputs: usize,
}

impl Module {
    fn parse_map(input: &[String]) -> AoCResult<HashMap<String, Self>> {
        let mut res = HashMap::new();
        let mut input_counts = HashMap::new();
        for line in input {
            let module = Module::parse(line)?;
            for output in module.outputs.iter() {
                if let Some(count) = input_counts.get(&output[..]) {
                    input_counts.insert(output.clone(), count+1);
                } else {
                    input_counts.insert(output.clone(), 1);
                }
            }
            res.insert(module.name.clone(), module);
        }
        for (name, count) in input_counts.into_iter() {
            if let Some(module) = res.get_mut(&name) {
                module.inputs = count;
            }
        }

        Ok(res)
    }

    fn parse(line: &str) -> AoCResult<Self> {
        let split = line.split(" -> ").collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(AoCError::BadInputFormat(
                format!("Parsing Module failed, ' -> ' not found in {}.", line)))
        }
        if split[0].is_empty() {
            return Err(AoCError::BadInputFormat("Parsing Module failed, name is empty".to_string()))
        }
        let gate = Gate::parse(split[0].chars().next()
            .expect("Is not empty"))?;
        let name = if gate == Gate::Broadcast {
            split[0].to_string()
        } else {
            split[0][1..].to_string()
        };
        let outputs = split[1].split(", ")
            .map(|str| str.to_string())
            .collect::<Vec<_>>();
        Ok(Self{
            name,
            gate,
            outputs,
            inputs: 0,
        })
    }

    fn execute(&mut self, pulse: Pulse, sender: &str) -> Vec<(String, Pulse, String)> {
        if let Some(output) = self.gate.execute(pulse, sender, self.inputs) {
            self.outputs.iter()
                .map(|receiver| (receiver.clone(), output, self.name.clone()))
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }
}

#[derive(Eq, PartialEq)]
enum Gate {
    FlipFlop(Pulse),
    Conjunction(HashMap<String, Pulse>),
    Broadcast,
}

impl Gate {
    fn parse(c: char) -> AoCResult<Self> {
        match c {
            'b' => Ok(Self::Broadcast),
            //'%' => Ok(Self::FlipFlop(Box::new(Pulse::Low))),
            '%' => Ok(Self::FlipFlop(Pulse::Low)),
            '&' => Ok(Self::Conjunction(HashMap::new())),
            _ => Err(AoCError::BadInputFormat(format!("Parsing Gate failed, unknown type '{}'", c)))
        }
    }

    fn execute(&mut self, pulse: Pulse, sender: &str, inputs: usize) -> Option<Pulse> {
        match self {
            Gate::FlipFlop(ref mut prev_pulse) => {
                if pulse == Pulse::Low {
                    prev_pulse.toggle();
                    Some(*prev_pulse)
                } else {
                    None
                }
            }
            Gate::Conjunction(last_pulses) => {
                last_pulses.insert(sender.to_string(), pulse);
                if last_pulses.values().filter(|p| **p == Pulse::High).count() == inputs {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }

            }
            Gate::Broadcast => Some(pulse),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn toggle(&mut self) {
        if *self == Self::Low {
            *self = Self::High;
        } else {
            *self = Self::Low;
        }
    }
}

impl Not for Pulse {
    type Output = Pulse;
    fn not(self) -> Self::Output {
        match self {
            Pulse::Low => Self::High,
            Pulse::High => Self::Low,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input_1() -> Vec<String> {
        vec![
            "broadcaster -> a, b, c".to_string(),
            "%a -> b".to_string(),
            "%b -> c".to_string(),
            "%c -> inv".to_string(),
            "&inv -> a".to_string(),
        ]
    }

    fn get_example_input_2() -> Vec<String> {
        vec![
            "broadcaster -> a".to_string(),
            "%a -> inv, con".to_string(),
            "&inv -> b".to_string(),
            "%b -> con".to_string(),
            "&con -> output".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input_1();
        assert_eq!(part_1(&input), Ok("32000000".to_string()));
        let input = get_example_input_2();
        assert_eq!(part_1(&input), Ok("11687500".to_string()));
    }

    #[test]
    fn check_input_part_1() -> AoCResult<()> {
        let input = get_input(2023, 20)?;
        assert_eq!(part_1(&input), Ok("791120136".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> AoCResult<()> {
        let input = get_input(2023, 20)?;
        assert_eq!(part_2(&input), Ok("expected".to_string())); // TODO
        Ok(())
    }
}
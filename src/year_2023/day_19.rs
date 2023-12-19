use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::errors::{AoCError, AoCResult};

pub fn part_1(input: &[String]) -> AoCResult<String> {
    let split = input.split(|line| line.is_empty()).collect::<Vec<_>>();
    if split.len() != 2 {
        return Err(AoCError::BadInputFormat(
            "Input has no empty line splitting workflows from parts".to_string()))
    }
    let workflows = Workflow::get_map(split[0])?;
    let mut parts = Part::get_list(split[1])?;
    for part in parts.iter_mut() {
        while !part.execute(&workflows)? {}
    }
    Ok(parts.iter()
        .filter(|part| part.target == Target::Accepted)
        .map(|part| part.sum())
        .sum::<usize>()
        .to_string())
}

pub fn part_2(input: &[String]) -> AoCResult<String> {
    let split = input.split(|line| line.is_empty()).collect::<Vec<_>>();
    if split.len() != 2 {
        return Err(AoCError::BadInputFormat(
            "Input has no empty line splitting workflows from parts".to_string()))
    }
    let workflows = Workflow::get_map(split[0])?;
    let mut finished = vec![];
    let mut range_parts = vec![RangePart::default()];
    while let Some(current) = range_parts.pop() {
        let tmp = current.execute(&workflows)?;
        for elem in tmp {
            match elem.target {
                Target::Accepted => finished.push(elem),
                Target::Rejected => {}
                Target::Workflow(_) => range_parts.push(elem)
            }
        }
    }
    Ok(finished.iter()
        .map(|elem| elem.sum())
        .sum::<usize>().to_string())

}

struct RangePart {
    variables: [RangeInclusive; 4],
    target: Target,
}

impl RangePart {
    fn new(variables: [RangeInclusive; 4], target: Target) -> Self {
        Self {
            variables,
            target,
        }
    }

    fn execute(self, workflows: &HashMap<String, Workflow>) -> AoCResult<Vec<RangePart>> {
        if let Target::Workflow(name) = &self.target {
            if let Some(workflow) = workflows.get(name) {
                let range_parts = workflow.execute_range(self);
                Ok(range_parts)
            } else {
                Err(AoCError::NoSolutionFoundError(
                    format!("Could not find workflow '{}'", name)))
            }
        } else {
            Ok(vec![self])
        }
    }

    fn sum(&self) -> usize {
        self.variables.iter().map(|range| range.end-range.start+1).product()
    }
}

impl Default for RangePart {
    fn default() -> Self {
        Self {
            variables: Default::default(),
            target: Target::Workflow("in".to_string()),
        }
    }
}

impl Display for RangePart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(x={},m={},a={},s={})", self.target, self.variables[0], self.variables[1],
               self.variables[2], self.variables[3])
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct RangeInclusive {
    start: usize,
    end: usize
}

impl RangeInclusive {
    fn new(start: usize, end: usize) -> Self {
        assert!(end >= start);
        Self {
            start,
            end
        }
    }

    /// First returned value is anything smaller than the given number, the second returned value
    /// is the rest
    fn split_smaller_than(&self, number: usize) -> (Option<Self>, Option<Self>) {
        if number <= self.start {
            return (None, Some(*self))
        }
        if number > self.end {
            return (Some(*self), None)
        }
        let first = Self::new(self.start, number-1);
        let second = Self::new(number, self.end);
        (Some(first), Some(second))
    }

    /// First returned value is greater smaller than the given number, the second returned value
    /// is the rest
    fn split_greater_than(&self, number: usize) -> (Option<Self>, Option<Self>) {
        if number >= self.end {
            return (None, Some(*self))
        }
        if number < self.start {
            return (Some(*self), None)
        }
        let first = Self::new(number+1, self.end);
        let second = Self::new(self.start, number);
        (Some(first), Some(second))
    }
}

impl Default for RangeInclusive {
    fn default() -> Self {
        Self {
            start: 1,
            end: 4000,
        }
    }
}

impl Display for RangeInclusive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{}]", self.start, self.end)
    }
}

struct Part {
    variables: [usize; 4],
    target: Target,
}

impl Part {
    fn get_list(input: &[String]) -> AoCResult<Vec<Self>> {
        input.iter()
            .map(|line| Self::parse(line))
            .collect()
    }

    fn new() -> Self {
        Self {
            variables: [0; 4],
            target: Target::Workflow("in".to_string()),
        }
    }

    fn parse(mut line: &str) -> AoCResult<Self> {
        if !line.starts_with('{') || !line.ends_with('}') {
            return Err(AoCError::BadInputFormat(
                format!("Parsing Part failed. Expected it to start with '{{' and end with '}}'. \
                Found '{}'", line)))
        }
        line = &line[1..line.len()-1];
        let split = line.split(',').collect::<Vec<_>>();
        if split.len() != 4 {
            return Err(AoCError::BadInputFormat(
                format!("Parsing Part failed, expected 4 comma separated variables, found {}",
                        split.len())))
        }
        let mut res = Self::new();
        for elem in split {
            res.parse_value(elem)?;
        }
        Ok(res)
    }

    fn parse_value(&mut self, str: &str) -> AoCResult<()> {
        if let Some(pos) = str.find('=') {
            let index = Variable::parse(&str[..pos])?.get_index();
            let value = str[pos+1..].parse::<usize>()
                .map_err(|e| AoCError::BadInputFormat(
                    format!("Parsing value from '{}' failed. {}", str, e)))?;
            self.variables[index] = value;
            Ok(())
        } else {
            Err(AoCError::BadInputFormat(
                format!("Parsing Part value failed. Could not find '=' in '{}'.", str)))
        }
    }

    fn execute(&mut self, workflows: &HashMap<String, Workflow>) -> AoCResult<bool> {
        if let Target::Workflow(name) = &self.target {
            if let Some(workflow) = workflows.get(name) {
                let new_target = workflow.execute(self);
                self.target = new_target;
                Ok(false)
            } else {
                Err(AoCError::NoSolutionFoundError(
                    format!("Could not find workflow '{}'", name)))
            }
        } else {
            Ok(true)
        }
    }

    fn sum(&self) -> usize {
        self.variables.iter().sum()
    }
}

struct Workflow {
    rules: Vec<Rule>,
    default: Target,
}

impl Workflow {
    fn get_map(input: &[String]) -> AoCResult<HashMap<String, Self>> {
        input.iter()
            .map(|line| Self::parse(line))
            .collect()
    }

    fn parse(line: &str) -> AoCResult<(String, Self)> {
        let pos_open = line.find('{')
            .ok_or_else(|| AoCError::BadInputFormat(
                format!("Parsing workflow failed, could not find '{{' in '{}'", line)))?;
        let pos_close = line.find('}')
            .ok_or_else(|| AoCError::BadInputFormat(
                format!("Parsing workflow failed, could not find '}}' in '{}'", line)))?;
        let name = line[..pos_open].to_string();
        let rule_str = &line[pos_open+1..pos_close];
        let split = rule_str.split(',').collect::<Vec<_>>();
        let rules = split[..split.len()-1].iter()
            .map(|str| Rule::parse(str))
            .collect::<AoCResult<Vec<_>>>()?;
        let default = Target::parse(split[split.len()-1]);

        let workflow = Self {
            rules,
            default,
        };
        Ok((name, workflow))
    }

    fn execute(&self, part: &Part) -> Target {
        for rule in self.rules.iter() {
            if let Some(target) = rule.execute(part) {
                return target
            }
        }
        self.default.clone()
    }

    fn execute_range(&self, part: RangePart) -> Vec<RangePart> {
        let mut res = vec![];
        let mut current = part;
        for rule in self.rules.iter() {
            let (modified, remaining) =
                rule.execute_range(current);
            if let Some(range_part) = modified {
                res.push(range_part);
            }
            if let Some(range_part) = remaining {
                current = range_part;
            } else {
                return res
            }
        }
        current.target = self.default.clone();
        res.push(current);
        res
    }
}

#[derive(Debug)]
enum Rule {
    GreaterThan(Variable, usize, Target),
    SmallerThan(Variable, usize, Target),
}

impl Rule {
    fn parse(str: &str) -> AoCResult<Self> {
        if let Some(pos_colon) = str.find(':') {
            let target = Target::parse(&str[pos_colon+1..]);
            if let Some(pos_smaller) = str.find('<') {
                let variable = Variable::parse(&str[0..pos_smaller])?;
                let value = str[pos_smaller+1..pos_colon].parse::<usize>()
                    .map_err(|e| AoCError::BadInputFormat(
                        format!("Parsing value from '{}' failed. {}", str, e)))?;
                Ok(Self::SmallerThan(variable, value, target))
            } else if let Some(pos_greater) = str.find('>') {
                let variable = Variable::parse(&str[0..pos_greater])?;
                let value = str[pos_greater+1..pos_colon].parse::<usize>()
                    .map_err(|e| AoCError::BadInputFormat(
                        format!("Parsing value from '{}' failed. {}", str, e)))?;
                Ok(Self::GreaterThan(variable, value, target))
            } else {
                Err(AoCError::BadInputFormat(
                    format!("Parsing rule failed, neither '<' nor '>' found in '{}'", str)))
            }
        } else {
            Err(AoCError::BadInputFormat(
                format!("Parsing rule failed, ':' not found in '{}'", str)))
        }
    }

    fn execute(&self, part: &Part) -> Option<Target> {
        match self {
            Rule::GreaterThan(var, val, tar) => {
                if part.variables[var.get_index()] > *val {
                    Some(tar.clone())
                } else {
                    None
                }
            }
            Rule::SmallerThan(var, val, tar) => {
                if part.variables[var.get_index()] < *val {
                    Some(tar.clone())
                } else {
                    None
                }
            }
        }
    }

    /// The first Option contains the modified part, the second option the remaining unmodified rest
    fn execute_range(&self, part: RangePart) -> (Option<RangePart>, Option<RangePart>) {
        match self {
            Rule::GreaterThan(var, val, tar) => {
                let relevant_range = part.variables[var.get_index()];
                let (greater, smaller) =
                    relevant_range.split_greater_than(*val);

                let greater =
                    Self::option_mapper(greater, &part, var, tar.clone());
                let smaller =
                    Self::option_mapper(smaller, &part, var, part.target.clone());
                (greater, smaller)
            }
            Rule::SmallerThan(var, val, tar) => {
                let relevant_range = part.variables[var.get_index()];
                let (smaller, greater) =
                    relevant_range.split_smaller_than(*val);

                let smaller =
                    Self::option_mapper(smaller, &part, var, tar.clone());
                let greater =
                    Self::option_mapper(greater, &part, var, part.target.clone());
                (smaller, greater)
            }
        }
    }

    fn option_mapper(option: Option<RangeInclusive>, rp: &RangePart, var: &Variable, tar: Target)
        -> Option<RangePart>
    {
        option.map(|range| {
            let mut variables = rp.variables;
            variables[var.get_index()] = range;
            RangePart::new(variables, tar)
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Target {
    Accepted,
    Rejected,
    Workflow(String),
}

impl Target {
    fn parse(str: &str) -> Self {
        match str {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            str => Self::Workflow(str.to_string()),
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Accepted => write!(f, "Accepted"),
            Target::Rejected => write!(f, "Rejected"),
            Target::Workflow(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug)]
enum Variable {
    X,
    M,
    A,
    S,
}

impl Variable {
    fn parse(str: &str) -> AoCResult<Self> {
        match str {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            str => Err(AoCError::BadInputFormat(
                format!("Parsing variable failed, only 'x', 'm', 'a' and 's' supported. \
                Found '{}'", str)))
        }
    }

    fn get_index(&self) -> usize {
        match self {
            Variable::X => 0,
            Variable::M => 1,
            Variable::A => 2,
            Variable::S => 3,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "px{a<2006:qkq,m>2090:A,rfg}".to_string(),
            "pv{a>1716:R,A}".to_string(),
            "lnx{m>1548:A,A}".to_string(),
            "rfg{s<537:gd,x>2440:R,A}".to_string(),
            "qs{s>3448:A,lnx}".to_string(),
            "qkq{x<1416:A,crn}".to_string(),
            "crn{x>2662:A,R}".to_string(),
            "in{s<1351:px,qqz}".to_string(),
            "qqz{s>2770:qs,m<1801:hdj,R}".to_string(),
            "gd{a>3333:R,R}".to_string(),
            "hdj{m>838:A,pv}".to_string(),
            "".to_string(),
            "{x=787,m=2655,a=1222,s=2876}".to_string(),
            "{x=1679,m=44,a=2067,s=496}".to_string(),
            "{x=2036,m=264,a=79,s=2244}".to_string(),
            "{x=2461,m=1339,a=466,s=291}".to_string(),
            "{x=2127,m=1623,a=2188,s=1013}".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("19114".to_string()));
    }

    #[test]
    fn check_input_part_1() -> AoCResult<()> {
        let input = get_input(2023, 19)?;
        assert_eq!(part_1(&input), Ok("348378".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("167409079868000".to_string()));
    }

    #[test]
    fn check_input_part_2() -> AoCResult<()> {
        let input = get_input(2023, 19)?;
        assert_eq!(part_2(&input), Ok("121158073425385".to_string()));
        Ok(())
    }
}
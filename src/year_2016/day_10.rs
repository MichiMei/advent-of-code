use std::cmp::{max, min};
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut factory = Factory::from_input(input)?;
    let result = factory.find_comparator(61, 17)?;

    Ok(result.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut factory = Factory::from_input(input)?;
    let result = factory.calculate_output()?;

    Ok(result.to_string())
}

#[derive(Clone)]
struct Bot {
    chips: Vec<usize>,
    low_target: Target,
    high_target: Target,
}

impl Bot {
    pub fn parse_bot(line: &str) -> Result<(Self, usize), AoCError<String>> {
        let words = line.split(' ').collect::<Vec<_>>();
        if words.len() != 12 ||
            words[0] != "bot" ||
            words[2] != "gives" ||
            words[3] != "low" ||
            words[4] != "to" ||
            words[7] != "and" ||
            words[8] != "high" ||
            words[9] != "to" {
            return Err(AoCError::BadInputFormat(format!("Bad bot declaration. Expected 'bot x \
                gives low to [bot|output] y and high to [bot|output] z'. Found '{}'", line)))
        }

        let bot_index = words[1].parse().map_err(|e| AoCError::BadInputFormat(format!("Parsing bot \
            index failed. Expected integer, found '{}'. {}", words[1], e)))?;
        let low_target = match words[5] {
            "bot" => {
                let target_index = words[6].parse().map_err(|e| AoCError::BadInputFormat(format!(
                    "Parsing bot index failed. Expected integer, found '{}'. {}", words[1], e)))?;
                Target::Bot(target_index)
            }
            "output" => {
                let target_index = words[6].parse().map_err(|e| AoCError::BadInputFormat(format!(
                    "Parsing output index failed. Expected integer, found '{}'. {}", words[1], e)))?;
                Target::Output(target_index)
            }
            _ => return Err(AoCError::BadInputFormat(format!("Bad bot declaration. Expected 'bot x \
                gives low to [bot|output] y and high to [bot|output] z'. Found '{}'", line)))
        };
        let high_target = match words[10] {
            "bot" => {
                let target_index = words[11].parse().map_err(|e| AoCError::BadInputFormat(format!(
                    "Parsing bot index failed. Expected integer, found '{}'. {}", words[1], e)))?;
                Target::Bot(target_index)
            }
            "output" => {
                let target_index = words[11].parse().map_err(|e| AoCError::BadInputFormat(format!(
                    "Parsing output index failed. Expected integer, found '{}'. {}", words[1], e)))?;
                Target::Output(target_index)
            }
            _ => return Err(AoCError::BadInputFormat(format!("Bad bot declaration. Expected 'bot x \
                gives low to [bot|output] y and high to [bot|output] z'. Found '{}'", line)))
        };
        Ok((Bot::new(low_target, high_target), bot_index))
    }

    pub fn new(low_target: Target, high_target: Target) -> Self {
        let chips = vec![];
        Self{chips, low_target, high_target}
    }

    /// Adds a chip to the bot, returns weather the bot is ready now
    pub fn add_chip(&mut self, chip: usize) -> Result<bool, AoCError<String>> {
        if self.chips.len() == 2 {
            return Err(AoCError::BadInputFormat("Bot received a third chip".to_string()))
        }
        self.chips.push(chip);
        Ok(self.chips.len() >= 2)
    }

    /// Executes the bot
    /// Returns a Transfer for each of the two contained chips
    pub fn execute(&mut self) -> Option<[Transfer; 2]> {
        if self.chips.len() != 2 {
            return None
        }

        let chip = min(self.chips[0], self.chips[1]);
        let target = self.low_target;
        let t0 = Transfer{chip, target};

        let chip = max(self.chips[0], self.chips[1]);
        let target = self.high_target;
        let t1 = Transfer{chip, target};

        self.chips.clear();
        Some([t0, t1])
    }

    pub fn is_ready(&self) -> bool {
        self.chips.len() == 2
    }
}

#[derive(Debug)]
struct Transfer {
    target: Target,
    chip: usize,
}

#[derive(Clone, Copy, Debug)]
enum Target {
    Bot(usize),
    Output(usize),
}

struct Factory {
    bots: Vec<Option<Bot>>,
    bots_ready: Vec<usize>,
    outputs: Vec<Option<usize>>,
}

impl Factory {
    pub fn from_input(input: &[String]) -> Result<Self, AoCError<String>> {
        let mut bots = vec![];
        let mut value_assignments = vec![];

        for line in input.iter() {
            if line.starts_with("value ") {
                value_assignments.push(line);
            } else {
                let (bot, index) = Bot::parse_bot(line)?;
                if bots.len() <= index {
                    bots.resize(index+1, None);
                }
                bots[index] = Some(bot);
            }
        }

        let mut factory = Self{bots, bots_ready: vec![], outputs: vec![]};
        factory.assign_values(&value_assignments)?;
        Ok(factory)
    }

    fn assign_values(&mut self, value_assignments: &[&String]) -> Result<(), AoCError<String>> {
        for line in value_assignments.iter() {
            self.assign_value(line)?;
        }
        Ok(())
    }

    fn assign_value(&mut self, line: &str) -> Result<(), AoCError<String>> {
        let words = line.split(' ').collect::<Vec<_>>();
        if words.len() != 6 ||
            words[0] != "value" ||
            words[2] != "goes" ||
            words[3] != "to" ||
            words[4] != "bot" {
            return Err(AoCError::BadInputFormat(format!("Bad value assignment. Expected 'value x \
                goes to bot y'. Found '{}'", line)));
        }

        let value = words[1].parse()
            .map_err(|e| AoCError::BadInputFormat(format!("Parsing value failed. \
            Expected integer, found '{}'. {}", words[1], e)))?;
        let bot_index = words[5].parse::<usize>()
            .map_err(|e| AoCError::BadInputFormat(format!("Parsing bot index failed. \
            Expected integer, found '{}'. {}", words[1], e)))?;

        if bot_index >= self.bots.len() || self.bots[bot_index].is_none() {
            return Err(AoCError::BadInputFormat(
                format!("Input asks for assignment to non existent bot {}", bot_index)))
        }
        if self.bots[bot_index].as_mut().expect("Was tested to be some")
            .add_chip(value)? {
            self.bots_ready.push(bot_index);
        }

        Ok(())
    }

    /// Executes possible bots until the bot comparing the two values was found
    /*pub fn find_comparator(&mut self, value0: usize, value1: usize)
        -> Result<usize, AoCError<String>> {
        while !self.bots_ready.is_empty() {
            let current_bot = self.bots_ready.pop().expect("bots_ready was tested to \
                not be empty");
            if let Some([t0, t1]) = self.bots[current_bot].as_mut()
                .expect("If a value was given to the bot it has to exist").execute() {
                if (t0.chip == value0 && t1.chip == value1) ||
                    (t0.chip == value1 && t1.chip == value0) {
                    return Ok(current_bot)
                }
                self.transfer(t0)?;
                self.transfer(t1)?;
            }
        }
        Err(AoCError::NoSolutionFoundError(
            format!("No bot comparing values {} and {} was found.", value0, value1)))
    }*/

    pub fn find_comparator(&mut self, value0: usize, value1: usize)
        -> Result<usize, AoCError<String>> {
        while let Some((current_bot, [t0, t1])) = self.execute_step() {
            if (t0.chip == value0 && t1.chip == value1) ||
                (t0.chip == value1 && t1.chip == value0) {
                return Ok(current_bot)
            }
            self.transfer(t0)?;
            self.transfer(t1)?;
        }
        Err(AoCError::NoSolutionFoundError(
            format!("No bot comparing values {} and {} was found.", value0, value1)))
    }

    pub fn calculate_output(&mut self) -> Result<usize, AoCError<String>> {
        while let Some((_, [t0, t1])) = self.execute_step() {
            self.transfer(t0)?;
            self.transfer(t1)?;
        }

        if self.outputs.len() <= 2 ||
            self.outputs[0].is_none() ||
            self.outputs[1].is_none() ||
            self.outputs[2].is_none() {
            return Err(AoCError::NoSolutionFoundError(
                "No solution was found, one of the outputs 0-2 was empty.".to_string()))
        }
        Ok(
            self.outputs[0].expect("output was tested to be some") *
            self.outputs[1].expect("output was tested to be some") *
            self.outputs[2].expect("output was tested to be some")
        )
    }

    fn execute_step(&mut self) -> Option<(usize, [Transfer; 2])> {
        let current_bot = self.get_next_ready()?;
        let transfers = self.bots[current_bot].as_mut()
            .expect("If a value was given to the bot it has to exist").execute()
            .expect("Bot was ready -> has to return some");
        Some((current_bot, transfers))
    }

    fn get_next_ready(&mut self) -> Option<usize> {
        while let Some(next) = self.bots_ready.pop() {
            if self.bots[next].as_mut().expect("bot was added to list -> has to exist")
                .is_ready() {
                return Some(next)
            }
        }
        None
    }

    fn transfer(&mut self, transfer: Transfer) -> Result<(), AoCError<String>> {
        match transfer.target {
            Target::Bot(index) => {
                if index >= self.bots.len() || self.bots[index].is_none() {
                    return Err(AoCError::BadInputFormat(
                        format!("Input asks for transfer to non existent bot {}", index)))
                }
                if self.bots[index].as_mut().expect("Bot was tested to be some")
                    .add_chip(transfer.chip)? {
                    self.bots_ready.push(index)
                }
            }
            Target::Output(index) => {
                if index >= self.outputs.len() {
                    self.outputs.resize(index+1, None);
                }
                if self.outputs[index].is_some() {
                    return Err(AoCError::BadInputFormat(
                        format!("Input asks for same output {} twice", index)))
                }
                self.outputs[index] = Some(transfer.chip);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_output() -> Vec<String> {
        vec![
            "value 5 goes to bot 2".to_string(),
            "bot 2 gives low to bot 1 and high to bot 0".to_string(),
            "value 3 goes to bot 1".to_string(),
            "bot 1 gives low to output 1 and high to bot 0".to_string(),
            "bot 0 gives low to output 2 and high to output 0".to_string(),
            "value 2 goes to bot 2".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() -> Result<(), AoCError<String>> {
        let v = get_output();
        let mut factory = Factory::from_input(&v)?;
        let res = factory.find_comparator(3, 5)?;

        assert_eq!(res, 0);

        Ok(())
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 10)?;
        assert_eq!(part_1(&input), Ok("73".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() -> Result<(), AoCError<String>> {
        let v = get_output();
        let mut factory = Factory::from_input(&v)?;
        let res = factory.calculate_output()?;

        assert_eq!(res, 30);

        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 10)?;
        assert_eq!(part_2(&input), Ok("3965".to_string()));
        Ok(())
    }
}
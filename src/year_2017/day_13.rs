use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let firewall = Firewall::parse(input)?;
    Ok(firewall.get_severity_sum(0).to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let firewall = Firewall::parse(input)?;
    test_delays(&firewall).map(|time| time.to_string())
}

fn test_delays(firewall: &Firewall) -> Result<usize, AoCError<String>> {
    for time in 0.. {
        if !firewall.get_caught(time) {
            return Ok(time)
        }
    }
    Err(AoCError::NoSolutionFoundError("No possible delay to hinder detection found.".to_string()))
}

struct Firewall {
    layers: Vec<Layer>,
}

impl Firewall {
    fn parse(input: &[String]) -> Result<Self, AoCError<String>> {
        let layers = input.iter()
            .map(|line| Layer::parse(line))
            .collect::<Result<_, _>>()?;
        Ok(Self{layers})
    }

    fn get_severity_sum(&self, time: usize) -> usize {
        self.layers.iter().map(|layer| layer.get_severity(time)).sum()
    }

    fn get_caught(&self, time: usize) -> bool {
        for layer in self.layers.iter() {
            if layer.get_caught(time) {
                return true
            }
        }
        false
    }
}

struct Layer {
    index: usize,
    size: usize,
}

impl Layer {
    fn parse(str: &str) -> Result<Self, AoCError<String>> {
        let words = str.split(": ").collect::<Vec<_>>();
        if words.len() != 2 {
            return Err(AoCError::BadInputFormat(
                format!("Expected '<depth>: <range>', found '{}'.", str)))
        }
        let index = words[0].parse()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing depth failed, expected number, found '{}'. {}", words[0], e)))?;
        let size = words[1].parse()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing depth failed, expected number, found '{}'. {}", words[1], e)))?;
        Ok(Self{index, size})
    }

    fn get_caught(&self, start_time: usize) -> bool {
        let arrival_time = start_time + self.index;
        let possible_states = self.size * 2 - 2;
        let current_state = arrival_time % possible_states;
        let current_pos = if current_state >= self.size {
            self.size - (current_state - self.size) - 2
        } else {
            current_state
        };
        current_pos == 0
    }

    fn get_severity(&self, time: usize) -> usize {
        if self.get_caught(time) {
            self.size * self.index
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "0: 3".to_string(),
            "1: 2".to_string(),
            "4: 4".to_string(),
            "6: 4".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let v = get_example_input();
        assert_eq!(part_1(&v), Ok("24".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_13.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("748".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = get_example_input();
        assert_eq!(part_2(&v), Ok("10".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_13.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("3873662".to_string()));
        Ok(())
    }
}
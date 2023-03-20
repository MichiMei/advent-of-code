use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength("Expected exactly one input line containing the \
            number of players".to_string()))
    }
    let count = input[0].parse()
        .map_err(|e| AoCError::BadInputFormat(format!(
            "Parsing number of elves failed. Expected a number, found '{}'. {}", input[0], e)))?;
    Ok(josephus_problem(count).to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength("Expected exactly one input line containing the \
            number of players".to_string()))
    }
    let count = input[0].parse()
        .map_err(|e| AoCError::BadInputFormat(format!(
            "Parsing number of elves failed. Expected a number, found '{}'. {}", input[0], e)))?;
    let mut circle = Circle::new(count);
    Ok(circle.solve().to_string())
}

fn josephus_problem(count: usize) -> usize {
    let next_smaller_power = 1 << (usize::BITS-count.leading_zeros()-1);
    2 * (count-next_smaller_power) + 1
}

struct Circle {
    participants: Vec<usize>,
    count: usize,
    pre_target: usize,
}

impl Circle {
    pub fn new(count: usize) -> Self {
        let mut participants = (1..count).collect::<Vec<_>>();
        participants.push(0);
        let pre_target = count/2-1;
        Self{participants, count, pre_target}
    }

    pub fn solve(&mut self) -> usize {
        while self.count > 1 {
            self.step();
        }
        self.pre_target+1
    }

    fn step(&mut self) {
        if self.count == 1 {
            return
        }
        let curr_next = self.participants[self.pre_target];
        let new_next = self.participants[curr_next];
        self.participants[self.pre_target] = new_next;
        if self.count%2 == 1 {
            self.pre_target = new_next;
        }
        self.count -= 1;
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["5".to_string()]), Ok("3".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 19)?;
        assert_eq!(part_1(&input), Ok("1842613".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["5".to_string()]), Ok("2".to_string()));
        assert_eq!(part_2(&vec!["13".to_string()]), Ok("4".to_string()));
        assert_eq!(part_2(&vec!["14".to_string()]), Ok("5".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 19)?;
        assert_eq!(part_2(&input), Ok("1424135".to_string()));
        Ok(())
    }
}
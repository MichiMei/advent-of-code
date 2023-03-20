use std::collections::VecDeque;
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the step size.".to_string()))
    }
    let step_size = input[0].parse()
        .map_err(|e| AoCError::BadInputFormat(
            format!("Parsing step size failed. Expected number, found '{}'. {}", &input[0], e)))?;
    let mut ring_buffer = RingBuffer::new(step_size);
    while ring_buffer.len() < 2018 {
        ring_buffer.add();
    }
    Ok(ring_buffer.get_next().to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the step size.".to_string()))
    }
    let step_size = input[0].parse()
        .map_err(|e| AoCError::BadInputFormat(
            format!("Parsing step size failed. Expected number, found '{}'. {}", &input[0], e)))?;
    let mut ring_buffer = SimulatedRingBuffer::new(step_size);
    for _ in 0..50_000_000 {
        ring_buffer.add();
    }
    Ok(ring_buffer.get_second()
        .map(|elem| elem.to_string())
        .expect("Index < len -> element has to exist"))
}

struct RingBuffer {
    data: VecDeque<usize>,
    current_pos: usize,
    steps: usize,
}

impl RingBuffer {
    fn new(steps: usize) -> Self {
        let data = VecDeque::from([0]);
        let current_pos = 0;
        Self {data, current_pos, steps}
    }

    fn add(&mut self) {
        self.current_pos = (self.current_pos+self.steps+1)%self.data.len();
        self.data.insert(self.current_pos, self.data.len());
    }

    fn get_next(&self) -> usize {
        let next_pos = (self.current_pos+1)%self.data.len();
        *self.data.get(next_pos).expect("next_pos < len")
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

struct SimulatedRingBuffer {
    second_element: Option<usize>,
    current_pos: usize,
    steps: usize,
    len: usize,
}

impl SimulatedRingBuffer {
    fn new(steps: usize) -> Self {
        let second_element = None;
        let current_pos = 0;
        let len = 1;
        Self {second_element, current_pos, steps, len}
    }

    fn add(&mut self) {
        self.current_pos = (self.current_pos+self.steps+1)%self.len();
        if self.current_pos == 0 {
            self.second_element = Some(self.len());
        }
        self.len += 1;
    }

    fn get_second(&self) -> Option<usize> {
        self.second_element
    }

    fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["3".to_string()]), Ok("638".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 17)?;
        assert_eq!(part_1(&input), Ok("777".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 17)?;
        assert_eq!(part_2(&input), Ok("39289581".to_string()));
        Ok(())
    }
}
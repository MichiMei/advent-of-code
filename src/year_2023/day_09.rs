use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    input.iter()
        .map(|line| Extrapolator::parse(line)
            .map(|mut ep| ep.extrapolate()))
        .sum::<Result<i32, _>>()
        .map(|v| v.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    input.iter()
        .map(|line| Extrapolator::parse(line)
            .map(|mut ep| ep.extrapolate_backwards()))
        .sum::<Result<i32, _>>()
        .map(|v| v.to_string())
}

struct Extrapolator {
    sequences: Vec<Vec<i32>>,
}

impl Extrapolator {
    fn parse(line: &str) -> Result<Self, AoCError<String>> {
        let sequence = line.split_whitespace()
            .map(|str| str.parse()
                .map_err(|e| AoCError::BadInputFormat(
                    format!("Parsing number '{}' failed. {}", str, e))))
            .collect::<Result<Vec<_>, _>>()?;
        let sequences = vec![sequence];
        Ok(Self {
            sequences,
        })
    }

    fn extrapolate(&mut self) -> i32 {
        while self.step_down() {}
        self.step_up()
    }

    fn step_down(&mut self) -> bool {
        let current = if let Some(row) = self.sequences.last() {
            row
        } else {
            return false
        };
        if current.iter().filter(|val| **val != 0).count() == 0 {
            return false
        }
        let next = current.windows(2)
            .map(|window| window[1]-window[0])
            .collect::<Vec<_>>();
        assert_eq!(current.len()-1, next.len());
        self.sequences.push(next);
        true
    }

    fn step_up(&mut self) -> i32 {
        self.sequences.last_mut().expect("sequences can't be empty").push(0);
        for index in (0..self.sequences.len()-1).rev() {
            let extrapolated =
                self.sequences[index].last().expect("sequence can't be empty") +
                self.sequences[index+1].last().expect("sequence can't be empty");
            self.sequences[index].push(extrapolated);
        }
        *self.sequences[0].last().expect("sequences can't be empty")
    }

    fn extrapolate_backwards(&mut self) -> i32 {
        while self.step_down() {}
        self.step_up_backwards()
    }

    fn step_up_backwards(&mut self) -> i32 {
        self.sequences.last_mut().expect("sequences can't be empty").push(0);
        for index in (0..self.sequences.len()-1).rev() {
            let extrapolated =
                self.sequences[index][0] -
                    self.sequences[index+1][0];
            self.sequences[index].insert(0, extrapolated);
        }
        self.sequences[0][0]
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("114".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 9)?;
        assert_eq!(part_1(&input), Ok("1647269739".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("2".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 9)?;
        assert_eq!(part_2(&input), Ok("864".to_string()));
        Ok(())
    }
}
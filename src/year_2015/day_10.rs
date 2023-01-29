use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Input is expected to be exactly 1 line, found {}", input.len())
        ))
    }
    let int = input.first().unwrap().parse().map_err(|e| AoCError::BadInputFormat(
        format!("Could not parse input, expected a positive number, found {}\n{}", input[0], e)
    ))?;
    let mut start = LASSequence::from(int);

    for _ in 0..40 {
        start = start.next()
    }

    Ok(start.len().to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Input is expected to be exactly 1 line, found {}", input.len())
        ))
    }
    let int = input.first().unwrap().parse().map_err(|e| AoCError::BadInputFormat(
        format!("Could not parse input, expected a positive number, found {}\n{}", input[0], e)
    ))?;
    let mut start = LASSequence::from(int);

    for _ in 0..50 {
        start = start.next()
    }

    Ok(start.len().to_string())
}

#[derive(Eq, PartialEq, Debug)]
struct LASSequence {
    sequence: Vec<u8>,
}

impl LASSequence {
    pub fn from(mut input: u32) -> Self {
        assert!(input > 0);
        let mut sequence = vec![];
        while input > 0 {
            sequence.push((input%10) as u8);
            input /= 10;
        }
        sequence.reverse();
        Self{sequence}
    }

    pub fn next(self) -> Self {
        let mut res = Self::new();
        let mut iter = self.sequence.into_iter();
        let mut count = 1;
        let mut prev = iter.next().unwrap();
        for val in iter {
            if prev == val {
                count += 1;
            } else {
                res.push(count, prev);
                prev = val;
                count = 1;
            }
        }
        res.push(count, prev);

        res
    }

    pub fn len(&self) -> usize {
        self.sequence.len()
    }

    fn new() -> Self {
        Self{sequence: vec![]}
    }

    fn push(&mut self, mut count: usize, digit: u8) {
        assert!(count > 0);
        let length = self.sequence.len();
        while count > 0 {
            self.sequence.push((count%10) as u8);
            count /= 10;
        }
        self.sequence[length..].reverse();
        self.sequence.push(digit);
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_10.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("360154".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_10.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("5103798".to_string()));
        Ok(())
    }

    #[test]
    fn check_sequence_next() {
        let sequence = LASSequence::from(1);
        assert_eq!(sequence.next(), LASSequence::from(11));

        let sequence = LASSequence::from(11);
        assert_eq!(sequence.next(), LASSequence::from(21));

        let sequence = LASSequence::from(21);
        assert_eq!(sequence.next(), LASSequence::from(1211));

        let sequence = LASSequence::from(1211);
        assert_eq!(sequence.next(), LASSequence::from(111221));

        let sequence = LASSequence::from(111221);
        assert_eq!(sequence.next(), LASSequence::from(312211));
    }
}
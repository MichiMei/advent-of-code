use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected exactly one line containing the starting curve".to_string()))
    }
    let length = 272;
    let mut dragon_curve = DragonCurve::parse(&input[0])?;
    while dragon_curve.data.len() < length {
        dragon_curve.step();
    }

    Ok(dragon_curve.get_checksum(length)
        .expect("Dragon curve was prolonged in while to be long enough"))
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected exactly one line containing the starting curve".to_string()))
    }
    let length = 35651584;
    let mut dragon_curve = DragonCurve::parse(&input[0])?;
    while dragon_curve.data.len() < length {
        dragon_curve.step();
    }

    Ok(dragon_curve.get_checksum(length)
        .expect("Dragon curve was prolonged in while to be long enough"))
}

#[derive(Clone)]
struct DragonCurve {
    data: Vec<bool>,
}

impl DragonCurve {
    pub fn parse(str: &str) -> Result<Self, AoCError<String>> {
        let mut data = vec![];
        for char in str.chars() {
            match char {
                '0' => data.push(false),
                '1' => data.push(true),
                c => return Err(AoCError::BadInputFormat(
                    format!("Only characters '0' and '1' allowed. Found '{}'", c)))
            }
        }
        Ok(Self{data})
    }

    pub fn step(&mut self) -> usize {
        let mut opposite = self.clone();
        opposite.reverse();
        opposite.negate();
        self.data.push(false);
        self.data.extend(opposite.data);
        self.data.len()
    }

    pub fn get_checksum(&self, length: usize) -> Option<String> {
        if length > self.data.len() {
            return None
        }
        let data = self.data[0..length].to_vec();
        Some(Self{data}.get_checksum_unchecked())
    }

    fn get_checksum_unchecked(&self) -> String {
        if self.data.len()%2 == 1 {
            return self.to_string()
        }
        let mut checksum = vec![];
        for pair in self.data.chunks_exact(2) {
            assert_eq!(pair.len(), 2);
            if pair[0] == pair[1] {
                checksum.push(true)
            } else {
                checksum.push(false)
            }
        }
        Self{data: checksum}.get_checksum_unchecked()
    }

    fn reverse(&mut self) {
        self.data.reverse();
    }

    fn negate(&mut self) {
        for elem in self.data.iter_mut() {
            *elem = !*elem;
        }
    }
}

impl Display for DragonCurve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for elem in self.data.iter() {
            if *elem {
                str = format!("{}1", str)
            } else {
                str = format!("{}0", str)
            }
        }
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_step() {
        let mut dc = DragonCurve::parse("1").unwrap();
        dc.step();
        assert_eq!(dc.to_string(), "100".to_string());
        let mut dc = DragonCurve::parse("0").unwrap();
        dc.step();
        assert_eq!(dc.to_string(), "001".to_string());
        let mut dc = DragonCurve::parse("11111").unwrap();
        dc.step();
        assert_eq!(dc.to_string(),
                   "11111000000".to_string());
        let mut dc = DragonCurve::parse("111100001010").unwrap();
        dc.step();
        assert_eq!(dc.to_string(),
                   "1111000010100101011110000".to_string());
    }

    #[test]
    fn check_checksum() {
        assert_eq!(DragonCurve::parse("110010110100").unwrap().get_checksum(12).unwrap(),
                   "100".to_string());
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 16)?;
        assert_eq!(part_1(&input), Ok("10011010010010010".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 16)?;
        assert_eq!(part_2(&input), Ok("10101011110100011".to_string()));
        Ok(())
    }
}
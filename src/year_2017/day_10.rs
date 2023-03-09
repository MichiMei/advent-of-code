use std::fmt::{Display, Formatter};
use std::ops::BitXor;
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing a list of hash lengths.".to_string()))
    }
    let numbers = parse_numbers(&input[0])?;
    let mut knot_hash = KnotHash::new(255);
    execute_hash_list(&mut knot_hash, &numbers).map(|res| res.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing a list of hash lengths.".to_string()))
    }
    Ok(complete_hash(&input[0]))
}

fn parse_numbers(line: &str) -> Result<Vec<usize>, AoCError<String>> {
    line.split(',')
        .map(|word| word.parse()
        .map_err(|e| AoCError::BadInputFormat(format!(
            "Parsing input numbers failed. {}", e))))
        .collect()
}

fn complete_hash(line: &str) -> String{
    let mut knot_hash = KnotHash::new(255);
    let hash_length_list = str_to_hash_length_list(line);
    for _ in 0..64 {
        _ = execute_hash_list(&mut knot_hash, &hash_length_list);
    }
    knot_hash.get_dense_hash()
}

fn str_to_hash_length_list(str: &str) -> Vec<usize> {
    let mut res = str.chars().map(|c| c as u8 as usize).collect::<Vec<_>>();
    res.extend([17, 31, 73, 47, 23]);
    res
}

fn execute_hash_list(knot_hash: &mut KnotHash, numbers: &Vec<usize>) -> Result<usize, AoCError<String>> {
    for number in numbers {
        knot_hash.hash(*number);
    }
    knot_hash.get_start_product()
        .ok_or_else(|| AoCError::NoSolutionFoundError("Hash-Length < 2".to_string()))
}

struct KnotHash {
    numbers: Vec<u8>,
    position: usize,
    skip_size: usize,
}

impl KnotHash {
    fn new(len: u8) -> Self {
        let numbers = (0u8..=len).collect();
        let position = 0;
        let skip_size = 0;
        Self{numbers, position, skip_size}
    }

    fn hash(&mut self, len: usize) {
        let normalized = self.numbers[self.position..].iter().copied().chain(self.numbers[0..self.position].iter().copied()).collect::<Vec<_>>();
        let mut rotated = normalized[0..len].to_vec();
        rotated.reverse();
        rotated.extend(&normalized[len..]);
        self.numbers = rotated[rotated.len()-self.position..].iter().copied().chain(rotated[0..rotated.len()-self.position].iter().copied()).collect();
        self.position = (self.position+len+self.skip_size)%self.numbers.len();
        self.skip_size += 1;
    }

    fn get_start_product(&self) -> Option<usize> {
        if self.numbers.len() < 2 {
            return None
        }
        Some(self.numbers[0] as usize * self.numbers[1] as usize)
    }

    fn get_dense_hash(&self) -> String {
        self.numbers
            .chunks_exact(16)
            .map(|bytes| bytes[1..]
                .iter().
                fold(bytes[0], |acc, b| acc.bitxor(b)))
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}

impl Display for KnotHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = format!("pos({}) skip_size({}) ", self.position, self.skip_size);
        for elem in self.numbers.iter() {
            str = format!("{}{} ", str, elem);
        }
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let mut knot_hash = KnotHash::new(4);
        assert_eq!(execute_hash_list(&mut knot_hash, &vec![3, 4, 1, 5]), Ok(12));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_10.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("13760".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["".to_string()]),
                   Ok("a2582a3a0e66e6e86e3812dcb672a272".to_string()));
        assert_eq!(part_2(&vec!["AoC 2017".to_string()]),
                   Ok("33efeb34ea91902bb2f59c9920caa6cd".to_string()));
        assert_eq!(part_2(&vec!["1,2,3".to_string()]),
                   Ok("3efbe78a8d82f29979031a4aa0b16a9d".to_string()));
        assert_eq!(part_2(&vec!["1,2,4".to_string()]),
                   Ok("63960835bcdc130f0b66d7ff4f6a5a8e".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_10.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("2da93395f1a6bb3472203252e3b17fe5".to_string()));
        Ok(())
    }
}
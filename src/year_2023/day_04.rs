use std::cmp::min;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let cards = input.iter()
        .map(|line| Card::parse(line)).collect::<Result<Vec<_>, _>>()?;
    Ok(cards.iter().map(|card| card.calc_score()).sum::<u32>().to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut cards = input.iter()
        .map(|line| Card::parse(line)).collect::<Result<Vec<_>, _>>()?;
    for index in 0..cards.len() {
        let hit = cards[index].calc_hit_numbers();
        let copies = cards[index].copies;
        for index in (index+1)..min(index+hit+1, cards.len()) {
            cards[index].add_copies(copies)
        }
    }
    Ok(cards.iter().map(|card| card.copies).sum::<usize>().to_string())
}

struct Card {
    id: usize,
    copies: usize,
    winning: HashSet<u32>,
    present: Vec<u32>,
}

impl Card {
    fn parse(line: &str) -> Result<Self, AoCError<String>> {
        let split = line.split(": ").collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(AoCError::BadInputFormat(
                format!("Expected 'Card <id>: <list of numbers> | <list of numbers>'. \
                Found '{}'", line)));
        }
        if !split[0].starts_with("Card ") {
            return Err(AoCError::BadInputFormat(
                format!("Expected 'Card <id>: <list of numbers> | <list of numbers>'. \
                Found '{}'", line)));
        }
        let id = split[0][5..].trim().parse::<usize>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Paring card id failed. Line: '{}'; {}", line, e)))?;
        let split = split[1].split(" | ").collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(AoCError::BadInputFormat(
                format!("Expected 'Card <id>: <list of numbers> | <list of numbers>'. \
                Found '{}'", line)));
        }
        let winning = Self::parse_num_list_as_set(split[0])?;
        let present = Self::parse_num_list_as_vec(split[1])?;
        Ok(Self {
            id,
            copies: 1,
            winning,
            present,
        })
    }

    fn parse_num_list_as_vec(str: &str) -> Result<Vec<u32>, AoCError<String>> {
        str.split_whitespace()
            .map(|num| num.parse()).collect::<Result<Vec<_>, _>>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing number failed. {}", e)))
    }

    fn parse_num_list_as_set(str: &str) -> Result<HashSet<u32>, AoCError<String>> {
        str.split_whitespace()
            .map(|num| num.parse()).collect::<Result<HashSet<_>, _>>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing number failed. {}", e)))
    }

    fn calc_hit_numbers(&self) -> usize {
        self.present.iter()
            .filter(|present| self.winning.contains(present))
            .count()
    }

    fn calc_score(&self) -> u32 {
        let hit = self.calc_hit_numbers() as u32;
        if hit == 0 {
            return 0
        }
        2u32.pow(hit-1)
    }

    fn add_copies(&mut self, amount: usize) {
        self.copies += amount
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {}: {:?} | {:?}", self.id, self.winning, self.present)
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("13".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 4)?;
        assert_eq!(part_1(&input), Ok("17803".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("30".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 4)?;
        assert_eq!(part_2(&input), Ok("5554894".to_string()));
        Ok(())
    }
}
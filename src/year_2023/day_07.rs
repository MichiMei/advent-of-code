use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut hands = input.iter()
        .map(|line| Hand::parse(line, false))
        .collect::<Result<Vec<_>, _>>()?;
    hands.sort();
    Ok(hands.iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum::<usize>()
        .to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut hands = input.iter()
        .map(|line| Hand::parse(line, true))
        .collect::<Result<Vec<_>, _>>()?;
    hands.sort();
    Ok(hands.iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum::<usize>()
        .to_string())
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    hand_type: Type,
}

impl Hand {
    fn parse(line: &str, j_is_joker: bool) -> Result<Self, AoCError<String>> {
        let split = line.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            todo!()
        }
        if split[0].len() != 5 {
            todo!()
        }
        let cards = split[0].chars()
            .map(|c| Card::parse(c, j_is_joker))
            .collect::<Result<Vec<_>, _>>()?;
        let bid = split[1].parse()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing bid failed. {}", e)))?;
        let hand_type = Type::get_from_cards(cards.clone());
        Ok(Self {
            cards,
            bid,
            hand_type,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type)
        }
        for (this, other) in self.cards.iter().zip(other.cards.iter()) {
            if this != other {
                return this.cmp(other)
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}{}{}{}{} {}",
               self.hand_type,
               self.cards[0],
               self.cards[1],
               self.cards[2],
               self.cards[3],
               self.cards[4],
               self.bid)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    C10,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    Joker,
}

impl Card {
    fn parse(c: char, j_is_joker: bool) -> Result<Self, AoCError<String>> {
        Ok(match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => {
                if j_is_joker {
                    Self::Joker
                } else {
                    Self::J
                }
            },
            'T' => Self::C10,
            '9' => Self::C9,
            '8' => Self::C8,
            '7' => Self::C7,
            '6' => Self::C6,
            '5' => Self::C5,
            '4' => Self::C4,
            '3' => Self::C3,
            '2' => Self::C2,
            _ => return Err(AoCError::BadInputFormat(format!("Unexpected card: {}", c))),
        })
    }

    fn get_value(&self) -> u8 {
        match self {
            Card::A => 13,
            Card::K => 12,
            Card::Q => 11,
            Card::J => 10,
            Card::C10 => 9,
            Card::C9 => 8,
            Card::C8 => 7,
            Card::C7 => 6,
            Card::C6 => 5,
            Card::C5 => 4,
            Card::C4 => 3,
            Card::C3 => 2,
            Card::C2 => 1,
            Card::Joker => 0,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::A => write!(f, "A"),
            Card::K => write!(f, "K"),
            Card::Q => write!(f, "Q"),
            Card::J => write!(f, "J"),
            Card::C10 => write!(f, "T"),
            Card::C9 => write!(f, "9"),
            Card::C8 => write!(f, "8"),
            Card::C7 => write!(f, "7"),
            Card::C6 => write!(f, "6"),
            Card::C5 => write!(f, "5"),
            Card::C4 => write!(f, "4"),
            Card::C3 => write!(f, "3"),
            Card::C2 => write!(f, "2"),
            Card::Joker => write!(f, "j"),
        }
    }
}

#[derive(Eq, PartialEq)]
enum Type {
    Kind5,
    Kind4,
    FullHouse,
    Kind3,
    Pair2,
    Pair1,
    HighCard,
}

impl Type {
    fn get_value(&self) -> u8 {
        match self {
            Type::Kind5 => 27,
            Type::Kind4 => 26,
            Type::FullHouse => 25,
            Type::Kind3 => 24,
            Type::Pair2 => 23,
            Type::Pair1 => 22,
            Type::HighCard => 1,
        }
    }

    fn get_from_cards(mut cards: Vec<Card>) -> Self {
        cards.sort();
        let mut iter = cards.iter();
        let mut prev = iter.next().expect("Hand has to contain 5 cards");
        let (mut counts, mut joker_count) = if let Card::Joker = *prev {
            (vec![0], 1)
        } else {
            (vec![1], 0)
        };
        for card in iter {
            if let Card::Joker = *card {
                joker_count += 1;
            } else if card == prev {
                let elem = counts.last_mut().expect("Is created from element");
                *elem += 1;
            } else {
                prev = card;
                counts.push(1)
            }
        }
        println!("{:?}\t{}", counts, joker_count);
        counts.sort();
        counts.reverse();
        counts[0] += joker_count;
        println!("{:?}", counts);
        match counts[0] {
            5 => Self::Kind5,
            4 => Self::Kind4,
            3 => {
                if counts[1] == 2 {
                    Self::FullHouse
                } else {
                    Self::Kind3
                }
            }
            2 => {
                if counts[1] == 2 {
                    Self::Pair2
                } else {
                    Self::Pair1
                }
            }
            1 => Self::HighCard,
            _ => panic!("Counts are only elements of [1..=5]"),
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Kind5 => write!(f, "5 of a kind"),
            Type::Kind4 => write!(f, "4 of a kind"),
            Type::FullHouse => write!(f, "full house "),
            Type::Kind3 => write!(f, "3 of a kind"),
            Type::Pair2 => write!(f, "2 pairs    "),
            Type::Pair1 => write!(f, "1 pair     "),
            Type::HighCard=> write!(f, "high card  "),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("6440".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 7)?;
        assert_eq!(part_1(&input), Ok("251806792".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("5905".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 7)?;
        assert_eq!(part_2(&input), Ok("252113488".to_string()));
        Ok(())
    }
}
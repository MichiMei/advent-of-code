use std::cmp::max;
use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let contained = Subset{
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut sum = 0;
    for line in input {
        let game = Game::parse(line)?;
        if game.possible(&contained) {
            sum += game.id;
        }
    }
    Ok(sum.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let mut sum = 0;
    for line in input {
        let game = Game::parse(line)?;
        sum += game.get_power();
    }
    Ok(sum.to_string())
}

enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn parse(str: &str) -> Result<Self, AoCError<String>> {
        match str {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(AoCError::BadInputFormat(
                format!("Expected 'red', 'green' or 'blue'. Found: {}", str)))
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Blue => write!(f, "blue"),
        }
    }
}

struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}

impl Subset {
    fn parse(str: &str) -> Result<Self, AoCError<String>> {
        let split = str.split(", ");
        let mut res = Self{
            red: 0,
            green: 0,
            blue: 0,
        };
        for elem in split {
            let cube_count = Self::parse_cube_count(elem)?;
            match cube_count.0 {
                Color::Red => {
                    if res.red != 0 {
                        return Err(AoCError::BadInputFormat(
                            format!("Subset contains multiple {} values", cube_count.0)));
                    } else {
                        res.red = cube_count.1;
                    }
                }
                Color::Green => {
                    if res.green != 0 {
                        return Err(AoCError::BadInputFormat(
                            format!("Subset contains multiple {} values", cube_count.0)));
                    } else {
                        res.green = cube_count.1;
                    }
                }
                Color::Blue => {
                    if res.blue != 0 {
                        return Err(AoCError::BadInputFormat(
                            format!("Subset contains multiple {} values", cube_count.0)));
                    } else {
                        res.blue = cube_count.1;
                    }
                }
            }
        }
        Ok(res)
    }

    fn parse_cube_count(str: &str) -> Result<(Color, u32), AoCError<String>> {
        let mut split = str.split(' ');
        let amount = split.next()
            .ok_or(AoCError::BadInputFormat(
                format!("Cube Count malformed, expected '<color>: <amount>', found: {}", str)))?
            .parse()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing amount failed. {}", e)))?;
        let color = Color::parse(
            split.next().ok_or(AoCError::BadInputFormat(
                format!("Cube Count malformed, expected '<color>: <amount>', found: {}", str)))?)?;
        if let Some(remainder) = split.next() {
            return Err(AoCError::BadInputFormat(
                format!("Expected Cube Count to end, found '{}'", remainder)))
        }
        Ok((color, amount))
    }

    fn possible(&self, contained: &Self) -> bool {
        if self.red > contained.red {
            return false
        }
        if self.green > contained.green {
            return false
        }
        if self.blue > contained.blue {
            return false
        }
        true
    }
}

struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

impl Game {
    fn parse(str: &str) -> Result<Self, AoCError<String>> {
        let split = str.split(": ").collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(AoCError::BadInputFormat(
                format!("Bad Game format, expected 'Game <id>: \
                <semicolon separated list of subsets>', found '{}'", str)));
        }
        if !split[0].starts_with("Game ") {
            return Err(AoCError::BadInputFormat(
                format!("Bad Game format, expected 'Game <id>: \
                <semicolon separated list of subsets>', found '{}'", str)));
        }
        let id = split[0][5..].parse().map_err(|e| AoCError::BadInputFormat(
            format!("Parsing Game id failed. {}", e)))?;
        let split = split[1].split("; ");
        let mut subsets = vec![];
        for elem in split {
            subsets.push(Subset::parse(elem)?)
        }
        Ok(Self {
            id,
            subsets,
        })
    }

    fn possible(&self, contained: &Subset) -> bool {
        for subset in &self.subsets {
            if !subset.possible(contained) {
                return false
            }
        }
        true
    }

    fn get_power(&self) -> u32 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for subset in &self.subsets {
            red = max(red, subset.red);
            green = max(green, subset.green);
            blue = max(blue, subset.blue);
        }
        red * green * blue
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&get_example_input()), Ok("8".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 2)?;
        assert_eq!(part_1(&input), Ok("2528".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&get_example_input()), Ok("2286".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 2)?;
        assert_eq!(part_2(&input), Ok("67363".to_string()));
        Ok(())
    }
}
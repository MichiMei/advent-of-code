use std::cmp::min;
use std::str::Chars;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut res = String::new();
    let mut start_button = '5';
    for line in input {
        let dir_iter = DirectionIterator::new(line);
        let mut keypad = SimpleKeypad::from_char(start_button)
            .expect("should not fail if given a digit in 1..=9");
        dir_iter.calculate_end_button(&mut keypad)?;
        start_button = keypad.get_digit();
        res = format!("{}{}", res, start_button);
    }

    Ok(res)
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut res = String::new();
    let mut x_pos = 0;
    let mut y_pos = 2;
    for line in input {
        let dir_iter = DirectionIterator::new(line);
        let mut keypad = ComplexKeypad::from_pos(x_pos, y_pos)
            .expect("should not fail if given a digit in 1..=9");
        dir_iter.calculate_end_button(&mut keypad)?;
        x_pos = keypad.x_pos;
        y_pos = keypad.y_pos;
        res = format!("{}{}", res, keypad.get_digit());
    }

    Ok(res)
}

struct DirectionIterator<'a> {
    chars_iter: Chars<'a>,
}

impl<'a> DirectionIterator<'a> {
    fn new(line: &'a str) -> Self {
        let chars_iter = line.chars();
        Self {chars_iter}
    }
}

impl <'a> DirectionIterator<'a> {
    fn calculate_end_button<K: Keypad>(self, keypad: &mut K) -> Result<(), AoCError<String>> {
        for dir_res in self {
            let dir = dir_res?;
            keypad.move_to(&dir);
        }
        Ok(())
    }
}

impl<'a> Iterator for DirectionIterator<'a> {
    type Item = Result<Direction, AoCError<String>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars_iter.next().map(|char| {
            Direction::from_char(char)
        })
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Result<Self, AoCError<String>> {
        Ok(match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            x => {
                let message = format!("Input contains unsupported character ({}). \
                    Only U, D, L and R are supported", x);
                return Err(AoCError::BadInputFormat(message))
            },
        })
    }
}

trait Keypad {
    fn move_to(&mut self, dir: &Direction);
    fn get_digit(&self) -> char;
}

struct SimpleKeypad {
    x_pos: usize,
    y_pos: usize,
}

impl SimpleKeypad {
    fn from_u8(digit: u8) -> Option<Self> {
        if !(1..=9).contains(&digit) {
            return None
        }
        let y_pos = ((digit-1)/3) as usize;
        let x_pos = ((digit-1)%3) as usize;

        Some(Self{x_pos, y_pos})
    }

    fn from_char(key: char) -> Option<Self> {
        let u8 = format!("{}", key).parse().ok()?;
        Self::from_u8(u8)
    }
}

impl Keypad for SimpleKeypad {
    fn move_to(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => {
                if let Some(new) = self.y_pos.checked_sub(1) {
                    self.y_pos = new;
                }
            }
            Direction::Down => self.y_pos = min(self.y_pos+1 ,2),
            Direction::Left => {
                if let Some(new) = self.x_pos.checked_sub(1) {
                    self.x_pos = new;
                }
            }
            Direction::Right => self.x_pos = min(self.x_pos+1 ,2),
        }
    }

    fn get_digit(&self) -> char {
        let u8_digit = self.y_pos*3 + self.x_pos + 1;
        assert!((1..=9).contains(&u8_digit));

        let str = format!("{}", u8_digit);
        assert_eq!(str.len(), 1);

        str.chars().next().expect("str had length 1 -> must have a char")
    }
}

struct ComplexKeypad {
    array: [[Option<char>; 5]; 5],
    x_pos: usize,
    y_pos: usize,
}

impl ComplexKeypad {
    fn from_pos(x_pos: usize, y_pos: usize) -> Option<Self> {
        let array = [
            [None     , None     , Some('1'), None     , None     ],
            [None     , Some('2'), Some('3'), Some('4'), None     ],
            [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
            [None     , Some('A'), Some('B'), Some('C'), None     ],
            [None     , None     , Some('D'), None     , None     ],
        ];
        if x_pos >= array[0].len() || y_pos >= array.len() {
            return None
        }
        Some(Self{array, x_pos, y_pos})
    }
}

impl Keypad for ComplexKeypad {
    fn move_to(&mut self, dir: &Direction) {
        let (new_x, new_y) = match dir {
            Direction::Up => {
                if let Some(new) = self.y_pos.checked_sub(1) {
                    (self.x_pos, new)
                } else {
                    (self.x_pos, self.y_pos)
                }
            }
            Direction::Down => (self.x_pos, min(self.y_pos+1 ,4)),
            Direction::Left => {
                if let Some(new) = self.x_pos.checked_sub(1) {
                    (new, self.y_pos)
                } else {
                    (self.x_pos, self.y_pos)
                }
            }
            Direction::Right => (min(self.x_pos+1 ,4), self.y_pos),
        };

        if self.array[new_y][new_x].is_some() {
            self.x_pos = new_x;
            self.y_pos = new_y;
        }
    }

    fn get_digit(&self) -> char {
        self.array[self.y_pos][self.x_pos].expect("position should never land on None")
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "ULL".to_string(),
            "RRDDD".to_string(),
            "LURDL".to_string(),
            "UUUUD".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let v = get_example_input();

        assert_eq!(part_1(&v), Ok("1985".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_02.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("47978".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = get_example_input();

        assert_eq!(part_2(&v), Ok("5DB3".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_02.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("659AD".to_string()));
        Ok(())
    }
}
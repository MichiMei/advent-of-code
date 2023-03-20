use std::str::Chars;
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected only a single line containing the stream.".to_string()))
    }
    let mut parser = StreamParser::new(&input[0]);
    parser.parse().map(|(score, _)| score.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected only a single line containing the stream.".to_string()))
    }
    let mut parser = StreamParser::new(&input[0]);
    parser.parse().map(|(_, score)| score.to_string())
}

struct StreamParser<'a> {
    stream: Chars<'a>,
    state: State,
    escape: bool,
    group_count: usize,
    garbage_count: usize,
}

impl<'a> StreamParser<'a> {
    fn new(str: &'a str) -> Self {
        let stream = str.chars();
        let state = State::new();
        let escape = false;
        let group_count = 0;
        let garbage_count = 0;
        Self{stream, state, escape, group_count, garbage_count}
    }

    fn parse(&mut self) -> Result<(usize, usize), AoCError<String>> {
        while self.next()? {}
        Ok((self.group_count, self.garbage_count))
    }

    fn next(&mut self) -> Result<bool, AoCError<String>> {
        if let Some(char) = self.stream.next() {
            if self.escape {
                self.escape = false;
                return Ok(true)
            }
            match char {
                '{' => {
                    if self.state.is_garbage() {
                        self.garbage_count += 1;
                    }
                    self.state = self.state.open_group()
                },
                '}' => {
                    if self.state.is_garbage() {
                        self.garbage_count += 1;
                    }
                    if let Some(score) = self.state.is_group() {
                        self.state = self.state.close_group();
                        self.group_count += score;
                    }
                }
                '<' => {
                    if self.state.is_garbage() {
                        self.garbage_count += 1;
                    }
                    self.state = self.state.start_garbage()
                },
                '>' => self.state = self.state.end_garbage()?,
                '!' => {
                    if self.state.is_garbage() {
                        self.escape = true;
                    }
                }
                ',' => {
                    if self.state.is_garbage() {
                        self.garbage_count += 1;
                    }
                }
                c => {
                    if self.state.is_garbage() {
                        self.garbage_count += 1;
                    }
                    if !self.state.is_garbage() {
                        return Err(AoCError::BadInputFormat(
                            format!("Found unsupported char '{}' outside of garbage.", c)))
                    }
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Copy, Clone)]
enum State {
    Group(usize),
    Garbage(usize),
}

impl State {
    fn new() -> Self {
        Self::Group(0)
    }

    fn open_group(self) -> Self {
        match self {
            State::Group(x) => State::Group(x+1),
            State::Garbage(_) => self
        }
    }

    fn close_group(self) -> Self {
        match self {
            State::Group(x) => State::Group(x-1),
            State::Garbage(_) => self
        }
    }

    fn start_garbage(self) -> Self {
        match self {
            State::Group(x) => State::Garbage(x),
            State::Garbage(_) => self
        }
    }

    fn end_garbage(self) -> Result<Self, AoCError<String>> {
        match self {
            State::Group(_) => Err(AoCError::BadInputFormat(
                "Found unsupported char '>' outside of garbage.".to_string())),
            State::Garbage(x) => Ok(Self::Group(x)),
        }
    }

    fn is_garbage(&self) -> bool {
        match self {
            State::Group(_) => false,
            State::Garbage(_) => true,
        }
    }

    fn is_group(&self) -> Option<usize> {
        match self {
            State::Group(x) => Some(*x),
            State::Garbage(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["{}".to_string()]), Ok("1".to_string()));
        assert_eq!(part_1(&vec!["{{{}}}".to_string()]), Ok("6".to_string()));
        assert_eq!(part_1(&vec!["{{},{}}".to_string()]), Ok("5".to_string()));
        assert_eq!(part_1(&vec!["{{{},{},{{}}}}".to_string()]), Ok("16".to_string()));
        assert_eq!(part_1(&vec!["{<a>,<a>,<a>,<a>}".to_string()]), Ok("1".to_string()));
        assert_eq!(part_1(&vec!["{{<ab>},{<ab>},{<ab>},{<ab>}}".to_string()]),
                   Ok("9".to_string()));
        assert_eq!(part_1(&vec!["{{<!!>},{<!!>},{<!!>},{<!!>}}".to_string()]),
                   Ok("9".to_string()));
        assert_eq!(part_1(&vec!["{{<a!>},{<a!>},{<a!>},{<ab>}}".to_string()]),
                   Ok("3".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 9)?;
        assert_eq!(part_1(&input), Ok("7616".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["<>".to_string()]), Ok("0".to_string()));
        assert_eq!(part_2(&vec!["<random characters>".to_string()]), Ok("17".to_string()));
        assert_eq!(part_2(&vec!["<<<<>".to_string()]), Ok("3".to_string()));
        assert_eq!(part_2(&vec!["<{!>}>".to_string()]), Ok("2".to_string()));
        assert_eq!(part_2(&vec!["<!!>".to_string()]), Ok("0".to_string()));
        assert_eq!(part_2(&vec!["<!!!>>".to_string()]), Ok("0".to_string()));
        assert_eq!(part_2(&vec!["<{o\"i!a,<{i<a>".to_string()]), Ok("10".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 9)?;
        assert_eq!(part_2(&input), Ok("3838".to_string()));
        Ok(())
    }
}
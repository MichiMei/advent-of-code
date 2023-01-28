use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, stdin};

pub mod year_2015;
pub mod year_2016;

pub fn read_lines_trimmed_from_file(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut res = vec![];
    for line in lines {
        let line = line?;
        res.push(String::from(line.trim()));
    }
    Ok(res)
}

pub fn read_lines_untrimmed_from_file(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut res = vec![];
    for line in lines {
        let line = line?;
        res.push(line);
    }
    Ok(res)
}

pub fn read_int_list_from_stdin() -> Vec<i32> {
    let mut res = vec![];
    for line in stdin().lock().lines() {
        let line = match line {
            Ok(str) => str,
            Err(_) => continue
        };
        let trimmed = line.trim();
        res.push(match trimmed.parse::<i32>() {
            Ok(int) => int,
            Err(_) => continue
        });
    }
    res
}

pub fn read_lines_trimmed_from_stdin() -> Vec<String> {
    let mut res = vec![];
    for line in stdin().lock().lines() {
        let line = match line {
            Ok(str) => str,
            Err(_) => continue
        };
        let trimmed = line.trim();
        res.push(String::from(trimmed));
    }
    res
}

pub fn read_lines_untrimmed_from_stdin() -> Vec<String> {
    let mut res = vec![];
    for line in stdin().lock().lines() {
        let line = match line {
            Ok(str) => str,
            Err(_) => continue
        };
        res.push(String::from(line));
    }
    res
}

pub mod errors {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    #[derive(Debug, PartialEq)]
    pub enum AoCError<Message: Debug + Display> {
        UnexpectedInputLength(Message),
        BadInputFormat(Message),
        NoSolutionFoundError(Message),
        MultipleSolutionsFoundError(Message)
    }

    impl<Message: Debug + Display> Display for AoCError<Message> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                AoCError::UnexpectedInputLength(message) => {
                    write!(f, "Input line count is not supported:\n{}", message)
                }
                AoCError::BadInputFormat(message) => {
                    write!(f, "The input has unexpected input:\n{}", message)
                }
                AoCError::NoSolutionFoundError(message) => {
                    write!(f, "No solution was found for the input:\n{}", message)
                }
                AoCError::MultipleSolutionsFoundError(message) => {
                    write!(f, "Multiple solutions were found for the input:\n{}", message)
                }
            }
        }
    }

    impl<Message: Debug + Display> Error for AoCError<Message> {}
}
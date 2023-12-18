use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, stdin};

pub mod year_2015;
pub mod year_2016;
pub mod year_2017;
pub mod year_2023;

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
        res.push(line);
    }
    res
}

pub mod output {
    pub fn bool_slice_to_string(slice: &[bool]) -> String {
        let mut output = String::new();
        for b in slice.iter() {
            if *b {
                output = format!("{}#", output);
            } else {
                output = format!("{}.", output);
            }
        }
        output
    }
}

pub mod errors {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    pub type AoCResult<T> = Result<T, AoCError<String>>;

    #[derive(Debug, PartialEq)]
    pub enum AoCError<Message: Debug + Display> {
        UnexpectedInputLength(Message),
        BadInputFormat(Message),
        NoSolutionFoundError(Message),
        MultipleSolutionsFoundError(Message),
        MultithreadingError(Message),
        IOError(Message),
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
                AoCError::MultithreadingError(message) => {
                    write!(f, "An error occurred while distributing the work to threads:\n{}",
                           message)
                }
                AoCError::IOError(message) => {
                    write!(f, "Input/Output operation failed:\n{}", message)
                }
            }
        }
    }

    impl<Message: Debug + Display> Error for AoCError<Message> {}
}

pub mod md5_collision {
    use std::fmt::Write;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use md5_rs::Context;
    use crate::errors::AoCError;

    pub fn find_hash_collision_parallel(input: &str, starting_nonce: usize, collision_length: usize)
        -> Result<Option<usize>, AoCError<String>> {
        let num_threads = num_cpus::get();
        let mutex = Arc::new(AtomicUsize::new(usize::MAX));

        let mut handles = Vec::with_capacity(num_threads);

        for thread_id in 0..num_threads {
            let input = input.to_string();
            let mutex = mutex.clone();
            let handle = std::thread::spawn(move || {
                collision_finder_thread(&input, starting_nonce+thread_id,
                                        num_threads, collision_length, mutex)
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread should not panic");
        }

        let nonce = mutex.load(Ordering::SeqCst);
        if nonce == usize::MAX && !collision(&hash(&format!("{}{}", input, nonce)),
                             collision_length) {
            return Ok(None)
        }

        Ok(Some(nonce))
    }

    fn collision_finder_thread(input: &str, starting_nonce: usize, step: usize, collision_length: usize, mutex: Arc<AtomicUsize>) {
        let mut nonce = starting_nonce;
        while nonce < mutex.load(Ordering::SeqCst) {
            let str = format!("{}{}", input, nonce);
            if collision(&hash(&str), collision_length) {
                let mut current = mutex.load(Ordering::SeqCst);
                while current > nonce {
                    match mutex.compare_exchange(current, nonce, Ordering::SeqCst, Ordering::SeqCst) {
                        Ok(_) => break,
                        Err(_) => current = mutex.load(Ordering::SeqCst),
                    }
                }
            }
            if let Some(new_nonce) = nonce.checked_add(step) {
                nonce = new_nonce;
            } else {
                break
            }
        }
    }

    pub fn hash(str: &str) -> [u8; 16] {
        let mut hasher = Context::new();
        hasher.read(str.as_bytes());
        hasher.finish()
    }

    fn collision(hash: &[u8], collision_length: usize) -> bool {
        let hex = hash.iter().fold(String::new(), |mut output, x| {
            let _ = write!(output, "{:02x?}", x);
            output
        });
        let pattern = "0".repeat(collision_length);
        hex.starts_with(&pattern)
    }
}

pub mod string_manipulation {
    use std::mem::swap;
    use crate::errors::AoCError;

    #[derive(Eq, PartialEq, Copy, Clone)]
    pub enum Direction {
        Left,
        Right,
    }

    impl Direction {
        pub fn reverse(self) -> Self {
            match self {
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            }
        }
    }

    /// Swaps the chars at the given positions.
    pub fn swap_positions(str: &str, mut src: usize, mut dest: usize) -> Result<String, AoCError<String>> {
        if src > dest {
            swap(&mut src, &mut dest);
        }
        if src >= str.len() {
            return Err(AoCError::BadInputFormat(format!("Swap positions src index out of bounds. \
        Password length {}, index {}", str.len(), src)))
        }
        if dest >= str.len() {
            return Err(AoCError::BadInputFormat(format!("Swap positions dest index out of bounds. \
        Password length {}, index {}", str.len(), dest)))
        }
        Ok(format!("{}{}{}{}{}",
                   &str[0..src],
                   &str[dest..dest+1],
                   &str[src+1..dest],
                   &str[src..src+1],
                   &str[dest+1..]))
    }

    /// Replaces all occurrences of char_x by char_y and reversed.
    pub fn swap_letters(str: &str, char_x: char, char_y: char) -> String {
        let mut pattern = "#".to_string();
        while str.contains(&pattern) {
            pattern = format!("{}#", pattern);
        }
        let mut res = str.replace(char_x, &pattern);
        res = res.replace(char_y, &char_x.to_string());
        res.replace(&pattern, &char_y.to_string())
    }

    /// Rotates the string a given number of steps to the right or left.
    /// 'abcde' rotated 2 steps right would result in 'deabc'
    pub fn rotate_steps(str: &str, dir: Direction, mut steps: usize) -> String {
        steps %= str.len();
        if dir == Direction::Right {
            steps = (str.len() - steps) % str.len();
        }

        format!("{}{}", &str[steps..], &str[0..steps])
    }

    /// Searches for the index of the first occurrence of the char, rotates the string right by 
    /// index+1 steps (or index+2 iff index >= 4).
    pub fn rotate_char_based(str: &str, char: char) -> Result<String, AoCError<String>> {
        let index = str.find(char)
            .ok_or_else(|| AoCError::BadInputFormat(format!("Char {} for char based rotating is not \
        contained in the password", char)))?;
        let steps = calculate_rotate_steps(index);
        Ok(rotate_steps(str, Direction::Right, steps))
    }

    /// Reverses the order of the characters from index_start to index_end (inclusive).
    pub fn reverse(str: &str, index_start: usize, index_end: usize) -> Result<String, AoCError<String>> {
        if index_start >= str.len() {
            return Err(AoCError::BadInputFormat(format!("Reverse index_start out of bounds. \
        Password length {}, index {}", str.len(), index_start)))
        }
        if index_end >= str.len() {
            return Err(AoCError::BadInputFormat(format!("Reverse index_end out of bounds. \
        Password length {}, index {}", str.len(), index_end)))
        }
        Ok(format!("{}{}{}",
                   &str[0..index_start],
                   &str[index_start..=index_end].chars().rev().collect::<String>(),
                   &str[index_end+1..]))
    }

    /// Removes the char at position src from the string. Inserts the char at position dest into the
    /// string.
    pub fn move_char(str: &str, src: usize, dest: usize) -> Result<String, AoCError<String>> {
        if src >= str.len() {
            return Err(AoCError::BadInputFormat(format!("Move src index out of bounds. \
        Password length {}, index {}", str.len(), src)))
        }
        if dest >= str.len() {
            return Err(AoCError::BadInputFormat(format!("Move dest index out of bounds. \
        Password length {}, index {}", str.len(), dest)))
        }

        let mut chars = str.chars().collect::<Vec<_>>();
        let char = chars.remove(src);
        chars.insert(dest, char);
        Ok(chars.iter().collect())
    }

    /// Reverse operation for 'rotate_char_based(..)'.
    pub fn reverse_rotate_char_based(str: &str, char: char) -> Result<String, AoCError<String>> {
        let char_index = str.find(char)
            .ok_or_else(|| AoCError::BadInputFormat(format!("Char {} for char based rotating is not \
        contained in the password", char)))?;
        let mut steps = None;
        for try_index in 0..str.len() {
            let try_steps = calculate_rotate_steps(try_index);
            if (try_index+ try_steps) % str.len() == char_index {
                if steps.is_none() {
                    steps = Some(try_steps);
                } else {
                    return Err(AoCError::BadInputFormat(
                        "Char based rotating could not be reversed".to_string()))
                }
            }
        }
        if let Some(steps) = steps {
            Ok(rotate_steps(str, Direction::Left, steps))
        } else {
            Err(AoCError::BadInputFormat(
                "Reversing char based rotating is impossible".to_string()))
        }
    }

    fn calculate_rotate_steps(char_index: usize) -> usize {
        if char_index >= 4 {
            char_index+2
        } else {
            char_index+1
        }
    }
}

pub mod input {
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    use std::path::Path;
    use aoc_client::AocClient;
    use crate::errors::AoCError;

    pub fn get_input(year: u16, day: u8) -> Result<Vec<String>, AoCError<String>> {
        let file_name = get_path(year, day)?;
        let file = if let Ok(file) = File::open(&file_name) {
            file
        } else {
            download(year, day)?;
            File::open(&file_name)
                .map_err(|e| AoCError::IOError(format!(
                    "Opening just created file failed. {}", e)))?
        };
        read_from_file(file)
    }

    fn get_path(year: u16, day: u8) -> Result<String, AoCError<String>> {
        if year < 2015 {
            return Err(AoCError::IOError("AoC only started in 2015.".to_string()))
        }
        if day == 0 || day > 25 {
            return Err(AoCError::IOError("Only days 1-25 supported.".to_string()))
        }
        Ok(format!("input/year_{}/input_day_{:02}.txt", year, day))
    }

    fn read_from_file(file: File) -> Result<Vec<String>, AoCError<String>> {
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let mut res = vec![];
        for line in lines {
            let line = line
                .map_err(|e| AoCError::IOError(format!("Reading from file failed: {}", e)))?;
            res.push(line);
        }
        Ok(res)
    }

    fn download(year: u16, day: u8) -> Result<(), AoCError<String>> {
        let client = AocClient::builder()
            .session_cookie_from_file("input/session_cookie")
            .map_err(|e| AoCError::IOError(format!(
                "Parsing session cookie failed: {}", e)))?
            .year(year as i32)
            .map_err(|e| AoCError::IOError(format!(
                "Parsing year failed: {}", e)))?
            .day(day as u32)
            .map_err(|e| AoCError::IOError(format!(
                "Parsing day failed: {}", e)))?
            .build()
            .map_err(|e| AoCError::IOError(format!(
                "Building AocClient failed: {}", e)))?;

        let input = client.get_input().map_err(|e| AoCError::IOError(format!(
            "Retrieving puzzle input failed: {}", e)))?;
        write_content_to_file(year, day, input)
    }

    fn write_content_to_file(year: u16, day: u8, content: String) -> Result<(), AoCError<String>> {
        let path = get_path(year, day)?;

        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| AoCError::IOError(format!(
                    "Creating folder structure '{:?}' failed. {}", parent, e)))?;
        }

        let mut file = File::create(&path)
            .map_err(|e| AoCError::IOError(format!(
                "Opening file '{}' failed: {}", path, e)))?;
        file.write_all(content.as_bytes())
            .map_err(|e| AoCError::IOError(format!("Writing to '{}' failed: {}", path, e)))?;
        Ok(())
    }


}

pub mod geometrics {
    use std::fmt::{Display, Formatter};
    use std::slice::Iter;
    use num::{CheckedAdd, CheckedSub, One};
    use crate::errors::{AoCError, AoCResult};

    //pub type Point = (usize, usize);
    pub type Point<I> = (I, I);

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Direction {
        pub fn parse_from_char(c: char) -> AoCResult<Self> {
            match c {
                'N'|'n' => Ok(Self::North),
                'E'|'e' => Ok(Self::East),
                'S'|'s' => Ok(Self::South),
                'W'|'w' => Ok(Self::West),
                c => Err(AoCError::BadInputFormat(
                    format!("Parsing Direction failed. Only initial letters (upper- and lowercase) \
                        supported. Found '{}'", c)))
            }
        }

        pub fn move_point<I: Copy + CheckedSub + CheckedAdd + One>(&self, point: &Point<I>)
            -> Option<Point<I>>
        {
            match self {
                Direction::North => point.1.checked_sub(&I::one()).map(|y| (point.0, y)),
                Direction::East => point.0.checked_add(&I::one()).map(|x| (x, point.1)),
                Direction::South => point.1.checked_add(&I::one()).map(|y| (point.0, y)),
                Direction::West => point.0.checked_sub(&I::one()).map(|x| (x, point.1)),
            }
        }

        pub fn get_right(&self) -> Self {
            match self {
                Direction::North => Self::East,
                Direction::East => Self::South,
                Direction::South => Self::West,
                Direction::West => Self::North,
            }
        }

        pub fn get_left(&self) -> Self {
            match self {
                Direction::North => Self::West,
                Direction::East => Self::North,
                Direction::South => Self::East,
                Direction::West => Self::South,
            }
        }

        pub fn get_opposing(&self) -> Self {
            match self {
                Direction::North => Self::South,
                Direction::East => Self::West,
                Direction::South => Self::North,
                Direction::West => Self::East,
            }
        }

        pub fn is_horizontal(&self) -> bool {
            match self {
                Direction::North => false,
                Direction::East => true,
                Direction::South => false,
                Direction::West => true,
            }
        }

        pub fn is_vertical(&self) -> bool {
            !self.is_horizontal()
        }

        pub fn get_horizontal() -> Vec<Self> {
            vec![
                Self::East,
                Self::West,
            ]
        }

        pub fn get_vertical() -> Vec<Self> {
            vec![
                Self::North,
                Self::South,
            ]
        }
    }

    pub struct Grid<T> {
        grid: Vec<Vec<T>>,
    }

    impl<T> Grid<T> {
        pub fn iter(&self) -> GridIter<'_, T> {
            GridIter {
                iter: self.grid.iter(),
            }
        }

        pub fn get_tile(&self, pos: &Point<usize>) -> Option<&T> {
            if let Some(row) = self.grid.get(pos.1) {
                row.get(pos.0)
            } else {
                None
            }
        }

        pub fn get_tile_mut(&mut self, pos: &Point<usize>) -> Option<&mut T> {
            if let Some(row) = self.grid.get_mut(pos.1) {
                row.get_mut(pos.0)
            } else {
                None
            }
        }

        pub fn set_tile(&mut self, pos: &Point<usize>, tile: T) -> bool {
            if let Some(prev) = self.get_tile_mut(pos) {
                *prev = tile;
                true
            } else {
                false
            }
        }

        pub fn get_dimension(&self) -> Point<usize> {
            if self.grid.is_empty() {
                return (0, 0)
            }
            (self.grid[0].len(), self.grid.len())
        }
    }


    impl<T: Eq> Grid<T> {
        pub fn get_all_positions_of(&self, pattern: &T) -> Vec<Point<usize>> {
            let mut res = vec![];
            for (row_index, row) in self.grid.iter().enumerate() {
                for (col_index, elem) in row.iter().enumerate() {
                    if elem == pattern {
                        res.push((row_index, col_index));
                    }
                }
            }
            res
        }
    }

    impl Grid<u8> {
        pub fn parse_digits(input: &[String]) -> AoCResult<Grid<u8>> {
            if input.is_empty() {
                return Err(AoCError::UnexpectedInputLength("Input cannot be empty".to_string()))
            }
            let mut grid = Vec::with_capacity(input.len());
            let width = input[0].len();
            for line in input {
                if width != line.len() {
                    return Err(AoCError::BadInputFormat(
                        "Lines need to have same number of digits.".to_string()))
                }
                let row = line.chars()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| (c as u8) - b'0')
                    .collect::<Vec<_>>();
                if width != row.len() {
                    return Err(AoCError::BadInputFormat(
                        "Input can only contain digits '0' to '9'".to_string()))
                }
                grid.push(row);
            }
            Ok(Self { grid })
        }
    }

    impl<T: Parsable> Grid<T> {
        pub fn parse(input: &[String]) -> AoCResult<Grid<T>> {
            if input.is_empty() {
                return Err(AoCError::UnexpectedInputLength("Input cannot be empty".to_string()))
            }
            let mut grid = Vec::with_capacity(input.len());
            let width = input[0].len();
            for line in input {
                if width != line.len() {
                    return Err(AoCError::BadInputFormat(
                        "Lines need to have same lengths".to_string()))
                }
                let row = line.chars()
                    .map(|c| T::parse(c))
                    .collect::<AoCResult<Vec<_>>>()?;
                assert_eq!(row.len(), width);
                grid.push(row);
            }
            Ok(Self { grid })
        }
    }

    impl<T: Display> Display for Grid<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for line in self.grid.iter() {
                for elem in line.iter() {
                    write!(f, "{}", elem)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    pub struct GridIter<'a, T> {
        iter: Iter<'a, Vec<T>>,
    }

    impl<'a, T> Iterator for GridIter<'a, T> {
        type Item = &'a Vec<T>;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }
    }

    pub trait Parsable {
        fn parse(c: char) -> AoCResult<Self> where Self: Sized;
    }
}
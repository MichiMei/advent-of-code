use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, stdin};

pub mod year_2015;
pub mod year_2016;
pub mod year_2017;

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

    #[derive(Debug, PartialEq)]
    pub enum AoCError<Message: Debug + Display> {
        UnexpectedInputLength(Message),
        BadInputFormat(Message),
        NoSolutionFoundError(Message),
        MultipleSolutionsFoundError(Message),
        MultithreadingError(Message),
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
            }
        }
    }

    impl<Message: Debug + Display> Error for AoCError<Message> {}
}

pub mod md5_collision {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use md5_rs::Context;
    use crate::errors::AoCError;

    pub fn find_hash_collision_parallel(input: &str, starting_nonce: usize, collision_length: usize)
        -> Result<Option<usize>, AoCError<String>> {
        let num_threads = num_cpus::get();
        let mutex = Arc::new(AtomicUsize::new(usize::MAX));

        let mut handles = vec![];
        handles.reserve(num_threads);

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
        let hex: String = hash.iter().map(|x| format!("{:02x?}", x)).collect();
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
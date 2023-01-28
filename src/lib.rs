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
        if nonce == usize::MAX {
            if !collision(&hash(&format!("{}{}", input, nonce)),
                             collision_length) {
                return Ok(None)
            }
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
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use md5_rs::Context;

pub fn part_1(input: &Vec<String>) -> Result<String, &str> {
    if input.len() != 1 {
        return Err(ERR_VEC_LENGTH)
    }
    let input = input.first().unwrap();
    find_hash_parallel(&input, 5).map(|t| t.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, &str> {
    if input.len() != 1 {
        return Err(ERR_VEC_LENGTH)
    }
    let input = input.first().unwrap();
    find_hash_parallel(&input, 6).map(|t| t.to_string())
}

fn find_hash_parallel(input: &str, zeros: usize) -> Result<usize, &str> {
    let num_threads = num_cpus::get();
    let mutex = Arc::new(AtomicUsize::new(usize::MAX));

    let mut handles = vec![];
    handles.reserve(num_threads);
    for modulo in 0..num_threads {
        let mutex = Arc::clone(&mutex);
        let input = input.to_string();
        let handle = std::thread::spawn(move || {
            let mut index = modulo;
            while index < mutex.load(Ordering::SeqCst) {
                let str = format!("{}{}", input, index);
                if leading_zeros(&hash(&str), zeros) {
                    let mut current = mutex.load(Ordering::SeqCst);
                    while current > index {
                        match mutex.compare_exchange(current, index, Ordering::SeqCst, Ordering::SeqCst) {
                            Ok(_) => break,
                            Err(_) => current = mutex.load(Ordering::SeqCst),
                        }
                    }
                }
                index += num_threads;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        if handle.join().is_err() {
            return Err(ERR_JOIN_FAILURE)
        }
    }

    Ok(mutex.load(Ordering::SeqCst))
}

fn hash(str: &str) -> String {
    let mut hasher = Context::new();
    hasher.read(str.as_bytes());
    let digest = hasher.finish();
    let mut res = String::new();
    for byte in digest {
        res = format!("{}{:0>2X}", res, byte);
    }
    res
}

fn leading_zeros(hash: &str, required_zeros: usize) -> bool {
    let pattern = "0".repeat(required_zeros);
    hash.starts_with(&pattern)
}

const ERR_VEC_LENGTH: &str = "The input is expected to be exactly one line";
const ERR_JOIN_FAILURE: &str = "Joining the threads failed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["abcdef".to_string()]), Ok("609043".to_string()));
        assert_eq!(part_1(&vec!["pqrstuv".to_string()]), Ok("1048970".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_04.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("282749".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_04.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("9962624".to_string()));
        Ok(())
    }
}
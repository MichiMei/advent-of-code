use std::collections::HashSet;
use crate::errors::AoCError;
use crate::md5_collision::hash;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the salt".to_string()))
    }
    let salt = &input[0];
    let mut cache = MD5Cache::new(salt, false);
    let mut count = 0;
    let mut index = 0;
    while count < 64 {
        if cache.hash_is_valid(index) {
            count += 1;
        }
        index += 1;
    }

    Ok((index-1).to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the salt".to_string()))
    }
    let salt = &input[0];
    let mut cache = MD5Cache::new(salt, true);
    let mut count = 0;
    let mut index = 0;
    while count < 64 {
    // while count < 1 {
        if cache.hash_is_valid(index) {
            count += 1;
        }
        index += 1;
    }

    Ok((index-1).to_string())
}

struct MD5Cache<'a> {
    cache: Vec<(Option<char>, HashSet<char>)>,
    salt: &'a str,
    key_stretching: bool,
}

impl<'a> MD5Cache<'a> {
    pub fn new(salt: &'a str, key_stretching: bool) -> Self {
        Self{cache: vec![], salt, key_stretching}
    }

    pub fn hash_is_valid(&mut self, index: usize) -> bool {
        if index+1000 >= self.cache.len() {
            //self.calculate_hash(index+5000);
            self.precalculate_hashes();
        }
        let triplet = self.get_triplet(index);
        if triplet.is_none() {
            return false;
        }
        for next in index+1..=index+1000 {
            if let Some(true) = self.contains_quintuplet(next, triplet
                .expect("triplet was tested to be some")) {
                return true
            }
        }
        false
    }

    fn get_triplet(&self, index: usize) -> Option<char> {
        if index >= self.cache.len() {
            return None
        }
        self.cache[index].0
    }

    fn contains_quintuplet(&self, index: usize, element: char) -> Option<bool> {
        if index >= self.cache.len() {
            return None
        }
        Some(self.cache[index].1.contains(&element))
    }

    fn precalculate_hashes(&mut self) {
        let amount = 4096;
        let num_threads = num_cpus::get();
        let block_size = amount/num_threads;
        let cache_length = self.cache.len();
        let mut handles = vec![];
        for thread_index in 0..num_threads {
            let start_index = cache_length + thread_index*block_size;
            let salt = self.salt.to_string();
            let key_stretching = self.key_stretching;
            let handle = std::thread::spawn(move || {
                let mut res = Vec::with_capacity(block_size);
                for hash_index in start_index..start_index+block_size {
                    let str = format!("{}{}", salt, hash_index);
                    let hash_str = if key_stretching {
                        let mut tmp = str;
                        for _ in 0..2017 {
                            tmp = hash(&tmp).iter()
                                .map(|b| format!("{:02x}", b)).collect::<String>();
                        }
                        tmp
                    } else {
                        hash(&str).iter().map(|b| format!("{:02x}", b)).collect::<String>()
                    };
                    let mut chars = hash_str.chars();
                    let mut triplets = None;
                    let mut quintuplet = HashSet::new();
                    let mut prev = chars.next().expect("hash has 32 chars");
                    let mut count = 1;
                    for elem in chars {
                        if elem == prev {
                            count += 1;
                            if count == 3 && triplets.is_none() {
                                triplets = Some(prev);
                            }
                            if count == 5 {
                                quintuplet.insert(prev);
                            }
                        } else {
                            prev = elem;
                            count = 1;
                        }
                    }
                    res.push((triplets, quintuplet))
                }
                res
            });
            handles.push(handle);
        }

        for handle in handles {
            let res = handle.join().expect("The treads should not panic");
            self.cache.extend(res);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["abc".to_string()]), Ok("22728".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 14)?;
        assert_eq!(part_1(&input), Ok("23769".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["abc".to_string()]), Ok("22551".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 14)?;
        assert_eq!(part_2(&input), Ok("20606".to_string()));
        Ok(())
    }
}
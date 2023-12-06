use std::collections::HashMap;
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 2 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Expected two lines of input, found {} lines", input.len())))
    }
    let mut caches = parse_as_list(&input[0], &input[1])?;

    let mut product = 1;
    for cache in caches.iter_mut() {
        let min = cache.find_min();
        let max = cache.find_max();
        let possible = max-min+1;
        product *= possible
    }

    Ok(product.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 2 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Expected two lines of input, found {} lines", input.len())))
    }
    let mut cache = parse_as_single_number(&input[0], &input[1])?;

    let min = cache.find_min();
    let max = cache.find_max();
    let possible = max-min+1;

    Ok(possible.to_string())
}

fn parse_as_list(mut times: &str, mut distances: &str) -> Result<Vec<Cache>, AoCError<String>> {
    if !times.starts_with("Time: ") {
        return Err(AoCError::BadInputFormat(
            format!("Expected time list to start with 'Time: ', found'{}'", times)))
    }
    times = times[6..].trim();
    if !distances.starts_with("Distance: ") {
        return Err(AoCError::BadInputFormat(
            format!("Expected distance list to start with 'Distance: ', found'{}'", distances)))
    }
    distances = distances[10..].trim();
    let times = times.split_whitespace().collect::<Vec<_>>();
    let distances = distances.split_whitespace().collect::<Vec<_>>();
    if times.len() != distances.len() {
        return Err(AoCError::BadInputFormat(
            format!("Expected time and distance lists to be same length. \
            {} != {}", times.len(), distances.len())))
    }
    let times = times.iter()
        .map(|str| str.parse::<usize>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing time failed. {}", e))))
        .collect::<Result<Vec<_>, _>>()?;
    let distances = distances.iter()
        .map(|str| str.parse::<usize>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing distance failed. {}", e))))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(times.into_iter().zip(distances)
        .map(|(time, distance)| Cache::new(Race{time, distance}))
        .collect())
}

fn parse_as_single_number(time: &str, distance: &str) -> Result<Cache, AoCError<String>> {
    if !time.starts_with("Time: ") {
        return Err(AoCError::BadInputFormat(
            format!("Expected time list to start with 'Time: ', found'{}'", time)))
    }
    let time = time[6..].trim().replace(' ', "");
    if !distance.starts_with("Distance: ") {
        return Err(AoCError::BadInputFormat(
            format!("Expected distance list to start with 'Distance: ', found'{}'", distance)))
    }
    let distance = distance[10..].trim().replace(' ', "");
    let time = time.parse::<usize>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing time failed. {}", e)))?;
    let distance = distance.parse::<usize>()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing distance failed. {}", e)))?;
    Ok(Cache::new(Race{time, distance}))
}

struct Race {
    time: usize,
    distance: usize,
}

struct Cache {
    race: Race,
    cache: HashMap<usize, usize>,
}

impl Cache {
    fn new(race: Race) -> Self {
        Self {
            race,
            cache: HashMap::new(),
        }
    }

    fn get_value(&mut self, press_time: usize) -> usize {
        if let Some(value) = self.cache.get(&press_time) {
            return *value
        }
        let drive_time = self.race.time - press_time;
        let value = drive_time*press_time;
        self.cache.insert(press_time, value);
        value
    }

    fn find_min(&mut self) -> usize {
        let goal = self.race.distance;
        let mut prev = (0usize, self.race.time/2);
        let mut curr = (prev.0+prev.1)/2;

        while self.get_value(curr) <= goal || self.get_value(curr-1) > goal {
            if self.get_value(curr) <= goal {
                let new_curr = (curr+prev.1)/2;
                assert_ne!(curr, new_curr);
                prev = (curr, prev.1);
                curr = new_curr;
            } else {
                let new_curr = (curr+prev.0)/2;
                assert_ne!(curr, new_curr);
                prev = (prev.0, curr);
                curr = new_curr;
            }
        }

        curr
    }

    fn find_max(&mut self) -> usize {
        let goal = self.race.distance;
        let mut prev = (self.race.time/2, self.race.time);
        let mut curr = (prev.0+prev.1)/2;

        while self.get_value(curr) <= goal || self.get_value(curr+1) > goal {
            if self.get_value(curr) <= goal {
                let new_curr = (curr+prev.0)/2;
                assert_ne!(curr, new_curr);
                prev = (prev.0, curr);
                curr = new_curr;
            } else {
                let new_curr = (curr+prev.1)/2;
                assert_ne!(curr, new_curr);
                prev = (curr, prev.1);
                curr = new_curr;
            }
        }

        curr
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("288".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 6)?;
        assert_eq!(part_1(&input), Ok("2612736".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("71503".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 6)?;
        assert_eq!(part_2(&input), Ok("29891250".to_string()));
        Ok(())
    }
}
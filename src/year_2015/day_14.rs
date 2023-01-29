use std::cmp::{max, min, Ordering};

pub fn part_1(input: &[String]) -> Result<String, &str> {
    let mut maximum = 0;
    for line in input {
        let reindeer = Reindeer::from(line)?;
        let distance = reindeer.get_distance(2503);
        maximum = max(maximum, distance);
    }

    Ok(maximum.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, &str> {
    let score = calculate_best_score(input, 2503)?;
    Ok(score.to_string())
}

fn calculate_best_score(input: &[String], rounds: usize) -> Result<usize, &str> {
    let mut reindeers = vec![];
    let mut scores = vec![];
    for line in input {
        reindeers.push(Reindeer::from(line)?);
        scores.push(0usize);
    }

    for _ in 0..rounds {
        let mut max_index = vec![];
        let mut max_distance = 0;
        for (index, reindeer) in reindeers.iter_mut().enumerate() {
            let tmp = reindeer.advance();
            match tmp.cmp(&max_distance) {
                Ordering::Less => {}
                Ordering::Equal => max_index.push(index),
                Ordering::Greater => {
                    max_distance = tmp;
                    max_index = vec![index];
                }
            }
        }
        assert!(!max_index.is_empty());
        for index in max_index {
            scores[index] += 1;
        }
    }

    let mut maximum = 0;
    for score in scores {
        maximum = max(maximum, score);
    }

    Ok(maximum)
}

struct Reindeer {
    speed: usize,
    fly_time: usize,
    rest_time: usize,
    status: Status,
    distance: usize,
}

impl Reindeer {
    fn from(str: &str) -> Result<Self, &str> {
        let words: Vec<&str> = str.split(' ').collect();
        if words.len() != 15 {
            return Err(ERR_INPUT_MALFORMED)
        }
        let speed = words[3].parse().map_err(|_| ERR_INPUT_MALFORMED)?;
        let fly_time = words[6].parse().map_err(|_| ERR_INPUT_MALFORMED)?;
        let rest_time = words[13].parse().map_err(|_| ERR_INPUT_MALFORMED)?;
        let status = Status::Flying(fly_time);
        let distance = 0;

        Ok(Self{speed, fly_time, rest_time, status, distance})
    }

    fn get_distance(&self, time: usize) -> usize {
        let full_cycles = time / (self.fly_time+self.rest_time);
        let last_cycle = min(self.fly_time, time%(self.fly_time+self.rest_time));

        full_cycles*self.fly_time*self.speed + last_cycle*self.speed
    }

    fn advance(&mut self) -> usize {
        match self.status {
            Status::Flying(1) => {
                self.status = Status::Resting(self.rest_time);
                self.distance += self.speed;
            }
            Status::Flying(remaining) => {
                self.status = Status::Flying(remaining-1);
                self.distance += self.speed;
            }
            Status::Resting(1) => {
                self.status = Status::Flying(self.fly_time);
            }
            Status::Resting(remaining) => {
                self.status = Status::Resting(remaining-1);
            }
        }
        self.distance
    }
}

enum Status {
    Flying(usize),
    Resting(usize),
}

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_reindeer_get_distance() -> Result<(), &'static str>{
        let v0 = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        let r0 = Reindeer::from(v0)?;
        assert_eq!(r0.get_distance(1000), 1120);
        let v1 = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let r1 = Reindeer::from(v1)?;
        assert_eq!(r1.get_distance(1000), 1056);
        Ok(())
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_14.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("2660".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = vec![
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.".to_string(),
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.".to_string()
        ];
        assert_eq!(calculate_best_score(&v, 1000), Ok(689));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_14.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("1256".to_string()));
        Ok(())
    }
}
use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let mut disks = vec![];
    for line in input {
        disks.push(Disk::parse(line)?);
    }
    disks.sort_unstable_by(|d0, d1| d0.states.cmp(&d1.states));
    disks.reverse();

    let valid_time = calculate_valid_time(&disks);

    Ok(valid_time.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let mut disks = vec![];
    for line in input {
        disks.push(Disk::parse(line)?);
    }
    disks.push(Disk::new(11, 0, disks.len()+1));
    disks.sort_unstable_by(|d0, d1| d0.states.cmp(&d1.states));
    disks.reverse();

    let valid_time = calculate_valid_time(&disks);

    Ok(valid_time.to_string())
}

fn calculate_valid_time(disks: &[Disk]) -> usize {
    let mut time = disks[0].get_first_valid_time();
    loop {
        assert!(disks[0].check_time(time));
        let mut all_valid = true;
        for disk in disks[1..].iter() {
            if !disk.check_time(time) {
                all_valid = false;
                break
            }
        }
        if all_valid {
            break time;
        }
        time += disks[0].states;
    }
}

struct Disk {
    states: usize,
    start_pos: usize,
    index: usize,
}

impl Disk {
    pub fn new(states: usize, start_pos: usize, index: usize) -> Self {
        Self{states, start_pos, index}
    }

    pub fn parse(line: &str) -> Result<Self, AoCError<String>> {
        let words = line.split(' ').collect::<Vec<_>>();
        if words.len() != 12 {
            return Err(AoCError::BadInputFormat(format!(
                "Expected input 'Disc #<disk-index> has <positions> positions; at time=0, it is at \
                position <start-position>.'. Found '{}'", line)))
        }

        let index = words[1][1..].parse()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing disk index failed: {}", e)))?;
        let states = words[3].parse()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing disk positions failed: {}", e)))?;
        let start_pos = words[11][0..words[11].len()-1].parse()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing starting position failed: {}", e)))?;

        Ok(Self{states, start_pos, index})
    }

    pub fn check_time(&self, time: usize) -> bool {
        (self.start_pos+time)%self.states == self.get_desired()
    }

    fn get_desired(&self) -> usize {
        let mut tmp = self.states;
        while tmp < self.index {
            tmp += self.states;
        }
        tmp-self.index
    }

    pub fn get_first_valid_time(&self) -> usize {
        let desired = self.states-self.index;
        (desired+self.states-self.start_pos)%self.states
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Disc #{} has {} positions; at time=0, it is at position {}.",
               self.index, self.states, self.start_pos)
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "Disc #1 has 5 positions; at time=0, it is at position 4.".to_string(),
            "Disc #2 has 2 positions; at time=0, it is at position 1.".to_string(),
        ];
        assert_eq!(part_1(&v), Ok("5".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_15.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("203660".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_15.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("2408135".to_string()));
        Ok(())
    }
}
use std::cmp::min;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut sum = 0;
    for line in input.iter() {
        let (l, w, h) = parse_side_lengths(line)?;
        sum += calc_surface(l, w, h) + calc_smallest_side(l, w, h);
    }
    Ok(sum.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut sum = 0;
    for line in input.iter() {
        let (l, w, h) = parse_side_lengths(line)?;
        sum += calc_volume(l, w, h) + calc_shortest_equator(l, w, h);
    }
    Ok(sum.to_string())
}

fn parse_side_lengths(line: &str) -> Result<(usize, usize, usize), AoCError<String>> {
    let split: Vec<&str> = line.split('x').collect();
    if split.len() != 3 {
        return Err(AoCError::BadInputFormat(
            format!("An input line is malformed. \
            Expected '<length>x<width>x<height>', found '{}'", line)
        ))
    }
    let l = parse_value(split[0])?;
    let w = parse_value(split[1])?;
    let h = parse_value(split[2])?;

    Ok((l, w, h))
}

fn parse_value(str: &str) -> Result<usize, AoCError<String>> {
    let val = match str.parse::<usize>() {
        Ok(v) => v,
        Err(_) => {
            return Err(AoCError::BadInputFormat(
                format!("Could not parse number, only positive numbers supported. Found {}", str)
            ))
        }
    };
    Ok(val)
}

fn calc_surface(l: usize, w: usize, h: usize) -> usize {
    2*l*w + 2*l*h + 2*w*h
}

fn calc_smallest_side(l: usize, w: usize, h: usize) -> usize {
    min(l*w, min(l*h, w*h))
}

fn calc_volume(l: usize, w: usize, h: usize) -> usize {
    l*w*h
}

fn calc_shortest_equator(l: usize, w: usize, h: usize) -> usize {
    let mut sides = vec![l, w, h];
    sides.sort();

    sides[0]*2 + sides[1]*2
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&["2x3x4".to_string()]), Ok("58".to_string()));
        assert_eq!(part_1(&["1x1x10".to_string()]), Ok("43".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_02.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;
        assert_eq!(part_1(&input), Ok("1606483".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&["2x3x4".to_string()]), Ok("34".to_string()));
        assert_eq!(part_2(&["1x1x10".to_string()]), Ok("14".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_02.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;
        assert_eq!(part_2(&input), Ok("3842356".to_string()));
        Ok(())
    }
}
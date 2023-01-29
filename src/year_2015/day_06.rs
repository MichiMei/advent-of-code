pub fn part_1(input: &[String]) -> Result<String, &str> {
    let line = vec![false; 1000];
    let mut grid = vec![line; 1000];
    for line in input {
        let (mode, c0, c1) = parse_line(line)?;
        for x in c0.0..=c1.0 {
            for y in c0.1..=c1.1 {
                match mode {
                    Mode::Turn(status) => grid[x][y] = status,
                    Mode::Toggle => grid[x][y] = !grid[x][y],
                }
            }
        }
    }
    let mut count = 0usize;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] {
                count += 1;
            }
        }
    }
    Ok(count.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, &str> {
    let line = vec![0u8; 1000];
    let mut grid = vec![line; 1000];
    for line in input {
        let (mode, c0, c1) = parse_line(line)?;
        for x in c0.0..=c1.0 {
            for y in c0.1..=c1.1 {
                match mode {
                    Mode::Turn(status) => {
                        if status {
                            grid[x][y] += 1;
                        } else if grid[x][y] > 0 {
                            grid[x][y] -= 1;
                        }
                    }
                    Mode::Toggle => grid[x][y] += 2,
                }
            }
        }
    }
    let mut count = 0i128;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            count += grid[x][y] as i128;
        }
    }
    Ok(count.to_string())
}

fn parse_line(str: &str) -> Result<(Mode, (usize, usize), (usize, usize)), &str> {
    if str.starts_with("turn") {
        let words: Vec<&str> = str.split(' ').collect();
        if words.len() != 5 {
            return Err(ERR_INPUT_MALFORMED)
        }
        let mode = match words[1] {
            "on" => Mode::Turn(true),
            "off" => Mode::Turn(false),
            _ => return Err(ERR_INPUT_MALFORMED)
        };
        let c0 = parse_corner(words[2])?;
        let c1 = parse_corner(words[4])?;
        Ok((mode, c0, c1))
    } else if str.starts_with("toggle") {
        let words: Vec<&str> = str.split(' ').collect();
        if words.len() != 4 {
            return Err(ERR_INPUT_MALFORMED)
        }
        let mode = Mode::Toggle;
        let c0 = parse_corner(words[1])?;
        let c1 = parse_corner(words[3])?;
        Ok((mode, c0, c1))
    } else {
        Err(ERR_INPUT_MALFORMED)
    }
}

fn parse_corner(str: &str) -> Result<(usize, usize), &'static str> {
    let words: Vec<&str> = str.split(',').collect();
    if words.len() != 2 {
        return Err(ERR_INPUT_MALFORMED)
    }
    let val0 = words[0].parse().map_err(|_| ERR_INPUT_MALFORMED)?;
    let val1 = words[1].parse().map_err(|_| ERR_INPUT_MALFORMED)?;
    Ok((val0, val1))
}

enum Mode {
    Turn(bool),
    Toggle,
}

const ERR_INPUT_MALFORMED: &str = "Input is malformed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let s0 = "turn on 0,0 through 999,999";
        let s1 = "toggle 0,0 through 999,0";
        let s2 = "turn off 499,499 through 500,500";

        let v0 = vec![s0.to_string()];
        let v1 = vec![s0.to_string(), s1.to_string()];
        let v2 = vec![s0.to_string(), s1.to_string(), s2.to_string()];

        assert_eq!(part_1(&v0), Ok("1000000".to_string()));
        assert_eq!(part_1(&v1), Ok("999000".to_string()));
        assert_eq!(part_1(&v2), Ok("998996".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_06.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("377891".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&["turn on 0,0 through 0,0".to_string()]),
                   Ok("1".to_string()));
        assert_eq!(part_2(&["toggle 0,0 through 999,999".to_string()]),
                   Ok("2000000".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_06.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("14110788".to_string()));
        Ok(())
    }
}
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let line = vec![false; 1000];
    let mut grid = vec![line; 1000];
    for line in input {
        let (mode, c0, c1) = parse_line(line)?;
        for row in grid[c0.0..=c1.0].iter_mut() {
            for elem in row[c0.1..=c1.1].iter_mut() {
                match mode {
                    Mode::Turn(status) => *elem = status,
                    Mode::Toggle => *elem = !*elem
                }
            }
        }
    }
    let mut count = 0usize;
    for row in grid.iter() {
        for elem in row.iter() {
            if *elem {
                count += 1;
            }
        }
    }
    Ok(count.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let line = vec![0u8; 1000];
    let mut grid = vec![line; 1000];
    for line in input {
        let (mode, c0, c1) = parse_line(line)?;
        for row in grid[c0.0..=c1.0].iter_mut() {
            for elem in row[c0.1..=c1.1].iter_mut() {
                match mode {
                    Mode::Turn(true) => *elem += 1,
                    Mode::Turn(false) => *elem = elem.saturating_sub(1),
                    Mode::Toggle => *elem += 2,
                }
            }
        }
    }
    let mut count = 0i128;
    for row in grid.iter() {
        for elem in row.iter() {
            count += *elem as i128
        }
    }
    Ok(count.to_string())
}

type Command = (Mode, (usize, usize), (usize, usize));

fn parse_line(str: &str) -> Result<Command, AoCError<String>> {
    if str.starts_with("turn") {
        let words: Vec<&str> = str.split(' ').collect();
        if words.len() != 5 {
            return Err(AoCError::BadInputFormat(
                format!("Bad 'turn' instruction. Expected: 'turn [on/off] <p0> through <p1>'.\n\
                found: {}", str)
            ))
        }
        let mode = match words[1] {
            "on" => Mode::Turn(true),
            "off" => Mode::Turn(false),
            _ => {
                return Err(AoCError::BadInputFormat(
                    format!("Bad 'turn' instruction. Expected: 'turn [on/off] <p0> through <p1>'.\n\
                        found: {}", str)
                ));
            }
        };
        let c0 = parse_corner(words[2])?;
        let c1 = parse_corner(words[4])?;
        Ok((mode, c0, c1))
    } else if str.starts_with("toggle") {
        let words: Vec<&str> = str.split(' ').collect();
        if words.len() != 4 {
            return Err(AoCError::BadInputFormat(
                format!("Bad 'toggle' instruction. Expected: 'toggle <p0> through <p1>'.\n\
                    found: {}", str)
            ))
        }
        let mode = Mode::Toggle;
        let c0 = parse_corner(words[1])?;
        let c1 = parse_corner(words[3])?;
        Ok((mode, c0, c1))
    } else {
        return Err(AoCError::BadInputFormat(
            format!("Bad instruction. Expected: \n
                \t'toggle <p0> through <p1>'.\n\
                \t'turn [on/off] <p0> through <p1>'
                found: {}", str)
        ))
    }
}

fn parse_corner(str: &str) -> Result<(usize, usize), AoCError<String>> {
    let words: Vec<&str> = str.split(',').collect();
    if words.len() != 2 {
        return Err(AoCError::BadInputFormat(
            format!("Could not parse point, expected 'x,y', found '{}'", str)
        ))
    }
    let val0 = words[0].parse().map_err(|_| AoCError::BadInputFormat(
        format!("Could not parse number. Only positive numbers allowed, found {}", words[0])
    ))?;
    let val1 = words[1].parse().map_err(|_| AoCError::BadInputFormat(
        format!("Could not parse number. Only positive numbers allowed, found {}", words[1])
    ))?;
    Ok((val0, val1))
}

enum Mode {
    Turn(bool),
    Toggle,
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
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
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 6)?;
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
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 6)?;
        assert_eq!(part_2(&input), Ok("14110788".to_string()));
        Ok(())
    }
}
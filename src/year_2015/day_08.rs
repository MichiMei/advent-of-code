use std::iter::Peekable;

pub fn part_1(input: &Vec<String>) -> Result<String, &str> {
    let mut sum = 0;

    for line in input {
        let res = remove_escape_characters(line)?;
        sum += line.len();
        sum -= res.chars().collect::<Vec<char>>().len();
    }
    Ok(sum.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, &str> {
    let mut sum = 0;

    for line in input {
        let res = encode(line);
        sum += res.len();
        sum -= line.len();
    }
    Ok(sum.to_string())
}

fn remove_escape_characters(str: &str) -> Result<String, &str> {
    if !str.is_ascii() {
        return Err(ERR_INPUT_MALFORMED)
    }
    let mut res = vec![];
    let mut bytes = str.bytes().peekable();

    if let Some(c) = bytes.next() {
        if c != 34 {
            return Err(ERR_INPUT_MALFORMED)
        }
    } else {
        return Err(ERR_INPUT_MALFORMED)
    }

    while let Some(byte) = bytes.next() {
        if bytes.peek().is_none() {
            if byte != 34 {
                return Err(ERR_INPUT_MALFORMED)
            }
            continue
        }

        match byte {
            92 => {
                res.push(parse_escape(&mut bytes)?)
            },
            _ => res.push(byte as char),
        }
    }
    Ok(res.into_iter().collect())
}

fn parse_escape(bytes: &mut Peekable<core::str::Bytes>) -> Result<char, &'static str> {
    match bytes.next() {
        Some(92) => Ok(92 as char),
        Some(34) => Ok(34 as char),
        Some(120) => {
            // read 2 char -> string -> u8 (hexadecimal) -> return
            let char_hex = [
                bytes.next().ok_or(ERR_INPUT_MALFORMED)?,
                bytes.next().ok_or(ERR_INPUT_MALFORMED)?
            ];
            let int = u8::from_str_radix(&String::from_utf8_lossy(&char_hex), 16).map_err(|_| ERR_INPUT_MALFORMED)?;
            Ok(int as char)
        }
        _ => Err(ERR_INPUT_MALFORMED),
    }
}

fn encode(str: &str) -> String {
    let chars = str.chars();
    let mut res = vec!['"'];
    for char in chars {
        match char {
            '"' => {
                res.push('\\');
                res.push('"');
            }
            '\\' => {
                res.push('\\');
                res.push('\\');
            }
            c => res.push(c),
        }
    }
    res.push('"');
    res.into_iter().collect()
}

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v0 = vec!["\"\"".to_string()];
        let v1 = vec!["\"abc\"".to_string()];
        let v2 = vec!["\"aaa\\\"aaa\"".to_string()];
        let v3 = vec!["\"\\x27\"".to_string()];

        assert_eq!(part_1(&v0), Ok("2".to_string()));
        assert_eq!(part_1(&v1), Ok("2".to_string()));
        assert_eq!(part_1(&v2), Ok("3".to_string()));
        assert_eq!(part_1(&v3), Ok("5".to_string()));

        let v4 = vec![
            "\"\"".to_string(),
            "\"abc\"".to_string(),
            "\"aaa\\\"aaa\"".to_string(),
            "\"\\x27\"".to_string()
        ];

        assert_eq!(part_1(&v4), Ok("12".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_08.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("1342".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v0 = vec!["\"\"".to_string()];
        let v1 = vec!["\"abc\"".to_string()];
        let v2 = vec!["\"aaa\\\"aaa\"".to_string()];
        let v3 = vec!["\"\\x27\"".to_string()];

        assert_eq!(part_2(&v0), Ok("4".to_string()));
        assert_eq!(part_2(&v1), Ok("4".to_string()));
        assert_eq!(part_2(&v2), Ok("6".to_string()));
        assert_eq!(part_2(&v3), Ok("5".to_string()));

        let v4 = vec![
            "\"\"".to_string(),
            "\"abc\"".to_string(),
            "\"aaa\\\"aaa\"".to_string(),
            "\"\\x27\"".to_string()
        ];

        assert_eq!(part_2(&v4), Ok("19".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_08.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("2074".to_string()));
        Ok(())
    }
}
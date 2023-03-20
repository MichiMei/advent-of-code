use std::iter::Peekable;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut sum = 0;

    for line in input {
        let res = remove_escape_characters(line)?;
        sum += line.len();
        sum -= res.chars().count();
    }
    Ok(sum.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut sum = 0;

    for line in input {
        let res = encode(line);
        sum += res.len();
        sum -= line.len();
    }
    Ok(sum.to_string())
}

fn remove_escape_characters(str: &str) -> Result<String, AoCError<String>> {
    if !str.is_ascii() {
        return Err(AoCError::BadInputFormat(
            format!("Only ascii strings supported, found '{}'", str)
        ))
    }
    let mut res = vec![];
    let mut bytes = str.bytes().peekable();

    if let Some(c) = bytes.next() {
        if c != b'\"' {
            return Err(AoCError::BadInputFormat(
                format!("Inputs have to start with '\"'. Found '{}'", str)
            ))
        }
    } else {
        return Err(AoCError::BadInputFormat(
            format!("Inputs have to start with '\"'. Found '{}'", str)
        ))
    }

    while let Some(byte) = bytes.next() {
        if bytes.peek().is_none() {
            if byte != b'\"' {
                return Err(AoCError::BadInputFormat(
                    format!("Inputs have to end with '\"'. Found '{}'", str)
                ))
            }
            continue
        }

        match byte {
            b'\\' => {
                res.push(parse_escape(&mut bytes)?)
            },
            _ => res.push(byte as char),
        }
    }
    Ok(res.into_iter().collect())
}

fn parse_escape(bytes: &mut Peekable<core::str::Bytes>) -> Result<char, AoCError<String>> {
    match bytes.next() {
        Some(b'\\') => Ok('\\'),
        Some(b'\"') => Ok('\"'),
        Some(b'x') => {
            // read 2 char -> string -> u8 (hexadecimal) -> return
            let char_hex = [
                bytes.next().ok_or_else(|| AoCError::BadInputFormat(
                    "At least two characters need to follow after '\\x'".to_string()
                ))?,
                bytes.next().ok_or_else(|| AoCError::BadInputFormat(
                    "At least two characters need to follow after '\\x'".to_string()
                ))?
            ];
            let int = u8::from_str_radix(&String::from_utf8_lossy(&char_hex), 16)
                .map_err(|e| AoCError::BadInputFormat(
                    format!("Could not parse byte following '\\x'. Found {:?}.\n{}", char_hex, e)
                ))?;
            Ok(int as char)
        }
        _ => {
            Err(AoCError::BadInputFormat(
                "Unexpected character after '\\'. Only '\\', '\"' and 'x' supported".to_string()
            ))
        },
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

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&["\"\"".to_string()]), Ok("2".to_string()));
        assert_eq!(part_1(&["\"abc\"".to_string()]), Ok("2".to_string()));
        assert_eq!(part_1(&["\"aaa\\\"aaa\"".to_string()]), Ok("3".to_string()));
        assert_eq!(part_1(&["\"\\x27\"".to_string()]), Ok("5".to_string()));

        let v4 = vec![
            "\"\"".to_string(),
            "\"abc\"".to_string(),
            "\"aaa\\\"aaa\"".to_string(),
            "\"\\x27\"".to_string()
        ];

        assert_eq!(part_1(&v4), Ok("12".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 8)?;
        assert_eq!(part_1(&input), Ok("1342".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&["\"\"".to_string()]), Ok("4".to_string()));
        assert_eq!(part_2(&["\"abc\"".to_string()]), Ok("4".to_string()));
        assert_eq!(part_2(&["\"aaa\\\"aaa\"".to_string()]), Ok("6".to_string()));
        assert_eq!(part_2(&["\"\\x27\"".to_string()]), Ok("5".to_string()));

        let v4 = vec![
            "\"\"".to_string(),
            "\"abc\"".to_string(),
            "\"aaa\\\"aaa\"".to_string(),
            "\"\\x27\"".to_string()
        ];

        assert_eq!(part_2(&v4), Ok("19".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 8)?;
        assert_eq!(part_2(&input), Ok("2074".to_string()));
        Ok(())
    }
}
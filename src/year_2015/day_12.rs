use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("The input is expected to be exactly one line, found {} lines", input.len())
        ))
    }
    let json = input.first().unwrap();
    let mut chars = json.chars();
    let mut sum = 0;
    while let Some(num) = get_next_int(&mut chars)? {
        sum += num;
    }

    Ok(sum.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("The input is expected to be exactly one line, found {} lines", input.len())
        ))
    }
    let line = input.first().unwrap();
    let json = JsonValue::parse(line)?;
    let sum = json.sum_red_aware();
    Ok(sum.to_string())
}

fn get_next_int(chars: &mut Chars) -> Result<Option<i32>, AoCError<String>> {
    let first = loop {
        let next = chars.next();
        if next.is_none() {
            return Ok(None)
        }
        let next = next.unwrap();
        if next.is_numeric() || next == '-' {
            break next
        }
    };

    let mut number_vec = vec![first];
    for next in chars.by_ref() {
        if !next.is_numeric() {
            break;
        }
        number_vec.push(next)
    }

    Ok(Some(number_vec.into_iter().collect::<String>().parse().map_err(|e| AoCError::BadInputFormat(
        format!("Parsing number failed.\n{}", e)
    ))?))
}

enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(i32),
}

impl JsonValue {
    fn parse(str: &str) -> Result<JsonValue, AoCError<String>> {
        Self::parse_chars(&mut str.chars().peekable())
    }

    fn parse_chars(chars: &mut Peekable<Chars>) -> Result<JsonValue, AoCError<String>> {
        let json_value = match chars.peek().ok_or_else(|| AoCError::BadInputFormat(
            "Input cannot be empty. Minimal input is '{}'".to_string()
        ))? {
            '{' => {
                chars.next();
                let mut content = HashMap::new();
                let mut value_expected = true;
                let mut comma_expected = false;
                let mut end_expected = true;
                loop {
                    trim(chars);
                    match chars.peek() {
                        Some('}') => {
                            chars.next();
                            if !end_expected {
                                return Err(AoCError::BadInputFormat(
                                    "Unexpected '}'".to_string()
                                ))
                            }
                            break
                        }
                        Some(',') => {
                            chars.next();
                            if !comma_expected {
                                return Err(AoCError::BadInputFormat(
                                    "Unexpected ','".to_string()
                                ))
                            }
                            value_expected = true;
                            comma_expected = false;
                            end_expected = false;
                        }
                        Some(_) => {
                            if !value_expected {
                                return Err(AoCError::BadInputFormat(
                                    "Unexpected value".to_string()
                                ))
                            }
                            let key = match Self::parse_chars(chars)? {
                                JsonValue::String(str) => str,
                                x => {
                                    return Err(AoCError::BadInputFormat(
                                        format!("Expected a String literal, found {}", x)
                                    ))
                                },
                            };
                            trim(chars);
                            let x = chars.next().ok_or_else(|| AoCError::BadInputFormat(
                                "Unexpected end, expected ':'".to_string()
                            ))?;
                            if x != ':' {
                                return Err(AoCError::BadInputFormat(
                                    format!("Expected ':', found {}", x)
                                ))
                            }
                            trim(chars);
                            let val = Self::parse_chars(chars)?;
                            content.insert(key, val);
                            value_expected = false;
                            comma_expected = true;
                            end_expected = true;
                        }
                        None => {
                            return Err(AoCError::BadInputFormat(
                                "Unexpected end".to_string()
                            ))
                        },
                    }
                };
                Self::Object(content)
            }
            '[' => {
                chars.next();
                let mut content = vec![];
                let mut value_expected = true;
                let mut comma_expected = false;
                let mut end_expected = true;
                loop {
                    trim(chars);
                    match chars.peek() {
                        Some(']') => {
                            chars.next();
                            if !end_expected {
                                return Err(AoCError::BadInputFormat(
                                    "Unexpected ']'".to_string()
                                ))
                            }
                            break
                        }
                        Some(',') => {
                            chars.next();
                            if !comma_expected {
                                return Err(AoCError::BadInputFormat(
                                    "Unexpected ','".to_string()
                                ))
                            }
                            value_expected = true;
                            comma_expected = false;
                            end_expected = false;
                        }
                        Some(_) => {
                            if !value_expected {
                                return Err(AoCError::BadInputFormat(
                                    "Unexpected value".to_string()
                                ))
                            }
                            content.push(Self::parse_chars(chars)?);
                            value_expected = false;
                            comma_expected = true;
                            end_expected = true;
                        },
                        None => {
                            return Err(AoCError::BadInputFormat(
                                "Unexpected end".to_string()
                            ))
                        }
                    }
                };
                Self::Array(content)
            }
            '"' => {
                chars.next();
                read_string(chars).map(Self::String)?
            }
            _ => {
                read_int(chars).map(Self::Number)?
            }
        };
        trim(chars);
        Ok(json_value)
    }

    fn sum_red_aware(&self) -> i32 {
        if self.contains("red") {
            return 0
        }
        match self {
            JsonValue::Object(map) => {
                let mut sum = 0;
                for elem in map.values() {
                    sum += elem.sum_red_aware();
                }
                sum
            }
            JsonValue::Array(list) => {
                let mut sum = 0;
                for elem in list {
                    sum += elem.sum_red_aware();
                }
                sum
            }
            JsonValue::String(_) => 0,
            JsonValue::Number(val) => *val,
        }
    }

    fn contains(&self, value: &str) -> bool {
        match self {
            JsonValue::Object(map) => {
                for (_, other) in map.iter() {
                    if let JsonValue::String(str) = other {
                        if str == value {
                            return true
                        }
                    }
                }
            }
            _ => return false,
        }

        false
    }
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::Object(map) => {
                write!(f, "{{").unwrap();
                let mut first = true;
                for (k, v) in map {
                    if first {
                        write!(f, "{}:{}", k, v).unwrap();
                    } else {
                        write!(f, ",{}:{}", k, v).unwrap();
                    }
                    first = false;
                }
                write!(f, "}}")
            }
            JsonValue::Array(list) => {
                write!(f, "[").unwrap();
                let mut iter = list.iter();
                if let Some(x) = iter.next() {
                    write!(f, "{}", x).unwrap();
                }
                for x in iter {
                    write!(f, ",{}", x).unwrap();
                }
                write!(f, "]")
            }
            JsonValue::String(val) => write!(f, "\"{}\"", val),
            JsonValue::Number(val) => write!(f, "{}", val),
        }
    }
}

fn read_int(chars: &mut Peekable<Chars>) -> Result<i32, AoCError<String>> {
    let mut tmp = vec![];
    while let Some(val) = chars.peek() {
        match val {
            ',' | ']' | '}' => break,
            _ => tmp.push(chars.next().unwrap()),
        }
    }
    tmp.into_iter().collect::<String>().parse().map_err(|e| AoCError::BadInputFormat(
        format!("Parsing number failed.\n{}", e)
    ))
}

fn read_string(chars: &mut Peekable<Chars>) -> Result<String, AoCError<String>> {
    let mut tmp = vec![];
    for val in chars.by_ref() {
        match val {
            '"' => return Ok(tmp.into_iter().collect()),
            _ => tmp.push(val),
        }
    }
    Err(AoCError::BadInputFormat(
        "Unexpected end".to_string()
    ))
}

fn trim(chars: &mut Peekable<Chars>) {
    while let Some(next) = chars.peek() {
        if !next.is_whitespace() {
            break
        }
        chars.next();
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&["[1,2,3]".to_string()]), Ok("6".to_string()));
        assert_eq!(part_1(&["{\"a\":2,\"b\":4}".to_string()]), Ok("6".to_string()));
        assert_eq!(part_1(&["[[[3]]]".to_string()]), Ok("3".to_string()));
        assert_eq!(part_1(&["{\"a\":{\"b\":4},\"c\":-1}".to_string()]), Ok("3".to_string()));
        assert_eq!(part_1(&["{\"a\":[-1,1]}".to_string()]), Ok("0".to_string()));
        assert_eq!(part_1(&["[-1,{\"a\":1}]".to_string()]), Ok("0".to_string()));
        assert_eq!(part_1(&["[]".to_string()]), Ok("0".to_string()));
        assert_eq!(part_1(&["{}".to_string()]), Ok("0".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 12)?;
        assert_eq!(part_1(&input), Ok("111754".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&["[1,2,3]".to_string()]), Ok("6".to_string()));
        assert_eq!(part_2(&["[1,{\"c\":\"red\",\"b\":2},3]".to_string()]),
                   Ok("4".to_string()));
        assert_eq!(part_2(&["{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}".to_string()]),
                   Ok("0".to_string()));
        assert_eq!(part_2(&["[1,\"red\",5]".to_string()]), Ok("6".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 12)?;
        assert_eq!(part_2(&input), Ok("65402".to_string()));
        Ok(())
    }
}
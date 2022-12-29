use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;

pub fn part_1(input: &Vec<String>) -> Result<String, &str> {
    if input.len() != 1 {
        return Err(ERR_VEC_LENGTH)
    }
    let json = input.first().unwrap();
    let mut chars = json.chars();
    let mut sum = 0;
    while let Some(num) = get_next_int(&mut chars)? {
        sum += num;
    }

    Ok(sum.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, &str> {
    if input.len() != 1 {
        return Err(ERR_VEC_LENGTH)
    }
    let line = input.first().unwrap();
    let json = JsonValue::parse(line)?;
    let sum = json.sum_red_aware();
    Ok(sum.to_string())
}

fn get_next_int(chars: &mut Chars) -> Result<Option<i32>, &'static str> {
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
    while let Some(next) = chars.next() {
        if !next.is_numeric() {
            break;
        }
        number_vec.push(next)
    }

    Ok(Some(number_vec.into_iter().collect::<String>().parse().map_err(|_| ERR_INPUT_MALFORMED)?))
}

enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(i32),
}

impl JsonValue {
    fn parse(str: &str) -> Result<JsonValue, &str> {
        Self::parse_chars(&mut str.chars().peekable(), 0)
    }

    fn parse_chars(chars: &mut Peekable<Chars>, debth: usize) -> Result<JsonValue, &'static str> {
        let json_value = match chars.peek().ok_or(ERR_JSON_EMPTY)? {
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
                                return Err(ERR_JSON_MALFORMED)
                            }
                            break
                        }
                        Some(',') => {
                            chars.next();
                            if !comma_expected {
                                return Err(ERR_JSON_MALFORMED)
                            }
                            value_expected = true;
                            comma_expected = false;
                            end_expected = false;
                        }
                        Some(_) => {
                            if !value_expected {
                                return Err(ERR_JSON_MALFORMED)
                            }
                            let key = match Self::parse_chars(chars, debth+1)? {
                                JsonValue::String(str) => str,
                                _ => return Err(ERR_JSON_MALFORMED),
                            };
                            trim(chars);
                            let x = chars.next().ok_or(ERR_JSON_MALFORMED)?;
                            if x != ':' {
                                return Err(ERR_JSON_MALFORMED)
                            }
                            trim(chars);
                            let val = Self::parse_chars(chars, debth+1)?;
                            content.insert(key, val);
                            value_expected = false;
                            comma_expected = true;
                            end_expected = true;
                        }
                        None => return Err(ERR_JSON_MALFORMED),
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
                                return Err(ERR_JSON_MALFORMED)
                            }
                            break
                        }
                        Some(',') => {
                            chars.next();
                            if !comma_expected {
                                return Err(ERR_JSON_MALFORMED)
                            }
                            value_expected = true;
                            comma_expected = false;
                            end_expected = false;
                        }
                        Some(_) => {
                            if !value_expected {
                                return Err(ERR_JSON_MALFORMED)
                            }
                            content.push(Self::parse_chars(chars, debth+1)?);
                            value_expected = false;
                            comma_expected = true;
                            end_expected = true;
                        },
                        None => return Err(ERR_JSON_MALFORMED),
                    }
                };
                Self::Array(content)
            }
            '"' => {
                chars.next();
                read_string(chars).map(|val| Self::String(val))?
            }
            _ => {
                read_int(chars).map(|val| Self::Number(val))?
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
                for (_, elem) in map {
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
                    match other {
                        JsonValue::String(str) => {
                            if str == value {
                                return true
                            }
                        }
                        _ => {}
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

fn read_int(chars: &mut Peekable<Chars>) -> Result<i32, &'static str> {
    let mut tmp = vec![];
    while let Some(val) = chars.peek() {
        match val {
            ',' | ']' | '}' => break,
            _ => tmp.push(chars.next().unwrap()),
        }
    }
    tmp.into_iter().collect::<String>().parse().map_err(|_| ERR_EXPECTED_INT)
}

fn read_string(chars: &mut Peekable<Chars>) -> Result<String, &'static str> {
    let mut tmp = vec![];
    while let Some(val) = chars.next() {
        match val {
            '"' => return Ok(tmp.into_iter().collect()),
            _ => tmp.push(val),
        }
    }
    Err(ERR_STRING_MALFORMED)
}

fn trim(chars: &mut Peekable<Chars>) {
    while let Some(next) = chars.peek() {
        if !next.is_whitespace() {
            break
        }
        chars.next();
    }
}

const ERR_VEC_LENGTH: &str = "The input is expected to be exactly one line";
const ERR_INPUT_MALFORMED: &str = "Input string is malformed";
const ERR_JSON_EMPTY: &str = "Json is empty";
const ERR_EXPECTED_INT: &str = "Json malformed, parsing int failed";
const ERR_STRING_MALFORMED: &str = "Json malformed, string termination missing";
const ERR_JSON_MALFORMED: &str = "Json malformed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["[1,2,3]".to_string()]), Ok("6".to_string()));
        assert_eq!(part_1(&vec!["{\"a\":2,\"b\":4}".to_string()]), Ok("6".to_string()));
        assert_eq!(part_1(&vec!["[[[3]]]".to_string()]), Ok("3".to_string()));
        assert_eq!(part_1(&vec!["{\"a\":{\"b\":4},\"c\":-1}".to_string()]), Ok("3".to_string()));
        assert_eq!(part_1(&vec!["{\"a\":[-1,1]}".to_string()]), Ok("0".to_string()));
        assert_eq!(part_1(&vec!["[-1,{\"a\":1}]".to_string()]), Ok("0".to_string()));
        assert_eq!(part_1(&vec!["[]".to_string()]), Ok("0".to_string()));
        assert_eq!(part_1(&vec!["{}".to_string()]), Ok("0".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_12.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("111754".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["[1,2,3]".to_string()]), Ok("6".to_string()));
        assert_eq!(part_2(&vec!["[1,{\"c\":\"red\",\"b\":2},3]".to_string()]), Ok("4".to_string()));
        assert_eq!(part_2(&vec!["{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}".to_string()]), Ok("0".to_string()));
        assert_eq!(part_2(&vec!["[1,\"red\",5]".to_string()]), Ok("6".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_12.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("65402".to_string()));
        Ok(())
    }
}
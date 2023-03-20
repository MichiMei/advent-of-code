use std::collections::HashSet;
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let mut count = 0;
    for line in input {
        let ip = IPv7::from_str(line).ok_or_else(||
            AoCError::BadInputFormat("Parsing IPv7 failed".to_string())
        )?;
        if ip.is_tls() {
            count += 1;
        }
    }

    Ok(count.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let mut count = 0;
    for line in input {
        let ip = IPv7::from_str(line).ok_or_else(||
            AoCError::BadInputFormat("Parsing IPv7 failed".to_string())
        )?;
        if ip.is_ssl() {
            count += 1;
        }
    }

    Ok(count.to_string())
}

#[derive(Debug)]
struct IPv7<'a> {
    parts: Vec<IPv7Part<'a>>,
}

impl<'a> IPv7<'a> {
    fn from_str(mut str: &'a str) -> Option<Self> {
        let mut parts = vec![];
        let mut hyper = false;

        while !str.is_empty() {
            let delim = if hyper {']'} else {'['};
            if let Some(pos) = str.find(delim) {
                parts.push(IPv7Part::new(&str[..pos], hyper)?);
                str = &str[pos+1..];
            } else {
                parts.push(IPv7Part::new(str, hyper)?);
                str = &str[str.len()..]
            }
            hyper = !hyper
        }
        Some(IPv7{parts})
    }

    fn is_tls(&self) -> bool {
        let mut tls = false;
        for part in self.parts.iter() {
            if part.is_tls() {
                if part.is_hyper() {
                    return false;
                }
                tls = true;
            }
        }
        tls
    }

    fn is_ssl(&self) -> bool {
        let mut abas = HashSet::new();
        let mut babs = HashSet::new();
        for part in self.parts.iter() {
            if part.is_hyper() {
                babs.extend(part.get_babs());
            } else {
                abas.extend(part.get_abas());
            }
        }
        abas.intersection(&babs).next().is_some()
    }
}

#[derive(Debug)]
struct IPv7Part<'a> {
    content: &'a str,
    is_hyper: bool,
}

impl<'a> IPv7Part<'a> {
    fn new(str: &'a str, is_hyper: bool) -> Option<Self> {
        if str.contains('[') || str.contains(']') {
            return None
        }
        Some(IPv7Part{content: str, is_hyper})
    }

    fn is_hyper(&self) -> bool {
        self.is_hyper
    }

    fn is_tls(&self) -> bool {
        if self.content.len() < 4 {
            return false
        }
        let mut chars = self.content.chars();
        let mut prev0 = chars.next().expect("string is >=4 chars long");
        let mut prev1 = chars.next().expect("string is >=4 chars long");
        let mut prev2 = chars.next().expect("string is >=4 chars long");

        for next in chars {
            if prev0 == next && prev1 == prev2 && prev0 != prev1 {
                return true
            }
            prev0 = prev1;
            prev1 = prev2;
            prev2 = next;
        }

        false
    }

    fn get_abas(&self) -> HashSet<(char, char)> {
        let mut res = HashSet::new();
        if self.content.len() < 3 {
            return res
        }
        let mut chars = self.content.chars();
        let mut prev0 = chars.next().expect("content length >= 3");
        let mut prev1 = chars.next().expect("content length >= 3");
        for next in chars {
            if next == prev0 && prev0 != prev1 {
                res.insert((prev0, prev1));
            }
            prev0 = prev1;
            prev1 = next;
        }
        res
    }

    fn get_babs(&self) -> HashSet<(char, char)> {
        self.get_abas().into_iter().map(|x| (x.1, x.0)).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "abba[mnop]qrst".to_string(),
            "abcd[bddb]xyyx".to_string(),
            "aaaa[qwer]tyui".to_string(),
            "ioxxoj[asdfgh]zxcvbn".to_string(),
        ];

        assert_eq!(part_1(&v), Ok("2".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 7)?;
        assert_eq!(part_1(&input), Ok("118".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = vec![
            "aba[bab]xyz".to_string(),
            "xyx[xyx]xyx".to_string(),
            "aaa[kek]eke".to_string(),
            "zazbz[bzb]cdb".to_string(),
        ];

        assert_eq!(part_2(&v), Ok("3".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 7)?;
        assert_eq!(part_2(&input), Ok("260".to_string()));
        Ok(())
    }
}
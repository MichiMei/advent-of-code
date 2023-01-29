use std::fmt::{Display, Formatter};

pub fn part_1(input: &[String]) -> Result<String, &str> {
    if input.len() != 1 {
        return Err(ERR_VEC_LENGTH)
    }
    let mut password = Password::from(input.first().unwrap());
    password.next();
    Ok(password.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, &str> {
    if input.len() != 1 {
        return Err(ERR_VEC_LENGTH)
    }
    let mut password = Password::from(input.first().unwrap());
    password.next();
    password.next();
    Ok(password.to_string())
}

struct Password {
    chars: Vec<char>,
}

impl Password {
    pub fn from(str: &str) -> Self {
        let chars = str.chars().collect();
        Self{chars}
    }

    pub fn next(&mut self) {
        self.add_one();
        while !self.is_valid() {
            self.add_one();
        }
    }

    fn add_one(&mut self) {
        for elem in self.chars.iter_mut().rev() {
            if *elem != 'z' {
                *elem = (*elem as u8 + 1) as char;
                break
            }
            *elem = 'a';
        }
    }

    fn is_valid(&self) -> bool {
        let x = self.contains_street();
        let y = self.contains_bad_char();
        let z = self.contains_doubles();

        x && !y && z
    }

    fn contains_street(&self) -> bool {
        let mut iter = self.chars.iter();
        let mut pre_prev = match iter.next() {
            None => return false,
            Some(v) => *v as u8,
        };
        let mut prev = match iter.next() {
            None => return false,
            Some(v) => *v as u8,
        };

        for curr in iter {
            let curr = *curr as u8;
            if pre_prev+1 == prev && prev+1 == curr {
                return true
            }
            pre_prev = prev;
            prev = curr;
        }
        false
    }

    fn contains_bad_char(&self) -> bool {
        for char in self.chars.iter() {
            match char {
                'i' => return true,
                'o' => return true,
                'l' => return true,
                _ => {}
            }
        }
        false
    }

    fn contains_doubles(&self) -> bool {
        let mut double = None;
        let iter = self.chars.iter();
        let mut prev = None;
        for curr in iter {
            if prev.is_some() && *curr == prev.unwrap() {
                if double.is_none() {
                    double = prev;
                    prev = None;
                    continue;
                } else if double.unwrap() != *curr {
                    return true
                }
            }
            prev = Some(*curr);
        }
        false
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chars.iter().collect::<String>())
    }
}

const ERR_VEC_LENGTH: &str = "The input is expected to be exactly one line";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&["abcdefgh".to_string()]), Ok("abcdffaa".to_string()));
        assert_eq!(part_1(&["ghijklmn".to_string()]), Ok("ghjaabcc".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_11.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("vzbxxyzz".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_11.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("vzcaabcc".to_string()));
        Ok(())
    }

    #[test]
    fn check_password_valid() {
        let p0 = Password::from("hijklmmn");
        assert!(!p0.is_valid());

        let p1 = Password::from("abbceffg");
        assert!(!p1.is_valid());

        let p2 = Password::from("abbcegjk");
        assert!(!p2.is_valid());

        let p3 = Password::from("abcdffaa");
        assert!(p3.is_valid());

        let p4 = Password::from("ghjaabcc");
        assert!(p4.is_valid());

        let p5 = Password::from("vzbxxyzz");
        assert!(p5.is_valid());
    }
}
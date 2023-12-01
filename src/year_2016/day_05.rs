use crate::errors::AoCError;
use crate::md5_collision::{find_hash_collision_parallel, hash};

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("The input is expected to be exactly one line. Found {} lines", input.len())
        ))
    }
    let mut res = String::new();
    let mut start_index = 0;
    for _ in 0..8 {
        if let Some(nonce) =
            find_hash_collision_parallel(&input[0], start_index, 5)? {
            let (char, _) = hash_is_partial_collision(hash(&format!("{}{}", input[0], nonce)))
                .expect("calculated a colliding nonce");
            res = format!("{}{}", res, char);
            start_index = nonce+1;
        } else {
            return Err(AoCError::NoSolutionFoundError(
                format!("Could not find a solution for '{}'", &input[0])
            ))
        }
    }
    Ok(res)
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("The input is expected to be exactly one line. Found {} lines", input.len())
        ))
    }
    let mut password = [None; 8];
    let mut missing_count = 8;
    let mut start_index = 0;
    while missing_count > 0 {
        if let Some(nonce) =
            find_hash_collision_parallel(&input[0], start_index, 5)? {
            let (pos_char, char) = hash_is_partial_collision(hash(&format!("{}{}", input[0], nonce)))
                .expect("calculated a colliding nonce");
            if pos_char.is_numeric() {
                let pos: usize = format!("{}", pos_char).parse().expect("is a numeric -> can be parsed");
                if pos < password.len() && password[pos].is_none() {
                    missing_count -= 1;
                    password[pos] = Some(char);
                }

            }
            if start_index == usize::MAX {
                break
            }
            start_index = nonce+1;
        } else {
            return Err(AoCError::NoSolutionFoundError(
                format!("Could not find a solution for '{}'", &input[0])
            ))
        }
    }
    if missing_count > 0 {
        return Err(AoCError::NoSolutionFoundError(
            format!("Could not find a solution for '{}'", &input[0])
        ))
    }
    let password = password.into_iter().collect::<Option<String>>()
        .expect("missing count == 0 -> all must be Some");

    Ok(password)
}

fn hash_is_partial_collision(hash: [u8; 16]) -> Option<(char, char)> {
    let hex: String = hash.iter().map(|x| format!("{:02x?}", x)).collect();
    /*let hex: String = hash.iter().fold(String::new(), |mut output, x| {
        let _ = write!(output, "{:02x?}", x);
        output
    });*/
    let mut iter = hex.chars();
    for _ in 0..5 {
        let char = iter.next().expect("Must contain Some as 16 bytes is 32 hex chars");
        if char != '0' {
            return None
        }
    }
    let first = iter.next().expect("Must contain Some as 16 bytes is 32 hex chars");
    let second = iter.next().expect("Must contain Some as 16 bytes is 32 hex chars");
    Some((first, second))
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&["abc".to_string()]), Ok("18f47a30".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 5)?;
        assert_eq!(part_1(&input), Ok("1a3099aa".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&["abc".to_string()]), Ok("05ace8e3".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 5)?;
        assert_eq!(part_2(&input), Ok("694190cd".to_string()));
        Ok(())
    }
}
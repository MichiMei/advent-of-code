use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut count = 0;
    for line in input {
        let room = Room::from_line(line)?;
        if room.check_real()? {
            count += room.sector_id;
        }
    }
    Ok(count.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut result = None;
    for line in input {
        let room = Room::from_line(line)?;
        let decrypted = room.decrypt()?;
        if decrypted.contains("north")
            && decrypted.contains("pole") {
            if result.is_none() {
                result = Some(room.sector_id)
            } else {
                return Err(AoCError::MultipleSolutionsFoundError(
                    format!("Found (at least) two solutions:\n{}\n{}",
                            result.expect("result was tested for None -> has to work!"),
                            room.sector_id)
                ))
            }
        }
    }
    if result.is_none() {
        return Err(AoCError::NoSolutionFoundError(String::new()))
    }
    Ok(result.expect("result was tested for None -> has to work!").to_string())
}

struct Room<'a> {
    encrypted_name: &'a str,
    sector_id: usize,
    checksum: &'a str,
}

impl<'a> Room<'a> {
    fn from_line(line: &'a str) -> Result<Self, AoCError<String>> {
        let split0 = line.rfind('-').ok_or_else(|| AoCError::BadInputFormat(
            "No '-' found in the input, each line should consist of \
            <encrypted_room_name>-<sector-id>[<checksum>]".to_string()
        ))?;
        let split1 = line.find('[').ok_or_else(|| AoCError::BadInputFormat(
            "No '[' found in the input, each line should consist of \
            <encrypted_room_name>-<sector-id>[<checksum>]".to_string()
        ))?;
        assert!(split0 < split1);
        let encrypted_name = &line[..split0];
        let sector_id = line[split0+1..split1].parse()
            .map_err(|e| AoCError::BadInputFormat(
            format!("Could not parse sector id, expected a positive number, \
            found {}:\n{}", &line[split0+1..split1], e)
        ))?;
        let checksum = &line[split1+1..line.len()-1];
        Ok(Self {encrypted_name, sector_id, checksum})
    }

    fn check_real(&self) -> Result<bool, AoCError<String>> {
        let offset_a = 'a' as usize;
        let offset_z = 'z' as usize;
        let mut counts = vec![0; offset_z-offset_a+1];
        assert_eq!(counts.len(), 26);
        for char in self.encrypted_name.chars() {
            match char {
                'a'..='z' => {
                    let index = char as usize - offset_a;
                    assert!(index < counts.len());
                    counts[index] += 1;
                }
                '-' => {}
                x => {
                    return Err(AoCError::BadInputFormat(
                        format!("Unexpected character in encrypted room name. \
                        Expected 'a' - 'z' or '-', found '{}'", x)
                    ))
                }
            }
        }
        let mut char_counts = counts.iter().enumerate()
            .map(|(index, count)| {
                let real_index = index+offset_a;
                assert!(real_index <= offset_z);
                (real_index as u8 as char, *count)
            })
            .collect::<Vec<_>>();
        char_counts.sort_by(|(_, val0), (_, val1)| val1.cmp(val0));
        let sorted_chars = char_counts.iter().map(|(char, _)| *char).take(5);

        for (char_enc, char_checksum) in sorted_chars.zip(self.checksum.chars()) {
            if char_enc != char_checksum {
                return Ok(false)
            }
        }
        Ok(true)
    }

    fn decrypt(&self) -> Result<String, AoCError<String>> {
        self.encrypted_name.chars()
            .map(|c| rotate_char(c, self.sector_id))
            .collect::<Result<String, AoCError<String>>>()
    }
}

fn rotate_char(c: char, rotation: usize) -> Result<char, AoCError<String>> {
    if c == '-' {
        return Ok(' ')
    }
    if !c.is_ascii_lowercase() {
        return Err(AoCError::BadInputFormat(format!("Unexpected character in encrypted room name. \
            Expected 'a' - 'z' or '-', found '{}'", c)))
    }
    let alphabet_index = ((c as u8) - b'a') as usize;
    let decrypted_index = (alphabet_index + rotation) % 26;
    let decrypted_char = ((decrypted_index as u8) + b'a') as char;

    Ok(decrypted_char)
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "aaaaa-bbb-z-y-x-123[abxyz]".to_string(),
            "a-b-c-d-e-f-g-h-987[abcde]".to_string(),
            "not-a-real-room-404[oarel]".to_string(),
            "totally-real-room-200[decoy]".to_string(),
        ];

        assert_eq!(part_1(&v), Ok("1514".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_04.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("245102".to_string()));
        Ok(())
    }

    #[test]
    fn check_room_decrypt() -> Result<(), AoCError<String>> {
        let room = Room::from_line("qzmt-zixmtkozy-ivhz-343[test]")?;
        let decrypted = room.decrypt()?;

        assert_eq!(decrypted, "very encrypted name".to_string());
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_04.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("324".to_string()));
        Ok(())
    }
}
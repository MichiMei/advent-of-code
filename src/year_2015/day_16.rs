use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let sues = parse_sues(input)?;
    let prop_str = "children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, \
        vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";

    let correct_props =
        Properties::from(&prop_str.split(", ").collect())?;

    let mut compatible = None;
    for sue in sues.iter() {
        if correct_props.compatible_part_1(&sue.props) {
            if compatible.is_some() {
                return Err(AoCError::MultipleSolutionsFoundError(
                    "Multiple Sue's could be possible".to_string()
                ))
            }
            compatible = Some(sue.index);
        }
    }

    Ok(compatible.ok_or_else(|| AoCError::NoSolutionFoundError(
        "No possible Sue found".to_string()
    ))?.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let sues = parse_sues(input)?;
    let prop_str = "children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, \
        vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";

    let correct_props =
        Properties::from(&prop_str.split(", ").collect())?;

    let mut compatible = None;
    for sue in sues.iter() {
        if correct_props.compatible_part_2(&sue.props) {
            if compatible.is_some() {
                return Err(AoCError::MultipleSolutionsFoundError(
                    "Multiple Sue's could be possible".to_string()
                ))
            }
            compatible = Some(sue.index);
        }
    }

    Ok(compatible.ok_or_else(|| AoCError::NoSolutionFoundError(
        "No possible Sue found".to_string()
    ))?.to_string())
}

fn parse_sues(input: &[String]) -> Result<Vec<Sue>, AoCError<String>> {
    let mut res = vec![];
    for line in input {
        res.push(Sue::from(line)?);
    }
    Ok(res)
}

struct Sue {
    index: u16,
    props: Properties,
}

impl Sue {
    fn from(line: &str) -> Result<Self, AoCError<String>> {

        let end = line.find(':').ok_or_else(|| AoCError::BadInputFormat(
            format!("Unexpected input line.\nExpected 'Sue <index>: {{<key>: <value>, }}'.\n\
            Found '{}'", line)
        ))?;

        let x = &line[4..end];

        let index = x.parse().map_err(|e| AoCError::BadInputFormat(
            format!("Parsing index failed, found '{}'.\n{}", x, e)
        ))?;
        let prop_strings: Vec<&str> = line[end+2..].split(", ").collect();

        let props = Properties::from(&prop_strings)?;

        Ok(Sue{index, props})
    }
}

struct Properties {
    props: [Option<u8>; 10],
}

impl Properties {
    fn from(prop_strings: &Vec<&str>) -> Result<Self, AoCError<String>> {
        let mut props = [None; 10];

        for str in prop_strings {
            let (name, val) = parse_property(str)?;
            match name {
                "children" => props[0] = Some(val),
                "cats" => props[1] = Some(val),
                "samoyeds" => props[2] = Some(val),
                "pomeranians" => props[3] = Some(val),
                "akitas" => props[4] = Some(val),
                "vizslas" => props[5] = Some(val),
                "goldfish" => props[6] = Some(val),
                "trees" => props[7] = Some(val),
                "cars" => props[8] = Some(val),
                "perfumes" => props[9] = Some(val),
                x => {
                    return Err(AoCError::BadInputFormat(
                        format!("Unexpected property '{}'", x)
                    ))
                }
            }
        }

        Ok(Self{props})
    }

    fn compatible_part_1(&self, other: &Self) -> bool {
        for (s, o) in self.props.iter().zip(other.props.iter()) {
            if s.is_some() && o.is_some() && s.unwrap() != o.unwrap() {
                return false
            }
        }
        true
    }

    fn compatible_part_2(&self, other: &Self) -> bool {
        for (index, (s, o)) in self.props.iter().
            zip(other.props.iter()).enumerate() {
            if s.is_some() && o.is_some() {
                match index {
                    1 | 7 => {
                        if s.unwrap() >= o.unwrap() {
                            return false
                        }
                    }
                    3 | 6 => {
                        if s.unwrap() <= o.unwrap() {
                            return false
                        }
                    }
                    _ => {
                        if s.unwrap() != o.unwrap() {
                            return false
                        }
                    }
                }
            }
        }
        true
    }
}

fn parse_property(str: &str) -> Result<(&str, u8), AoCError<String>> {
    let words: Vec<&str> = str.split(": ").collect();
    if words.len() != 2 {
        return Err(AoCError::BadInputFormat(
            format!("Parsing property failed.\nExpected '<property-name>: <property-value>.\n\
            Found '{}'", str)
        ))
    }
    Ok((words[0], words[1].parse().map_err(|e| AoCError::BadInputFormat(
        format!("Parsing number failed. Found '{}'.\n{}", words[1], e)
    ))?))
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 16)?;
        assert_eq!(part_1(&input), Ok("40".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2015, 16)?;
        assert_eq!(part_2(&input), Ok("241".to_string()));
        Ok(())
    }
}
pub fn part_1(input: &Vec<String>) -> Result<String, &str> {
    let sues = parse_sues(input)?;
    let prop_str = "children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, \
        vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";

    let correct_props =
        Properties::from(&prop_str.split(", ").collect())?;

    let mut compatible = None;
    for sue in sues.iter() {
        if correct_props.compatible_part_1(&sue.props) {
            if compatible.is_some() {
                return Err(ERR_MULTIPLE_COMPATIBLE)
            }
            compatible = Some(sue.index);
        }
    }

    Ok(compatible.ok_or(ERR_NO_COMPATIBLE)?.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, &str> {
    let sues = parse_sues(input)?;
    let prop_str = "children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, \
        vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";

    let correct_props =
        Properties::from(&prop_str.split(", ").collect())?;

    let mut compatible = None;
    for sue in sues.iter() {
        if correct_props.compatible_part_2(&sue.props) {
            if compatible.is_some() {
                return Err(ERR_MULTIPLE_COMPATIBLE)
            }
            compatible = Some(sue.index);
        }
    }

    Ok(compatible.ok_or(ERR_NO_COMPATIBLE)?.to_string())
}

fn parse_sues(input: &Vec<String>) -> Result<Vec<Sue>, &str> {
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
    fn from(line: &str) -> Result<Self, &str> {

        let end = line.find(':').ok_or(ERR_INPUT_MALFORMED)?;

        let x = &line[4..end];

        let index = x.parse().map_err(|_| ERR_INPUT_MALFORMED)?;
        let prop_strings: Vec<&str> = (&line[end+2..]).split(", ").collect();

        let props = Properties::from(&prop_strings)?;

        Ok(Sue{index, props})
    }
}

struct Properties {
    props: [Option<u8>; 10],
}

impl Properties {
    fn from(prop_strings: &Vec<&str>) -> Result<Self, &'static str> {
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
                _ => return Err(ERR_INPUT_MALFORMED)
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

fn parse_property(str: &str) -> Result<(&str, u8), &'static str> {
    let words: Vec<&str> = str.split(": ").collect();
    if words.len() != 2 {
        return Err(ERR_INPUT_MALFORMED)
    }
    Ok((words[0], words[1].parse().map_err(|_| ERR_INPUT_MALFORMED)?))
}

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";
const ERR_MULTIPLE_COMPATIBLE: &str = "Multiple Sues could be right";
const ERR_NO_COMPATIBLE: &str = "No right Sues found";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_16.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("40".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_16.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("241".to_string()));
        Ok(())
    }
}
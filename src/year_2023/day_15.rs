use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Input has to be a single line. Has {} lines.", input.len())))
    }
    input[0].split(',')
        .map(hash)
        .sum::<Result<usize, _>>()
        .map(|value| value.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            format!("Input has to be a single line. Has {} lines.", input.len())))
    }
    let mut hash_map = HashMap::new();
    for str in input[0].split(',') {
        hash_map.execute_instruction(str)?;
    }
    Ok(hash_map.count().to_string())
}

fn hash(str: &str) -> Result<usize, AoCError<String>> {
    let mut result = 0;
    for c in str.chars() {
        if !c.is_ascii() {
            return Err(AoCError::BadInputFormat("Input contains non ascii character".to_string()));
        }
        let value = c as u8;
        result += value as usize;
        result *= 17;
        result %= 256;
    }
    Ok(result)
}

struct HashMap {
    spaces: [Vec<(String, u8)>; 256],
}

impl HashMap {
    fn new() -> Self {
        let spaces = std::iter::repeat_with(Vec::new)
            .take(256)
            .collect::<Vec<_>>()
            .try_into()
            .expect("try_into only fails if sizes differ");
        Self { spaces }
    }

    fn execute_instruction(&mut self, str: &str) -> Result<(), AoCError<String>> {
        match Instruction::parse(str)? {
            Instruction::Insert(label, value) => {
                let index = hash(&label)?;
                if let Some(prev) =  self.spaces[index].iter_mut()
                    .find(|(prev_label, _)| &label == prev_label) {
                    prev.1 = value;
                } else {
                    self.spaces[index].push((label, value));
                }
            }
            Instruction::Remove(label) => {
                let index = hash(&label)?;
                if let Some((pos, _)) = self.spaces[index].iter()
                    .enumerate()
                    .find(|(_, (prev_label, _))| &label == prev_label) {
                    self.spaces[index].remove(pos);
                }
            }
        }
        Ok(())
    }

    fn count(&self) -> usize {
        self.spaces.iter()
            .enumerate()
            .flat_map(|(box_index, list)| list.iter()
                .enumerate()
                .map(move |(list_index, (_, value))| (*value as usize)*(list_index+1)*(box_index+1)))
            .sum()
    }
}

enum Instruction {
    Insert(String, u8),
    Remove(String),
}

impl Instruction {
    fn parse(str: &str) -> Result<Self, AoCError<String>> {
        if let Some(pos) = str.find('=') {
            let label = str[..pos].to_string();
            let value = str[pos+1..].parse()
                .map_err(|e| AoCError::BadInputFormat(
                    format!("Parsing Instruction::Insert value failed: '{}'. {}", str, e)))?;
            Ok(Self::Insert(label, value))
        } else if let Some(pos) = str.find('-') {
            if pos != str.len()-1 {
                return Err(AoCError::BadInputFormat(
                    format!("Parsing Instruction::Remove failed. After '-' the instruction should \
                    end. Found '{}'.", str)))
            }
            let label = str[..pos].to_string();
            Ok(Self::Remove(label))
        } else {
            return Err(AoCError::BadInputFormat(format!("Parsing instruction '{}' failed.", str)))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec!["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()]
    }

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["HASH".to_string()]), Ok("52".to_string()));
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("1320".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 15)?;
        assert_eq!(part_1(&input), Ok("517015".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("145".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 15)?;
        assert_eq!(part_2(&input), Ok("286104".to_string()));
        Ok(())
    }
}
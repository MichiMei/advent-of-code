use std::fmt::{Display, Formatter};
use crate::errors::AoCError;
use crate::output::bool_slice_to_string;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let mut screen = Screen::new(50, 6);
    for line in input {
        let instruction = Instruction::parse(line)?;
        screen.execute(instruction);
    }
    Ok(screen.count_pixels().to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let mut screen = Screen::new(50, 6);
    for line in input {
        let instruction = Instruction::parse(line)?;
        screen.execute(instruction);
    }
    Ok(screen.to_string())
}

struct Screen {
    pixels: Vec<Vec<bool>>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let line = vec![false; width];
        let pixels = vec![line; height];
        Self{pixels}
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Rect(width, height) => self.rect(width, height),
            Instruction::RotateRow(index, shifts) => self.rotate_row(index, shifts),
            Instruction::RotateColumn(index, shifts) => self.rotate_column(index, shifts),
        }
    }

    pub fn count_pixels(&self) -> usize {
        let mut count = 0;
        for line in self.pixels.iter() {
            for elem in line.iter() {
                if *elem {
                    count += 1;
                }
            }
        }
        count
    }

    fn rect(&mut self, width: usize, height: usize) {
        for line in self.pixels[0..height].iter_mut() {
            for pixel in line[0..width].iter_mut() {
                *pixel = true;
            }
        }
    }

    fn rotate_row(&mut self, index: usize, shifts: usize) {
        let len = self.pixels[index].len();
        let mut new_line = vec![false; len];
        for (index, val) in self.pixels[index].iter().enumerate() {
            new_line[(index+shifts)%len] = *val;
        }
        self.pixels[index] = new_line;
    }

    fn rotate_column(&mut self, index: usize, shifts: usize) {
        let len = self.pixels.len();
        let mut column = vec![false; len];
        for (row_index, line) in self.pixels.iter().enumerate() {
            column[(row_index+shifts)%len] = line[index];
        }

        for (line, elem) in self.pixels.iter_mut().zip(column.iter()) {
            line[index] = *elem;
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.pixels.iter() {
            write!(f, "{}", bool_slice_to_string(line)).unwrap();
            writeln!(f).unwrap();
        }
        write!(f, "")
    }
}

enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl Instruction {
    pub fn parse(line: &str) -> Result<Self, AoCError<String>> {
        let words: Vec<&str> = line.split(' ').collect();
        match words[0] {
            "rect" => Self::parse_rect(&words[1..]),
            "rotate" => Self::parse_rotate(&words[1..]),
            x => Err(AoCError::BadInputFormat(
                format!("Unexpected instruction '{}'. Only 'rect' and 'rotate' supported.", x)))
        }
    }

    fn parse_rect(words: &[&str]) -> Result<Self, AoCError<String>> {
        if words.len() != 1 {
            return Err(AoCError::BadInputFormat(
                "To many arguments in instruction. Expected 'rect <width>x<height>'".to_string()))
        }
        let dimensions_str: Vec<&str> = words[0].split('x').collect();
        if dimensions_str.len() != 2 {
            return Err(AoCError::BadInputFormat(
                "Unexpected 'rect' format. Expected 'rect <width>x<height>'".to_string()))
        }
        let width = dimensions_str[0].parse()
            .map_err(|e|
                AoCError::BadInputFormat(format!("Parsing width failed: {}", e)))?;
        let height = dimensions_str[1].parse()
            .map_err(|e|
                AoCError::BadInputFormat(format!("Parsing height failed: {}", e)))?;

        Ok(Self::Rect(width, height))
    }

    fn parse_rotate(words: &[&str]) -> Result<Self, AoCError<String>> {
        if words.len() != 4 {
            return Err(AoCError::BadInputFormat(
                "To many arguments in instruction. Expected 'rotate [row y=|column x=]<index> \
                <shifts>'".to_string()))
        }
        let index_str: Vec<&str> = words[1].split('=').collect();
        if index_str.len() != 2 {
            return Err(AoCError::BadInputFormat(
                "Unexpected 'rotate' format. Expected 'rotate [row y=|column x=]<index> \
                <shifts>'".to_string()))
        }
        let index = index_str[1].parse().map_err(|e|
            AoCError::BadInputFormat(format!("Parsing index failed: {}", e)))?;

        let shifts = words[3].parse().map_err(|e|
            AoCError::BadInputFormat(format!("Parsing shifts failed: {}", e)))?;

        match words[0] {
            "row" => Ok(Self::RotateRow(index, shifts)),
            "column" => Ok(Self::RotateColumn(index, shifts)),
            _ => Err(AoCError::BadInputFormat(
                "Unexpected 'rotate' format. Expected 'rotate [row y=|column x=]<index> \
                <shifts>'".to_string())),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Rect(width, height) => {
                write!(f, "rect {}x{}", width, height)
            }
            Instruction::RotateRow(index, shifts) => {
                write!(f, "rotate row y={} {}", index, shifts)
            }
            Instruction::RotateColumn(index, shifts) => {
                write!(f, "rotate column x={} {}", index, shifts)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() -> Result<(), AoCError<String>> {
        let v = vec![
            "rect 3x2".to_string(),
            "rotate column x=1 by 1".to_string(),
            "rotate row y=0 by 4".to_string(),
            "rotate column x=1 by 1".to_string(),
        ];

        let mut screen = Screen::new(7, 3);
        for line in v {
            let instruction = Instruction::parse(&line)?;
            screen.execute(instruction);
        }

        assert_eq!(screen.count_pixels(), 6);

        Ok(())
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_08.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("128".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_08.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        let expected = "\
            ####..##...##..###...##..###..#..#.#...#.##...##..\n\
            #....#..#.#..#.#..#.#..#.#..#.#..#.#...##..#.#..#.\n\
            ###..#..#.#..#.#..#.#....#..#.####..#.#.#..#.#..#.\n\
            #....#..#.####.###..#.##.###..#..#...#..####.#..#.\n\
            #....#..#.#..#.#.#..#..#.#....#..#...#..#..#.#..#.\n\
            ####..##..#..#.#..#..###.#....#..#...#..#..#..##..\n\
            ";

        assert_eq!(part_2(&input), Ok(expected.to_string()));
        Ok(())
    }
}
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength("Expected exactly one input line containing the \
            string representation of the first row".to_string()))
    }
    let row = Row::parse(&input[0])?;
    let count = calculate_safe(row, 40);
    Ok(count.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength("Expected exactly one input line containing the \
            string representation of the first row".to_string()))
    }
    let row = Row::parse(&input[0])?;
    let count = calculate_safe(row, 400000);
    Ok(count.to_string())
}

fn calculate_safe(mut row: Row, length: usize) -> usize {
    let mut count = row.count_safe();
    for _ in 1..length {
        row = row.create_next_row();
        count += row.count_safe();
    }
    count
}

struct Row {
    tiles: Vec<FloorTile>,
}

impl Row {
    pub fn parse(line: &str) -> Result<Self, AoCError<String>> {
        let mut tiles = vec![];
        for char in line.chars() {
            tiles.push(FloorTile::parse(char)?);
        }
        Ok(Self{tiles})
    }

    pub fn create_next_row(&self) -> Row {
        let mut tiles = vec![];
        if self.tiles.is_empty() {
            return Self{tiles}
        }
        let mut iter = self.tiles.iter();
        let mut left = &FloorTile::Safe;
        let mut middle = iter.next().expect("Row has to have at least one element");
        for right in iter {
            let new_tile = FloorTile::tile_from(left, right);
            tiles.push(new_tile);
            left = middle;
            middle = right;
        }
        let right = &FloorTile::Safe;
        let new_tile = FloorTile::tile_from(left, right);
        tiles.push(new_tile);

        assert_eq!(self.tiles.len(), tiles.len());
        Self{tiles}
    }

    pub fn count_safe(&self) -> usize {
        self.tiles.iter().map(|tile| tile.count_safe()).sum()
    }
}

#[derive(Eq, PartialEq)]
enum FloorTile {
    Safe,
    Trap,
}

impl FloorTile {
    pub fn parse(c: char) -> Result<Self, AoCError<String>> {
        match c {
            '.' => Ok(FloorTile::Safe),
            '^' => Ok(FloorTile::Trap),
            c => Err(AoCError::BadInputFormat(
                format!("Unexpected char in input. Only '.' and '^' supported. Found '{}'", c))),
        }
    }

    pub fn tile_from(left: &Self, right: &Self) -> Self {
        if left != right {
            return Self::Trap
        }
        Self::Safe
    }

    pub fn count_safe(&self) -> usize {
        match self {
            FloorTile::Safe => 1,
            FloorTile::Trap => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() -> Result<(), AoCError<String>> {
        let row = Row::parse("..^^.")?;
        assert_eq!(calculate_safe(row, 3), 6);

        let row = Row::parse(".^^.^.^^^^")?;
        assert_eq!(calculate_safe(row, 10), 38);

        Ok(())
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_18.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("2005".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_18.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("20008491".to_string()));
        Ok(())
    }
}
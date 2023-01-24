pub fn part_1(input: &Vec<String>) -> Result<String, &str> {
    if input.len() != 1 {
        return Err(ERR_INPUT_MALFORMED)
    }
    let index = parse_input(&input[0])?;

    let res = get_code(index);

    Ok(res.to_string())
}

pub fn part_2(_: &Vec<String>) -> Result<String, &str> {
    Ok("Merry Christmas!".to_string())
}

fn parse_input(line: &str) -> Result<(usize, usize), &str> {
    let words: Vec<&str> = line.split(" ").collect();
    assert_eq!(words.len(), 19);
    let mut row_str = words[16];
    row_str = &row_str[..row_str.len()-1];
    let mut col_str = words[18];
    col_str = &col_str[..col_str.len()-1];

    let row = row_str.parse().map_err(|_| ERR_INPUT_MALFORMED)?;
    let col = col_str.parse().map_err(|_| ERR_INPUT_MALFORMED)?;

    Ok((row, col))
}

fn get_code(goal: (usize, usize)) -> u32 {
    let mut index = (1,1);
    let mut code = 20151125;

    while index != goal {
        index = next_index(index);
        code = next_code(code);
    }

    code
}

fn next_code(current: u32) -> u32 {
    let product = (current as u64) * 252533u64;
    let remainder = product % 33554393;
    remainder as u32
}

fn next_index((row, col): (usize, usize)) -> (usize, usize) {
    if row == 1 {
        return (col+1, row)
    }
    (row-1, col+1)
}

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    const TABLE: [[i32; 6]; 6] = [
        [20151125, 18749137, 17289845, 30943339, 10071777, 33511524],
        [31916031, 21629792, 16929656,  7726640, 15514188,  4041754],
        [16080970,  8057251,  1601130,  7981243, 11661866, 16474243],
        [24592653, 32451966, 21345942,  9380097, 10600672, 31527494],
        [   77061, 17552253, 28094349,  6899651,  9250759, 31663883],
        [33071741,  6796745, 25397450, 24659492,  1534922, 27995004],
    ];

    #[test]
    fn check_examples_part_1() {
        for (row_index, row) in TABLE.iter().enumerate() {
            for (col_index, expected) in row.iter().enumerate() {
                let input = vec![
                    format!("To continue, please consult the code grid in the manual.  \
                    Enter the code at row {}, column {}.", row_index+1, col_index+1),
                ];

                assert_eq!(part_1(&input), Ok(expected.to_string()));
            }
        }
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_25.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("19980801".to_string()));
        Ok(())
    }
}
use crate::errors::AoCError;
use crate::year_2017::lib_2017::knot_hash::KnotHash;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the 'key string'.".to_string()))
    }
    Ok(count_used_on_disk(&input[0]).to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength(
            "Expected a single line containing the 'key string'.".to_string()))
    }
    let grid = create_grid(&input[0]);
    Ok(count_components(grid).to_string())
}

fn count_used_on_disk(input: &str) -> u32 {
    (0..128)
        .map(|row_index| calculate_disk_hash(input, row_index))
        .map(|hash| count_used_in_row(&hash))
        .sum()
}

fn calculate_disk_hash(key_string: &str, disk_index: usize) -> Vec<u8> {
    let input = format!("{}-{}", key_string, disk_index);
    let mut knot_hash = KnotHash::new(255);
    knot_hash.complete_hash(&input);
    knot_hash.get_dense_hash_bytes()
}

fn count_used_in_row(hash: &[u8]) -> u32 {
    hash.iter().map(|byte| byte.count_ones()).sum()
}

fn create_grid(input: &str) -> Vec<Vec<bool>> {
    (0..128).map(|index| create_row(input, index)).collect()
}

fn create_row(input: &str, disk_index: usize) -> Vec<bool> {
    let hash = calculate_disk_hash(input, disk_index);
    hash.iter().flat_map(|b| byte_to_bool_vec(*b)).collect()
}

fn byte_to_bool_vec(mut byte: u8) -> Vec<bool> {
    let mut res = vec![];
    for _ in 0..8 {
        if byte%2 == 1 {
            res.push(true);
        } else {
            res.push(false);
        }
        byte >>= 1;
    }
    res.reverse();
    res
}

fn count_components(mut grid: Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    let mut p = (0, 0);
    while let Some(next) = next(&p) {
        p = next;
        if grid[p.1][p.0] {
            handle_component(&mut grid, p);
            count += 1;
        }
    }
    count
}

fn handle_component(grid: &mut [Vec<bool>], p: Point) {
    let mut queue = vec![p];
    while !queue.is_empty() {
        let p = queue.pop().expect("Checked by while");
        grid[p.1][p.0] = false;
        if p.0 > 0 && grid[p.1][p.0-1] {
            queue.push((p.0-1, p.1))
        }
        if p.1 > 0 && grid[p.1-1][p.0] {
            queue.push((p.0, p.1-1))
        }
        if p.0 < 127 && grid[p.1][p.0+1] {
            queue.push((p.0+1, p.1))
        }
        if p.1 < 127 && grid[p.1+1][p.0] {
            queue.push((p.0, p.1+1))
        }
    }
}

type Point = (usize, usize);

fn next(p: &Point) -> Option<Point> {
    if p.0 == 127 {
        if p.1 == 127 {
            None
        } else {
            Some((0, p.1+1))
        }
    } else {
        Some((p.0+1, p.1))
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        assert_eq!(part_1(&vec!["flqrgnkx".to_string()]), Ok("8108".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_14.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("8194".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        assert_eq!(part_2(&vec!["flqrgnkx".to_string()]), Ok("1242".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_14.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("1141".to_string()));
        Ok(())
    }
}
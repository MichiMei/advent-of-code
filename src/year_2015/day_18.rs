use std::fmt::{Display, Formatter};
use std::mem::swap;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut lights = Lights::from(input)?;
    let mut tmp = Lights::new(lights.grid.len(), lights.grid[0].len());

    let lights_ref = &mut lights;
    let tmp_ref = &mut tmp;

    for _ in 0..100 {
        lights_ref.next(tmp_ref);
        swap(lights_ref, tmp_ref);
    }

    Ok(lights_ref.count_on().to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut lights = Lights::from(input)?;
    let mut tmp = Lights::new(lights.grid.len(), lights.grid[0].len());

    let lights_ref = &mut lights;
    let tmp_ref = &mut tmp;

    lights_ref.switch_corners_on();

    for _ in 0..100 {
        lights_ref.next(tmp_ref);
        swap(lights_ref, tmp_ref);
        lights_ref.switch_corners_on();
    }

    Ok(lights_ref.count_on().to_string())
}

struct Lights {
    grid: Vec<Vec<bool>>,
}

impl Lights {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![false; width]; height];
        Self{grid}
    }

    fn from(input: &[String]) -> Result<Self, AoCError<String>> {
        let mut grid = vec![];
        for line in input {
            let mut row = vec![];
            for c in line.chars() {
                match c {
                    '.' => row.push(false),
                    '#' => row.push(true),
                    c => return Err(AoCError::BadInputFormat(format!(
                        "Found unexpected character '{}'. Only '#' and '.' allowed!", c
                    ))),
                }
            }
            grid.push(row)
        }
        Ok(Self{grid})
    }

    fn next(&self, next: &mut Self) {
        assert_eq!(self.grid.len(), next.grid.len());
        for y_index in 0..self.grid.len() {
            assert_eq!(self.grid[y_index].len(), next.grid[y_index].len());
            for x_index in 0..self.grid[y_index].len() {
                let neighbors = self.count_neighbors(x_index, y_index);
                if self.grid[y_index][x_index] {
                    // light is on -> 2 | 3 for on
                    if neighbors == 2 || neighbors == 3 {
                        next.grid[y_index][x_index] = true;
                    } else {
                        next.grid[y_index][x_index] = false;
                    }
                } else {
                    // light is off -> 3 for on
                    if neighbors == 3 {
                        next.grid[y_index][x_index] = true;
                    } else {
                        next.grid[y_index][x_index] = false;
                    }
                }
            }
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        let up = y > 0;
        let down = y+1 < self.grid.len();
        let left = x > 0;
        let right = x+1 < self.grid[y].len();

        if up && self.grid[y-1][x] {
            count += 1;
        }
        if up && left && self.grid[y-1][x-1] {
            count += 1;
        }
        if up && right && self.grid[y-1][x+1] {
            count += 1;
        }
        if left && self.grid[y][x-1] {
            count += 1;
        }
        if right && self.grid[y][x+1] {
            count += 1;
        }
        if down && self.grid[y+1][x] {
            count += 1;
        }
        if down && left && self.grid[y+1][x-1] {
            count += 1;
        }
        if down && right && self.grid[y+1][x+1] {
            count += 1;
        }

        count
    }

    fn count_on(&self) -> usize {
        let mut count = 0;
        for line in self.grid.iter() {
            for elem in line.iter() {
                if *elem {
                    count += 1;
                }
            }
        }

        count
    }

    fn switch_corners_on(&mut self) {
        let width = self.grid.len()-1;
        let height = self.grid[0].len()-1;

        self.grid[0][0] = true;
        self.grid[0][height] = true;
        self.grid[width][0] = true;
        self.grid[width][height] = true;
    }
}

impl Display for Lights {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter() {
            for elem in line.iter() {
                if *elem {
                    write!(f, "#").unwrap();
                } else {
                    write!(f, ".").unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod test {
    use std::mem::swap;
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() -> Result<(), AoCError<String>> {
        let v = vec![
            ".#.#.#".to_string(),
            "...##.".to_string(),
            "#....#".to_string(),
            "..#...".to_string(),
            "#.#..#".to_string(),
            "####..".to_string(),
        ];
        let mut lights = Lights::from(&v)?;
        let mut tmp = Lights::new(lights.grid.len(), lights.grid[0].len());

        let lights_ref = &mut lights;
        let tmp_ref = &mut tmp;

        for _ in 0..4 {
            lights_ref.next(tmp_ref);
            swap(lights_ref, tmp_ref);
        }

        assert_eq!(lights_ref.count_on(), 4);
        Ok(())
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_18.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("814".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() -> Result<(), AoCError<String>>{
        let v = vec![
            ".#.#.#".to_string(),
            "...##.".to_string(),
            "#....#".to_string(),
            "..#...".to_string(),
            "#.#..#".to_string(),
            "####..".to_string(),
        ];
        let mut lights = Lights::from(&v)?;
        let mut tmp = Lights::new(lights.grid.len(), lights.grid[0].len());

        let lights_ref = &mut lights;
        let tmp_ref = &mut tmp;

        lights_ref.switch_corners_on();

        for _ in 0..5 {
            lights_ref.next(tmp_ref);
            swap(lights_ref, tmp_ref);
            lights_ref.switch_corners_on();
        }

        assert_eq!(lights_ref.count_on(), 17);
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_18.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("924".to_string()));
        Ok(())
    }
}
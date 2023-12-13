use std::collections::HashSet;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    /*let grids = input.split(|line| line.is_empty())
        .map(Grid::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let values = grids.iter().enumerate()
        .map(|(index, grid)| grid.get_value()
            .map_err(|e| AoCError::NoSolutionFoundError(
                format!("{} in {}", e, index))))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(values.iter()
        .sum::<usize>()
        .to_string())
     */
    let grids = input.split(|line| line.is_empty())
        .map(Grid::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let values = grids.iter().enumerate()
        .map(|(index, grid)| grid.get_value_with_smudge(0)
            .map_err(|e| AoCError::NoSolutionFoundError(
                format!("{} in {}", e, index))))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(values.iter()
        .sum::<usize>()
        .to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let grids = input.split(|line| line.is_empty())
        .map(Grid::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let values = grids.iter().enumerate()
        .map(|(index, grid)| grid.get_value_with_smudge(1)
            .map_err(|e| AoCError::NoSolutionFoundError(
                format!("{} in {}", e, index))))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(values.iter()
        .sum::<usize>()
        .to_string())
}

struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    fn parse(input: &[String]) -> Result<Self, AoCError<String>> {
        if input.is_empty() {
            return Err(AoCError::BadInputFormat("Empty grid is not supported".to_string()))
        }
        let grid = input.iter()
            .map(|line| line.chars()
                .map(Tile::parse)
                .collect::<Result<_, _>>())
            .collect::<Result<_, _>>()?;
        Ok(Self { grid })
    }

    fn find_horizontal_mirrors_with_smudge(&self, smudges: usize) -> Vec<usize> {
        let mut res = HashSet::new();
        let len = self.grid.len();

        let top_to_bottom = self.grid[1..].iter()
            .enumerate()
            .map(|(index, row)|
                (index, Self::compare_rows_with_smudge(row, &self.grid[0])))
            .filter(|(_, s)| *s <= smudges)
            .map(|(index, s)| (index+1, s))
            .collect::<Vec<_>>();
        for (index, mut smudge_count) in top_to_bottom {
            let mut correct = true;
            for (top, bottom) in (1..index).zip((1..index).rev()) {
                if top > bottom {
                    break;
                }
                smudge_count +=
                    Self::compare_rows_with_smudge(&self.grid[top], &self.grid[bottom]);
                if smudge_count > smudges {
                    correct = false;
                    break;
                }
            }
            if smudge_count == smudges && (index+1)%2 == 0 {
                assert!(correct);
                assert_eq!((index+1)%2, 0);
                res.insert((index+1)/2);
            }
        }

        let bottom_to_top = self.grid[0..len-1].iter()
            .enumerate()
            .map(|(index, row)|
                (index, Self::compare_rows_with_smudge(row, self.grid.last()
                    .expect("grid can't be empty"))))
            .filter(|(_, s)| *s <= smudges)
            .collect::<Vec<_>>();
        for (index, mut smudge_count) in bottom_to_top {
            let mut correct = true;
            for (top, bottom) in (index+1..len).zip((index..len-1).rev()) {
                if top >= bottom {
                    break;
                }
                smudge_count +=
                    Self::compare_rows_with_smudge(&self.grid[top], &self.grid[bottom]);
                if smudge_count > smudges {
                    correct = false;
                    break;
                }
            }
            if smudge_count == smudges && (len-index)%2 == 0{
                assert!(correct);
                res.insert((len-index)/2+index);
            }
        }
        res.into_iter().collect()
    }

    fn compare_rows_with_smudge(first: &[Tile], other: &[Tile]) -> usize {
        first.iter().zip(other.iter())
            .filter(|(f, o)| f != o)
            .count()
    }

    fn find_vertical_mirrors_with_smudge(&self, smudges: usize) -> Vec<usize> {
        let len = self.grid[0].len();
        let mut possible = (0..len-1).map(|i| (i, 0usize)).collect::<Vec<_>>();
        for row in self.grid.iter() {
            if possible.is_empty() {
                return vec![]
            }
            let mut tmp = vec![];
            for (index, mut sc) in possible {
                let mut correct = true;
                for (left, right) in (0..=index).rev().zip(index+1..len) {
                    if row[left] != row[right] {
                        sc += 1;
                    }
                    if sc > smudges {
                        correct = false;
                        break;
                    }
                }
                if sc <= smudges {
                    assert!(correct);
                    tmp.push((index, sc));
                }
            }
            possible = tmp;
        }
        possible.into_iter()
            .filter(|(_, sc)| *sc == smudges)
            .map(|(val, _)| val+1).collect()
    }

    fn get_value_with_smudge(&self, smudges: usize) -> Result<usize, AoCError<String>> {
        let vertical = self.find_vertical_mirrors_with_smudge(smudges);
        let horizontal = self.find_horizontal_mirrors_with_smudge(smudges);
        if vertical.len() + horizontal.len() > 1 {
            return Err(AoCError::MultipleSolutionsFoundError(
                format!("Found too many mirrors: {} + {}", vertical.len(), horizontal.len())))
        }
        if vertical.is_empty() && horizontal.is_empty() {
            return Err(AoCError::NoSolutionFoundError("Found no solution".to_string()))
        }
        if let Some(value) = vertical.first() {
            Ok(*value)
        } else if let Some(value) = horizontal.first() {
            Ok(value*100)
        } else {
            panic!("Unreachable as vertical or horizontal is not empty")
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn parse(c: char) -> Result<Self, AoCError<String>> {
        match c {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rock),
            c => Err(AoCError::BadInputFormat(
                format!("Only '.' and '#' supported, found '{}'", c))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
            "".to_string(),
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("405".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 13)?;
        assert_eq!(part_1(&input), Ok("33975".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("400".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2023, 13)?;
        assert_eq!(part_2(&input), Ok("29083".to_string()));
        Ok(())
    }
}
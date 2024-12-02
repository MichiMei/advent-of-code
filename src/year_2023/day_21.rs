use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use crate::errors::{AoCError, AoCResult};
use crate::geometrics::{Direction, Grid, Parsable, Point};

pub fn part_1(input: &[String]) -> AoCResult<String> {
    let grid = Grid::parse(input)?;
    let tmp = grid.get_all_positions_of(&Tile::Start);
    if tmp.len() != 1 {
        return Err(AoCError::BadInputFormat("Found multiple start tiles.".to_string()))
    }
    let start = tmp[0];
    Ok(get_reachable(&grid, start, 64).to_string())
}

pub fn part_2(input: &[String]) -> AoCResult<String> {
    let grid = Grid::parse(input)?;
    let tmp = grid.get_all_positions_of(&Tile::Start);
    if tmp.len() != 1 {
        return Err(AoCError::BadInputFormat("Found multiple start tiles.".to_string()))
    }
    let start = tmp[0];
    if !check_start_and_size_constrain(&grid, start) {
        return Err(AoCError::NoSolutionFoundError(
            "Faulty constraint: Expected start point to be exactly centered.".to_string()))
    }
    if !check_strait_line_constrain(&grid, start) {
        return Err(AoCError::NoSolutionFoundError(
            "Faulty constraint: Expected a strait horizontal and vertical line without rocks \
            from start".to_string()))
    }
    if !check_empty_border_constrain(&grid) {
        return Err(AoCError::NoSolutionFoundError(
            "Faulty constraint: Expected a rock free border around the grid.".to_string()))
    }
    Ok(reachable_wrapping(&grid, start, 26501365).to_string())
}

fn get_valid_neighbors(grid: &Grid<Tile>, pos: Point<usize>) -> Vec<Point<usize>> {
    Direction::get_all_neighbors(&pos).into_iter()
        .filter(|point| {
            if let Some(tile) = grid.get_tile(point) {
                tile != &Tile::Rock
            } else {
                false
            }
        })
        .collect()
}

fn get_reachable(grid: &Grid<Tile>, start: Point<usize>, steps: usize) -> usize {
    let mut reachable_even = HashSet::new();
    let mut reachable_odd = HashSet::new();

    let mut pq = VecDeque::new();
    pq.push_back((start, 0usize));
    while let Some((curr_point, curr_steps)) = pq.pop_front() {
        if curr_steps > steps {
            break;
        }
        let reachable = if curr_steps%2 == 0 {
            &mut reachable_even
        } else {
            &mut reachable_odd
        };
        if reachable.contains(&curr_point) {
            continue
        }
        reachable.insert(curr_point);
        get_valid_neighbors(grid, curr_point).into_iter()
            .for_each(|point| pq.push_back((point, curr_steps+1)));
    }
    if steps%2 == 0 {
        reachable_even.len()
    } else {
        reachable_odd.len()
    }
}

fn check_start_and_size_constrain(grid: &Grid<Tile>, start: Point<usize>) -> bool {
    let size = grid.get_dimension();
    if size.0 != size.1 {
        return false
    }
    if size.0%2 != 1 {
        return false
    }
    if (size.0-1)/2 != start.0 {
        return false
    }
    if (size.1-1)/2 != start.1 {
        return false
    }
    true
}

fn check_strait_line_constrain(grid: &Grid<Tile>, start: Point<usize>) -> bool {
    check_line_constrain_dir(grid, start, Direction::North) &&
        check_line_constrain_dir(grid, start, Direction::East) &&
        check_line_constrain_dir(grid, start, Direction::South) &&
        check_line_constrain_dir(grid, start, Direction::West)
}

fn check_line_constrain_dir(grid: &Grid<Tile>, start: Point<usize>, dir: Direction) -> bool {
    let mut current = start;
    while let Some(next) = dir.move_point(&current) {
        if let Some(tile) = grid.get_tile(&next) {
            if tile == &Tile::Rock {
                return false
            }
        } else {
            break
        }
        current = next;
    }
    true
}

fn check_empty_border_constrain(grid: &Grid<Tile>) -> bool {
    let size = grid.get_dimension();
    for x in 0..size.0 {
        if let Some(tile) = grid.get_tile(&(x,0)) {
            if tile == &Tile::Rock {
                return false
            }
        } else {
            return false
        }
        if let Some(tile) = grid.get_tile(&(x,size.1-1)) {
            if tile == &Tile::Rock {
                return false
            }
        } else {
            return false
        }
    }
    for y in 0..size.1 {
        if let Some(tile) = grid.get_tile(&(0,y)) {
            if tile == &Tile::Rock {
                return false
            }
        } else {
            return false
        }
        if let Some(tile) = grid.get_tile(&(size.0-1,y)) {
            if tile == &Tile::Rock {
                return false
            }
        } else {
            return false
        }
    }
    true
}

fn reachable_wrapping(grid: &Grid<Tile>, start: Point<usize>, steps: usize) -> usize {
    assert_eq!(start.0, start.1);
    let steps_to_corner = start.0+start.1;
    let steps_to_border = start.0;
    let center_values = get_values_from_start(grid, start);
    let mut sum = if steps%2 == 0 {
        center_values.even_reachable
    } else {
        center_values.odd_reachable
    };
    for dir in Direction::get_all_directions() {
        let tmp = reachable_wrapping_direction(grid, steps.saturating_sub(steps_to_border+1), dir);
        sum += tmp;
    }
    for dir in CornerDirection::get_all_directions() {
        let tmp = reachable_wrapping_corner(grid, steps.saturating_sub(steps_to_corner+2), dir);
        sum += tmp;
    }
    sum
}

fn reachable_wrapping_direction(grid: &Grid<Tile>, steps: usize, dir: Direction) -> usize {
    let steps = steps as i32;
    let size = grid.get_dimension();
    let start = match dir {
        Direction::North => (size.0/2, size.1-1),
        Direction::East => (0, size.1/2),
        Direction::South => (size.0/2, 0),
        Direction::West => (size.0-1, size.1/2),
    };
    let values = get_values_from_start(grid, start);
    let steps_to_cross = size.0 as i32;
    assert_eq!(size.0, size.1);

    let mut sum = 0;
    let mut remaining = steps;
    while remaining >= 0 {
        if remaining as usize >= values.complete_fill {
            if remaining%2 == 0 {
                sum += values.even_reachable;
            } else {
                sum += values.odd_reachable;
            }
        } else {
            let tmp = get_reachable(grid, start, remaining as usize);
            sum += tmp;
        }
        remaining -= steps_to_cross;
    }
    sum
}

fn reachable_wrapping_corner(grid: &Grid<Tile>, steps: usize, dir: CornerDirection) -> usize {
    let steps = steps as i32;
    let size = grid.get_dimension();
    let start = match dir {
        CornerDirection::NorthEast => (0, size.1-1),
        CornerDirection::SouthEast => (0, 0),
        CornerDirection::SouthWest => (size.0-1, 0),
        CornerDirection::NorthWest => (size.0-1, size.1-1),
    };
    let values = get_values_from_start(grid, start);
    let steps_to_next_diagonal = size.0 as i32;
    assert_eq!(size.0, size.1);

    let mut sum = 0;
    let mut remaining = steps;
    let mut block_count = 1;
    while remaining >= 0 {
        if remaining as usize >= values.complete_fill {
            if remaining%2 == 0 {
                sum += values.even_reachable*block_count;
            } else {
                sum += values.odd_reachable*block_count;
            }
        } else {
            let tmp = get_reachable(grid, start, remaining as usize);
            sum += tmp*block_count;
        }
        remaining -= steps_to_next_diagonal;
        block_count += 1;
    }
    sum
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum CornerDirection {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl CornerDirection {
    fn get_all_directions() -> Vec<Self> {
        vec![
            Self::NorthEast,
            Self::SouthEast,
            Self::SouthWest,
            Self::NorthWest,
        ]
    }
}

fn get_values_from_start(grid: &Grid<Tile>, start: Point<usize>) -> Values {
    let mut reachable_even = HashSet::new();
    let mut reachable_odd = HashSet::new();

    let mut pq = VecDeque::new();
    let mut max_step = 0;
    pq.push_back((start, 0usize));
    while let Some((curr_point, curr_steps)) = pq.pop_front() {
        let reachable = if curr_steps%2 == 0 {
            &mut reachable_even
        } else {
            &mut reachable_odd
        };
        if reachable.contains(&curr_point) {
            continue
        }
        reachable.insert(curr_point);
        max_step = max(max_step, curr_steps);
        get_valid_neighbors(grid, curr_point).into_iter()
            .for_each(|point| pq.push_back((point, curr_steps+1)));
    }
    Values {
        odd_reachable: reachable_odd.len(),
        even_reachable: reachable_even.len(),
        complete_fill: max_step,
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Values {
    even_reachable: usize,
    odd_reachable: usize,
    complete_fill: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Start,
    Garden,
    Rock,
}

impl Parsable for Tile {
    fn parse(c: char) -> AoCResult<Self> where Self: Sized {
        match c {
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Garden),
            '#' => Ok(Self::Rock),
            _ => Err(AoCError::BadInputFormat(
                format!("Only 'S', '#' and '.' allowed. Found '{}'.", c)))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "...........".to_string(),
            ".....###.#.".to_string(),
            ".###.##..#.".to_string(),
            "..#.#...#..".to_string(),
            "....#.#....".to_string(),
            ".##..S####.".to_string(),
            ".##..#...#.".to_string(),
            ".......##..".to_string(),
            ".##.#.####.".to_string(),
            ".##..##.##.".to_string(),
            "...........".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() -> AoCResult<()> {
        let input = get_example_input();
        let grid = Grid::parse(&input)?;
        let tmp = grid.get_all_positions_of(&Tile::Start);
        assert_eq!(tmp.len(), 1);
        let start = tmp[0];
        assert_eq!(get_reachable(&grid, start, 6), 16);
        Ok(())
    }

    #[test]
    fn check_input_part_1() -> AoCResult<()> {
        let input = get_input(2023, 21)?;
        assert_eq!(part_1(&input), Ok("3751".to_string()));
        Ok(())
    }

    #[test]
    fn check_get_values_from_start() -> AoCResult<()> {
        let input = get_example_input();
        let grid = Grid::parse(&input)?;
        let correct = Values {
            complete_fill: 14,
            even_reachable: 42,
            odd_reachable: 39,
        };
        assert_eq!(get_values_from_start(&grid, (5, 5)), correct);

        Ok(())
    }

    fn get_own_example_input() -> Vec<String> {
        vec![
            ".......".to_string(),
            "..#..#.".to_string(),
            "..#.#..".to_string(),
            "...S...".to_string(),
            ".#...#.".to_string(),
            "..#....".to_string(),
            ".......".to_string(),
        ]
    }

    #[test]
    fn check_example_part_2_own() -> AoCResult<()> {
        let input = get_own_example_input();
        let grid = Grid::parse(&input)?;
        let tmp = grid.get_all_positions_of(&Tile::Start);
        let start = tmp[0];
        assert_eq!(reachable_wrapping(&grid, start, 15), 216);
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> AoCResult<()> {
        let input = get_input(2023, 21)?;
        assert_eq!(part_2(&input), Ok("619407349431167".to_string()));
        Ok(())
    }
}
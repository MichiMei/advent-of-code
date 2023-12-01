use std::collections::{HashMap, HashSet};
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength("Expected only a single integer".to_string()))
    }
    let fav_number = input[0].parse()
        .map_err(|_|
            AoCError::UnexpectedInputLength("Expected only a single integer".to_string()))?;
    let mut maze = Maze::new(fav_number);
    let start = (1, 1);
    let end = (31, 39);
    dijkstra_target(&mut maze, start, end)
        .map(|dist| dist.to_string())
        .ok_or_else(|| AoCError::NoSolutionFoundError(
            "(31, 39) is unreachable from (1,1)".to_string()))
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    if input.len() != 1 {
        return Err(AoCError::UnexpectedInputLength("Expected only a single integer".to_string()))
    }
    let fav_number = input[0].parse()
        .map_err(|_|
            AoCError::UnexpectedInputLength("Expected only a single integer".to_string()))?;
    let mut maze = Maze::new(fav_number);
    let start = (1, 1);
    let range = 50;
    Ok(dijkstra_range(&mut maze, start, range).to_string())
}

fn dijkstra_target(maze: &mut Maze, start: Point, goal: Point) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue = vec![(start, 0)];

    while let Some((curr_point, curr_dist)) = queue.pop() {
        if curr_point == goal {
            return Some(curr_dist)
        }
        if visited.contains(&curr_point) {
            continue
        }
        visited.insert(curr_point);
        let neighbors = get_neighbors(maze, curr_point);
        for neighbor in neighbors {
            queue.push((neighbor, curr_dist+1))
        }
        queue.sort_unstable_by(|(_, d0), (_, d1)| d1.cmp(d0));
    }
    None
}

fn dijkstra_range(maze: &mut Maze, start: Point, range: usize) -> usize {
    let mut visited = HashSet::new();
    let mut queue = vec![(start, 0)];

    while let Some((curr_point, curr_dist)) = queue.pop() {
        if curr_dist > range {
            return visited.len()
        }
        if visited.contains(&curr_point) {
            continue
        }
        visited.insert(curr_point);
        let neighbors = get_neighbors(maze, curr_point);
        for neighbor in neighbors {
            queue.push((neighbor, curr_dist+1))
        }
        queue.sort_unstable_by(|(_, d0), (_, d1)| d1.cmp(d0));
    }
    visited.len()
}

fn get_neighbors(maze: &mut Maze, p: Point) -> Vec<Point> {
    let mut res = vec![];
    if p.0 > 0 && maze.point_is_floor((p.0-1, p.1)) {
        res.push((p.0-1, p.1))
    }
    if p.1 > 0 && maze.point_is_floor((p.0, p.1-1)) {
        res.push((p.0, p.1-1))
    }
    if maze.point_is_floor((p.0+1, p.1)) {
        res.push((p.0+1, p.1))
    }
    if maze.point_is_floor((p.0, p.1+1)) {
        res.push((p.0, p.1+1))
    }
    res
}

type Point = (usize, usize);

struct Maze {
    cache: HashMap<Point, bool>,
    fav_number: usize,
}

impl Maze {
    pub fn new(fav_number: usize) -> Self {
        let cache = HashMap::new();
        Self{cache, fav_number}
    }

    pub fn point_is_floor(&mut self, point: Point) -> bool {
        if let Some(is_floor) = self.cache.get(&point) {
            return *is_floor
        }
        let is_floor = self.calculate(point);
        self.cache.insert(point, is_floor);
        is_floor
    }

    fn calculate(&mut self, p: Point) -> bool {
        let number = p.0*p.0 + 3*p.0 + 2*p.0*p.1 + p.1 + p.1*p.1 + self.fav_number;
        number.count_ones()%2 == 0
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let fav_number = 10;
        let mut maze = Maze::new(fav_number);
        let start = (1, 1);
        let end = (7, 4);
        assert_eq!(dijkstra_target(&mut maze, start, end), Some(11));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 13)?;
        assert_eq!(part_1(&input), Ok("92".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 13)?;
        assert_eq!(part_2(&input), Ok("124".to_string()));
        Ok(())
    }
}
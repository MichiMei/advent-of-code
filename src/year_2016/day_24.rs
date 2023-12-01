use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let board = Board::parse(input)?;
    let shortest_path_matrix = all_pairs_shortest_path(&board)?;
    Ok(calculate_shortest_hamilton_path(&shortest_path_matrix, false).to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let board = Board::parse(input)?;
    let shortest_path_matrix = all_pairs_shortest_path(&board)?;
    Ok(calculate_shortest_hamilton_path(&shortest_path_matrix, true).to_string())
}

fn all_pairs_shortest_path(board: &Board) -> Result<Vec<Vec<usize>>, AoCError<String>> {
    let mut shortest_path_matrix = vec![];
    let row = vec![0; board.targets.len()];
    shortest_path_matrix.resize(board.targets.len(), row);

    for (targets, target_index) in board.targets.iter() {
        let distances = dijkstra(board, *targets)?;
        for (index, distance) in distances.iter().enumerate() {
            shortest_path_matrix[index][*target_index] = *distance;
            shortest_path_matrix[*target_index][index] = *distance;
        }
    }

    Ok(shortest_path_matrix)
}

fn dijkstra(board: &Board, start: Point) -> Result<Vec<usize>, AoCError<String>> {
    let mut res = vec![None; board.targets.len()];

    let mut visited = HashMap::new();
    let mut dequeue = VecDeque::new();
    dequeue.push_back((start, 0usize, None));

    while !dequeue.is_empty() {
        let (current, distance, prev) =
            dequeue.pop_front().expect("Checked in While");
        if visited.get(&current).is_some() {
            continue
        }
        visited.insert(current, (distance, prev));
        if let Some(target) = board.targets.get(&current) {
            assert!(res[*target].is_none());
            res[*target] = Some(distance);
        }
        let neighbors = get_neighbors(board, current);
        for elem in neighbors {
            dequeue.push_back((elem, distance+1, Some(current)));
        }
    }

    res.into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| AoCError::NoSolutionFoundError("Some nodes are not connected'".to_string()))
}

fn get_neighbors(board: &Board, node: Point) -> Vec<Point> {
    let mut res = vec![];
    if node.0 > 0 && board.grid[node.1][node.0-1].is_floor() {
        res.push((node.0-1, node.1));
    }
    if node.1 > 0 && board.grid[node.1-1][node.0].is_floor() {
        res.push((node.0, node.1-1));
    }
    if node.0+1 < board.grid[0].len() && board.grid[node.1][node.0+1].is_floor() {
        res.push((node.0+1, node.1));
    }
    if node.1+1 < board.grid.len() && board.grid[node.1+1][node.0].is_floor() {
        res.push((node.0, node.1+1));
    }

    res
}

fn calculate_shortest_hamilton_path(shortest_path_matrix: &Vec<Vec<usize>>, return_to_start: bool)
    -> usize {
    let remaining = (1..shortest_path_matrix.len()).collect();
    calculate_shortest_hamilton_path_rec(shortest_path_matrix, 0, &remaining,
                                         return_to_start)
}

fn calculate_shortest_hamilton_path_rec(spm: &Vec<Vec<usize>>, current: usize,
                                        remaining: &VecDeque<usize>, return_to_start: bool)
    -> usize {
    if remaining.is_empty() {
        return if return_to_start {
            spm[current][0]
        } else {
            0
        }
    }

    let mut shortest = usize::MAX;
    let mut new_remaining = remaining.clone();
    for _ in 0..remaining.len() {
        let next = new_remaining.pop_front().expect("Checked in recursion ending");
        let res = calculate_shortest_hamilton_path_rec(spm, next, &new_remaining,
                                                       return_to_start);
        shortest = min(shortest, res+spm[current][next]);
        new_remaining.push_back(next);
    }

    shortest
}

struct Board {
    grid: Vec<Vec<Tile>>,
    targets: HashMap<Point, usize>,
}

impl Board {
    pub fn parse(input: &Vec<String>) -> Result<Self, AoCError<String>> {
        let grid = vec![];
        let targets = HashMap::new();
        let mut board = Self{grid, targets};
        for line in input {
            board.add_line(line)?;
        }
        Ok(board)
    }

    fn add_line(&mut self, line: &str) -> Result<(), AoCError<String>> {
        let mut row = vec![];
        for (index, char) in line.chars().enumerate() {
            let (tile, target) = Tile::parse(char)?;
            row.push(tile);
            if let Some(target) = target {
                self.targets.insert((index, self.grid.len()), target);
            }
        }
        self.grid.push(row);
        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for line in self.grid.iter() {
            for elem in line.iter() {
                str = format!("{}{}", str, elem);
            }
            str = format!("{}\n", str);
        }
        str = format!("{}\n", str);
        for elem in self.targets.iter(){
            str = format!("{}{}\t({}, {})\n", str, elem.1, elem.0.0, elem.0.1);
        }
        write!(f, "{}", str)
    }
}

enum Tile {
    Wall,
    Floor,
}

impl Tile {
    pub fn parse(char: char) -> Result<(Self, Option<usize>), AoCError<String>> {
        match char {
            '#' => Ok((Self::Wall, None)),
            '.' => Ok((Self::Floor, None)),
            '0' => Ok((Self::Floor, Some(0))),
            '1' => Ok((Self::Floor, Some(1))),
            '2' => Ok((Self::Floor, Some(2))),
            '3' => Ok((Self::Floor, Some(3))),
            '4' => Ok((Self::Floor, Some(4))),
            '5' => Ok((Self::Floor, Some(5))),
            '6' => Ok((Self::Floor, Some(6))),
            '7' => Ok((Self::Floor, Some(7))),
            '8' => Ok((Self::Floor, Some(8))),
            '9' => Ok((Self::Floor, Some(9))),
            c => Err(AoCError::BadInputFormat(format!(
                "Unsupported character '{}'. Only '#', '.' and digits '0' to '9' allowed", c))),
        }
    }

    pub fn is_floor(&self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Floor => true,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Floor => write!(f, "."),
        }
    }
}

type Point = (usize, usize);

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "###########".to_string(),
            "#0.1.....2#".to_string(),
            "#.#######.#".to_string(),
            "#4.......3#".to_string(),
            "###########".to_string(),
        ];

        assert_eq!(part_1(&v), Ok("14".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 24)?;
        assert_eq!(part_1(&input), Ok("518".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 24)?;
        assert_eq!(part_2(&input), Ok("716".to_string()));
        Ok(())
    }
}
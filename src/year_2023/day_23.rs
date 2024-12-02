use std::cmp::max;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::errors::{AoCError, AoCResult};
use crate::geometrics::{Direction, Grid, Parsable, Point};

pub fn part_1(input: &[String]) -> AoCResult<String> {
    let grid: Grid<Tile> = Grid::parse(input)?;
    grid.longest_hike(true).map(|v| v.to_string())
}

pub fn part_2(input: &[String]) -> AoCResult<String> {
    let grid: Grid<Tile> = Grid::parse(input)?;
    let start = grid.get_start()
        .ok_or_else(|| AoCError::NoSolutionFoundError("No start found".to_string()))?;
    let end = grid.get_end()
        .ok_or_else(|| AoCError::NoSolutionFoundError("No end found".to_string()))?;

    todo!()
}

impl Grid<Tile> {
    fn longest_hike(&self, slippery: bool) -> AoCResult<usize> {
        let start = self.get_start()
            .ok_or_else(|| AoCError::NoSolutionFoundError("Could not find start".to_string()))?;
        let end = self.get_end()
            .ok_or_else(|| AoCError::NoSolutionFoundError("Could not find end".to_string()))?;
                let mut visited = HashMap::new();
        self.follow(start, end, &mut visited, 0, slippery)
            .ok_or_else(|| AoCError::NoSolutionFoundError("No path found".to_string()))
    }

    fn follow(&self, pos: Point<usize>, goal: Point<usize>,
        visited: &mut HashMap<Point<usize>, usize>, counter: usize, slippery: bool)
        -> Option<usize>
    {
        if counter == 1000 {
            println!("ended with {} tiles", visited.len());
            let mut v = visited.iter().collect::<Vec<_>>();
            v.sort_by_key(|(_, c)| **c);
            println!("{:?}", v);
        }
        assert!(counter < 1000);
        if let Some(tile) = self.get_tile(&pos) {
            if tile == &Tile::Wall {
                return None
            }
            if pos == goal {
                println!("found path with {} tiles", visited.len());
                let mut v = visited.iter().collect::<Vec<_>>();
                v.sort_by_key(|(_, c)| **c);
                println!("{:?}", v);
                return Some(0)
            }
            visited.insert(pos, counter);
            //visited.insert(pos);
            let mut max_path = None;
            for dir in tile.get_directions(slippery) {
                if let Some(new_pos) = dir.move_point(&pos) {
                    if visited.get(&new_pos).is_some() {
                    //if visited.contains(&new_pos) {
                        continue
                    }
                    let tile = self.get_tile(&new_pos);
                    if tile.is_none() || tile.unwrap() == &Tile::Wall {
                        continue
                    }
                    let res =
                        self.follow(new_pos, goal, visited, counter+1, slippery);
                    max_path = max(max_path, res);
                }
            }
            visited.remove(&pos);

            return max_path.map(|val| val+1)
        }
        None
    }

    fn get_start(&self) -> Option<Point<usize>> {
        let starts = self.row_iter(0)?
            .enumerate()
            .filter(|(_, elem)| elem != &&Tile::Wall)
            .collect::<Vec<_>>();
        if starts.len() > 1 {
            return None
        }
        starts.first().map(|(x, _)| (*x, 0))
    }

    fn get_end(&self) -> Option<Point<usize>> {
        let last = self.get_dimension().1-1;
        let starts = self.row_iter(last)?
            .enumerate()
            .filter(|(_, elem)| elem != &&Tile::Wall)
            .collect::<Vec<_>>();
        if starts.len() > 1 {
            return None
        }
        starts.first().map(|(x, _)| (*x, last))
    }
}
/*
fn get_node_index(node_map: &mut HashMap<Point<usize>, usize>, node: &Point<usize>) -> usize {
    if let Some(index) = node_map.get(node) {
        *index
    } else {
        let index = node_map.len();
        node_map.insert(*node, index);
        index
    }
}

fn grid_to_graph(grid: &Grid<Tile>, gt: GraphType) 
    -> AoCResult<(GraphWeighted, HashMap<Point<usize>, usize>)> 
{
    let mut graph = GraphWeighted::new(gt);
    let mut map = HashMap::new();
    let size = grid.get_dimension();
    for x in 0..size.0 {
        for y in 0..size.1 {
            let from_point = (x, y);
            let from_node = get_node_index(&mut map, &from_point);
            let tile = grid.get_tile(&from_point).ok_or_else(|| todo!())?;
            match tile {
                Tile::Wall => {}
                Tile::Path => {
                    let neighbors = tile.get_directions(true).into_iter()
                        .flat_map(|dir| dir.move_point(&from_point))
                        .filter(|pos| {
                            if let Some(tile) = grid.get_tile(pos) {
                                tile != &Tile::Wall
                            } else {
                                false
                            }
                        })
                        .collect::<Vec<_>>();
                    for neighbor in neighbors {
                        let to_node = get_node_index(&mut map, &neighbor);
                        graph.add_edge(from_node, to_node, 1);
                    }
                }
                Tile::Slope(dir) => {
                    let to_point = dir.move_point(&from_point).ok_or_else(|| todo!())?;
                    let to_node = get_node_index(&mut map, &to_point);
                    graph.add_edge(from_node, to_node, 1);
                }
            }
            
            
        }
    }
    
    
    
    todo!()
}
*/
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Wall,
    Path,
    Slope(Direction),
}

impl Tile {
    fn get_directions(&self, slippery: bool) -> Vec<Direction> {
        if !slippery {
            match self {
                Tile::Wall => vec![],
                _ => Direction::get_all_directions(),
            }
        } else {
            match self {
                Tile::Wall => vec![],
                Tile::Path => Direction::get_all_directions(),
                Tile::Slope(dir) => vec![*dir],
            }
        }
    }
}

impl Parsable for Tile {
    fn parse(c: char) -> AoCResult<Self> where Self: Sized {
        match c {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Path),
            '<' => Ok(Self::Slope(Direction::West)),
            '>' => Ok(Self::Slope(Direction::East)),
            '^' => Ok(Self::Slope(Direction::North)),
            'v' => Ok(Self::Slope(Direction::South)),
            _ => Err(AoCError::BadInputFormat(
                format!("Parsing Tile failed expected one of '.#<>^v'. Found '{}'", c)))
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Path => write!(f, "."),
            Tile::Slope(dir) => {
                match dir {
                    Direction::North => write!(f, "^"),
                    Direction::East => write!(f, ">"),
                    Direction::South => write!(f, "v"),
                    Direction::West => write!(f, "<"),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "#.#####################".to_string(),
            "#.......#########...###".to_string(),
            "#######.#########.#.###".to_string(),
            "###.....#.>.>.###.#.###".to_string(),
            "###v#####.#v#.###.#.###".to_string(),
            "###.>...#.#.#.....#...#".to_string(),
            "###v###.#.#.#########.#".to_string(),
            "###...#.#.#.......#...#".to_string(),
            "#####.#.#.#######.#.###".to_string(),
            "#.....#.#.#.......#...#".to_string(),
            "#.#####.#.#.#########v#".to_string(),
            "#.#...#...#...###...>.#".to_string(),
            "#.#.#v#######v###.###v#".to_string(),
            "#...#.>.#...>.>.#.###.#".to_string(),
            "#####v#.#.###v#.#.###.#".to_string(),
            "#.....#...#...#.#.#...#".to_string(),
            "#.#########.###.#.#.###".to_string(),
            "#...###...#...#...#.###".to_string(),
            "###.###.#.###v#####v###".to_string(),
            "#...#...#.#.>.>.#.>.###".to_string(),
            "#.###.###.#.###.#.#v###".to_string(),
            "#.....###...###...#...#".to_string(),
            "#####################.#".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("94".to_string()));
    }

    #[test]
    fn check_input_part_1() -> AoCResult<()> {
        let input = get_input(2023, 23)?;
        assert_eq!(part_1(&input), Ok("2326".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("154".to_string()));
    }

    #[test]
    fn check_input_part_2() -> AoCResult<()> {
        let input = get_input(2023, 23)?;
        assert_eq!(part_2(&input), Ok("expected".to_string())); // TODO
        Ok(())
    }
}
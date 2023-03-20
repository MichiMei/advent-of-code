use std::collections::HashMap;
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let cluster = parse_input(input)?;
    Ok(iterate_steps(cluster, 10000, false).to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let cluster = parse_input(input)?;
    Ok(iterate_steps(cluster, 10000000, true).to_string())
}

fn parse_input(input: &Vec<String>) -> Result<Cluster, AoCError<String>> {
    let x_offset = if input[0].len()%2 == 1 {
        input[0].len()/2
    } else {
        return Err(AoCError::BadInputFormat("Lines must have odd length.".to_string()))
    };
    let y_offset = if input.len()%2 == 1 {
        input.len()/2
    } else {
        return Err(AoCError::BadInputFormat("Input must have odd number of lines".to_string()))
    };
    let mut cluster = Cluster::new();
    for (line_index, line) in input.iter().enumerate() {
        for (col_index, elem) in line.chars().enumerate() {
            match elem {
                '.' => {}
                '#' => {
                    let point = (col_index as i32 - x_offset as i32,
                                 line_index as i32 - y_offset as i32);
                    cluster.insert(point, Status::Infected);
                }
                c => return Err(AoCError::BadInputFormat(format!(
                    "Unsupported character '{}'. Only '.' and #' supported.", c)))
            }
        }
    }
    Ok(cluster)
}

fn iterate_steps(mut cluster: Cluster, iterations: usize, complex: bool) -> usize {
    let mut position = Position::new();
    for _ in 0..iterations {
        position.step(&mut cluster, complex);
    }
    position.infection_counter
}

type Point = (i32, i32);
type Cluster = HashMap<Point, Status>;

struct Position {
    point: Point,
    direction: Direction,
    infection_counter: usize,
}

impl Position {
    fn new() -> Self {
        let point = (0, 0);
        let direction = Direction::Up;
        let infection_counter = 0;
        Self {point, direction, infection_counter}
    }

    fn step(&mut self, cluster: &mut Cluster, complex_mode: bool) {
        let status = cluster.get(&self.point).unwrap_or(&Status::Clean);
        self.direction = self.direction.turn_status_based(status);
        let new_status = if complex_mode {
            status.toggle_complex()
        } else {
            status.toggle_simple()
        };
        if new_status == Status::Infected {
            self.infection_counter += 1;
        }
        if new_status == Status::Clean {
            cluster.remove(&self.point);
        } else {
            cluster.insert(self.point, new_status);
        }
        self.point = self.direction.move_point(&self.point)
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up, Right, Down, Left,
}

impl Direction {
    fn move_point(&self, point: &Point) -> Point {
        match self {
            Direction::Up =>       (point.0,   point.1-1),
            Direction::Right =>    (point.0+1, point.1),
            Direction::Down =>     (point.0,   point.1+1),
            Direction::Left =>     (point.0-1, point.1),
        }
    }

    fn turn_status_based(&self, status: &Status) -> Self {
        match status {
            Status::Clean => self.turn_left(),
            Status::Weakened => *self,
            Status::Infected => self.turn_right(),
            Status::Flagged => self.turn_right().turn_right(),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

#[derive(Eq, PartialEq)]
enum Status {
    Clean, Weakened, Infected, Flagged,
}

impl Status {
    fn toggle_simple(&self) -> Self {
        match self {
            Status::Clean => Status::Infected,
            Status::Weakened => panic!("In simple mode status cannot be weakened"),
            Status::Infected => Status::Clean,
            Status::Flagged => panic!("In simple mode status cannot be flagged"),
        }
    }

    fn toggle_complex(&self) -> Self {
        match self {
            Status::Clean => Status::Weakened,
            Status::Weakened => Status::Infected,
            Status::Infected => Status::Flagged,
            Status::Flagged => Status::Clean
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "..#".to_string(),
            "#..".to_string(),
            "...".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let v = get_example_input();
        assert_eq!(part_1(&v), Ok("5587".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 22)?;
        assert_eq!(part_1(&input), Ok("5404".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = get_example_input();
        assert_eq!(part_2(&v), Ok("2511944".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 22)?;
        assert_eq!(part_2(&input), Ok("2511672".to_string()));
        Ok(())
    }
}
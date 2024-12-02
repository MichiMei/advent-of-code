use std::cmp::{max, min};
use std::collections::HashSet;
use crate::errors::{AoCError, AoCResult};
use crate::geometrics::{Point, Point3D};

pub fn part_1(input: &[String]) -> AoCResult<String> {
    let mut list = Block::parse_list(input)?;
    let mut stack_area = StackArea::from_blocks(&list);
    stack_area.set_status(&mut list);
    settle(&mut list);
    Ok(count_destroyable(&list).to_string())
}

pub fn part_2(input: &[String]) -> AoCResult<String> {
    let mut list = Block::parse_list(input)?;
    let mut stack_area = StackArea::from_blocks(&list);
    stack_area.set_status(&mut list);
    settle(&mut list);
    let supporting = get_supporting(&list);
    let falling = (0..list.len())
        .map(|id| count_falling(&list, &supporting, id))
        .collect::<Vec<_>>();

    Ok(falling.iter()
        .sum::<usize>()
        .to_string())
}

fn settle(list: &mut [Block]) {
    let mut remaining = HashSet::from_iter(0..list.len());
    while !remaining.is_empty() {
        let mut tmp = HashSet::new();
        for id in remaining {
            if let Some(steps) = list[id].settle(list) {
                list[id].move_down(steps);
            } else {
                tmp.insert(id);
            }
        }
        remaining = tmp;
    }
}

fn count_falling(list: &[Block], supporting: &[HashSet<usize>], id: usize) -> usize {
    let mut falling = vec![false; list.len()];
    falling[id] = true;
    let mut remaining = supporting[id].iter().copied().collect::<Vec<_>>();
    while let Some(next) = remaining.pop() {
        let still_supporting = list[next].get_real_blocking(list)
            .iter()
            .copied()
            .filter(|supp| !falling[*supp])
            .collect::<Vec<_>>();
        if still_supporting.is_empty() {
            falling[next] = true;
            supporting[next].iter()
                .copied()
                .for_each(|other| remaining.push(other))
        }

    }
    falling.iter()
        .filter(|b| **b)
        .count()-1
}

fn get_supporting(list: &[Block]) -> Vec<HashSet<usize>> {
    let mut res = vec![HashSet::new(); list.len()];
    for block in list {
        let supporting = block.get_real_blocking(list);
        for other in supporting {
            res[other].insert(block.id);
        }
    }

    res
}

fn count_destroyable(list: &Vec<Block>) -> usize {
    let mut destroyable = vec![true; list.len()];

    for (_id, block) in list.iter().enumerate() {
        let blocking = block.get_real_blocking(list);
        if blocking.len() == 1 {
            destroyable[blocking[0]] = false;
        }
    }

    destroyable.iter()
        .filter(|elem| **elem)
        .count()
}

struct StackArea {
    grid: Vec<Vec<Vec<(usize, usize)>>>,
    size: Point<usize>,
}

impl StackArea {
    fn from_blocks(list: &[Block]) -> Self {
        let mut res = Self {
            grid: vec![vec![vec![]]],
            size: (1, 1),
        };
        for block in list.iter() {
            res.add_block(block);
        }
        res
    }

    fn add_block(&mut self, block: &Block) {
        for cube in block.cubes.iter() {
            self.add(*cube, block.id);
        }
    }

    fn add(&mut self, point: Point3D<usize>, id: usize) {
        if point.x >= self.size.0 || point.y >= self.size.1 {
            self.resize(point);
        }
        self.grid[point.x][point.y].push((point.z, id));
    }

    fn resize(&mut self, point3d: Point3D<usize>) {
        if point3d.x >= self.size.0 {
            self.grid.resize(point3d.x+1, vec![vec![]; self.size.1]);
            self.size.0 = point3d.x+1;
            assert_eq!(self.size.0, self.grid.len());
        }
        if point3d.y >= self.size.1 {
            for y_row in self.grid.iter_mut() {
                y_row.resize(point3d.y+1, vec![]);
            }
            self.size.1 = point3d.y+1;
            assert_eq!(self.size.1, self.grid[0].len());
        }
    }

    fn sort(&mut self) {
        for row in self.grid.iter_mut() {
            for stack in row.iter_mut() {
                stack.sort_by(|(z0, _), (z1, _)| z0.cmp(z1))
            }
        }
    }

    fn set_status(&mut self, list: &mut [Block]) {
        self.sort();
        for row in self.grid.iter() {
            for stack in row.iter() {
                for window in stack.windows(2) {
                    let block1 = window[1];
                    let block0 = window[0];
                    if block0.1 == block1.1 {
                        continue
                    }
                    assert!(block0.0 < block1.0);
                    list[block1.1].blocking.insert(block0.1);
                }

            }
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Block {
    id: usize,
    cubes: Vec<Point3D<usize>>,
    blocking: HashSet<usize>,
    settled: bool,
}

impl Block {
    fn parse_list(input: &[String]) -> AoCResult<Vec<Self>> {
        input.iter()
            .enumerate()
            .map(|(index, line)| Self::parse(line, index))
            .collect()
    }

    fn parse(line: &str, id: usize) -> AoCResult<Self> {
        let split = line.split('~').collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(AoCError::BadInputFormat(
                format!("Parsing Block failed. '~' not found in '{}'.", line)))
        }
        let corner0 = Self::parse_point(split[0])?;
        let corner1 = Self::parse_point(split[1])?;
        let mut cubes = vec![];
        for x in min(corner0.x, corner1.x)..=max(corner0.x, corner1.x) {
            for y in min(corner0.y, corner1.y)..=max(corner0.y, corner1.y) {
                for z in min(corner0.z, corner1.z)..=max(corner0.z, corner1.z) {
                    cubes.push(Point3D{x, y, z})
                }
            }
        }
        Ok(Self {
            id,
            cubes,
            blocking: HashSet::new(),
            settled: false,
        })
    }

    fn parse_point(str: &str) -> AoCResult<Point3D<usize>> {
        let split = str.split(',').collect::<Vec<_>>();
        if split.len() != 3 {
            return Err(AoCError::BadInputFormat(
                format!("Parsing point failed, expected 'x,y,z'. Found '{}'.", str)))
        }
        let numbers = split.iter()
            .map(|str| str.parse()
                .map_err(|e| AoCError::BadInputFormat(
                    format!("Parsing number '{}' failed. {}", str, e))))
            .collect::<AoCResult<Vec<_>>>()?;
        Ok(Point3D {
            x: numbers[0],
            y: numbers[1],
            z: numbers[2],
        })
    }

    fn max(&self) -> usize {
        self.cubes.iter()
            .map(|cube| cube.z)
            .max().expect("Cubes cannot be empty")
    }

    fn min(&self) -> usize {
        self.cubes.iter()
            .map(|cube| cube.z)
            .min().expect("Cubes cannot be empty")
    }

    fn move_down(&mut self, steps: usize) {
        for cube in self.cubes.iter_mut() {
            cube.z -= steps;
        }
        self.settled = true;
    }

    fn settle(&self, list: &[Block]) -> Option<usize> {
        let mut maximum = 0;
        for id in self.blocking.iter() {
            if !list[*id].settled {
                return None
            }
            maximum = max(maximum, list[*id].max());
        }
        assert!(self.min() > maximum);
        let diff = self.min() - (maximum + 1);
        Some(diff)
    }

    fn get_real_blocking(&self, list: &[Block]) -> Vec<usize> {
        self.blocking.iter()
            .copied()
            .filter(|id| list[*id].max()+1 == self.min())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "1,0,1~1,2,1".to_string(),
            "0,0,2~2,0,2".to_string(),
            "0,2,3~2,2,3".to_string(),
            "0,0,4~0,2,4".to_string(),
            "2,0,5~2,2,5".to_string(),
            "0,1,6~2,1,6".to_string(),
            "1,1,8~1,1,9".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("5".to_string()));
    }

    #[test]
    fn check_input_part_1() -> AoCResult<()> {
        let input = get_input(2023, 22)?;
        assert_eq!(part_1(&input), Ok("411".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let input = get_example_input();
        assert_eq!(part_2(&input), Ok("7".to_string()));
    }

    #[test]
    fn check_input_part_2() -> AoCResult<()> {
        let input = get_input(2023, 22)?;
        assert_eq!(part_2(&input), Ok("47671".to_string()));
        Ok(())
    }
}
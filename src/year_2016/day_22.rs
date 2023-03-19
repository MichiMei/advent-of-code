use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let nodes = parse_df_output(input)?;
    let count = count_viable_pairs(&nodes);
    Ok(count.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let nodes = parse_df_output(input)?;

    let (node_array, src, empty) = get_node_array(&nodes)?;
    let min_size = check_node_array(&node_array, src, empty)?;

    let empty_dest = (src.0-1, src.1);
    let move_empty_steps =
        find_shortest_path(&node_array, empty, empty_dest, min_size)?;
    let data_transfer_steps =
        calculate_data_transfer_steps(src, (0,0), empty_dest);
    let steps = move_empty_steps+data_transfer_steps;
    println!("{} + {} = {}", move_empty_steps, data_transfer_steps, steps);

    Ok(steps.to_string())
}

/// Parses the output of df and returns a list of Node objects containing all relevant data.
/// Throws an error if one of the lines is malformed.
fn parse_df_output(input: &[String]) -> Result<Vec<Node>, AoCError<String>> {
    input.iter()
        .skip(2)
        .map(|line| Node::parse(line))
        .collect()
}

/// Returns how many different data transfers are possible.
/// A data transfer from node u to node v is possible, iff u.used <= v.avail.
fn count_viable_pairs(nodes: &[Node]) -> usize {
    let mut sorted_used = nodes.iter()
        .map(|node| node.used).collect::<Vec<_>>();
    sorted_used.sort_unstable();
    let mut sorted_avail = nodes.iter()
        .map(|node| node.available).collect::<Vec<_>>();
    sorted_avail.sort_unstable();

    let mut iter_used = sorted_used.iter()
        .skip_while(|used| **used == 0);
    let mut iter_avail = sorted_avail.iter();
    let mut next_used = iter_used.next();
    let mut next_avail = iter_avail.next();

    let mut count = 0;

    while next_used.is_some() && next_avail.is_some() {
        if next_used.unwrap() > next_avail.unwrap() {
            next_avail = iter_avail.next();
        } else {
            count += 1 + iter_avail.len();
            next_used = iter_used.next();
        }
    }

    count
}

/// Transforms a list of nodes to a HashMap of Point (x, y) to nodes for fast node access by
/// coordinates. Additionally the data source and empty node are returned.
/// Throws an error, if
/// 1. More than a single empty node is found (optimal solving not supported)
/// 2. No empty node is found (optimal solving not supported)
/// 3. No data source node found (with y=0 and largest possible x)
fn get_node_array(nodes: &[Node])
    -> Result<(NodeArray, Point, Point), AoCError<String>> {
    let mut map = HashMap::new();
    let mut src: Option<Point> = None;
    let mut empty = None;
    for node in nodes.iter() {
        map.insert(node.point, node);
        if node.point.1 == 0 && (src.is_none() || src.unwrap().0 < node.point.0) {
            src = Some(node.point);
        }
        if node.used == 0 {
            if empty.is_none() {
                empty = Some(node.point);
            } else {
                return Err(AoCError::NoSolutionFoundError(
                    "Node array contains multiple empty nodes. Solving unsupported".to_string()))
            }
        }
    }
    let src = src
        .ok_or_else(|| AoCError::BadInputFormat("No node with y=0 found.".to_string()))?;
    let empty = empty
        .ok_or_else(|| AoCError::NoSolutionFoundError(
            "No empty node found. Solving unsupported".to_string()))?;
    Ok((map, src, empty))
}

/// Checks if any of the requirements for optimal solving are violated and returns the minimum size
/// of all nodes.
/// Throws an error, if
/// 1. Any of the regular sized notes used exceeds the size of any other node.
/// 2. No direct two wide path between data source and destination exists
fn check_node_array(nodes: &NodeArray, src: Point, empty: Point) -> Result<u16, AoCError<String>> {
    let min_size = check_sizes(nodes, empty)?;
    check_free_path(nodes, src, min_size)?;
    Ok(min_size)
}

/// Checks if the used memory of any regular sized note (used < empty_node.size) exceeds the size of
/// any other node.
/// Returns the minimum size of all nodes.
/// Throws an error if the condition is violated.
fn check_sizes(nodes: &NodeArray, empty: Point) -> Result<u16, AoCError<String>> {
    let empty_node = nodes.get(&empty).expect("Should not have been removed");
    let mut max_used = empty_node.used;
    let mut min_size = empty_node.size;

    for node in nodes.values() {
        if node.used <= empty_node.size {
            max_used = max(max_used, node.used);
        }
        min_size = min(min_size, node.size);
    }
    if max_used > min_size {
        Err(AoCError::NoSolutionFoundError("Some regular nodes are too full.".to_string()))
    } else {
        Ok(min_size)
    }
}

/// Checks if a two wide direct path (no corners or diagonal parts) exist between the data source
/// and destination.
/// Throws an error if the condition is violated.
fn check_free_path(nodes: &NodeArray, src: Point, min_size: u16) -> Result<(), AoCError<String>> {
    for x in 0..=src.0 {
        let direct = nodes.get(&(x, 0))
            .ok_or_else(|| AoCError::BadInputFormat(format!("Node ({}, 0) missing!", x)))?;
        let indirect = nodes.get(&(x, 1))
            .ok_or_else(|| AoCError::BadInputFormat(format!("Node ({}, 1) missing!", x)))?;
        if direct.used > min_size || indirect.used > min_size {
            return Err(AoCError::NoSolutionFoundError(
                "Path (2 wide) between source and destination node blocked. Solving unsupported."
                    .to_string()))
        }
    }
    Ok(())
}

/// Calculates the shortest path between the given notes (empty node and node next to data source
/// closest to data destination) using dijkstra's algorithm.
/// Returns the number of swaps needed to place the empty node between data source and destination
/// adjacent to data source.
/// Throws an error if no path exists (considering nodes with too much used memory).
fn find_shortest_path(nodes: &NodeArray, src: Point, dest: Point, min_size: u16)
    -> Result<usize, AoCError<String>> {
    let mut visited = HashSet::new();
    let mut dequeue = VecDeque::new();
    dequeue.push_back((src, 0));
    while !dequeue.is_empty() {
        let (curr_point, curr_dist) = dequeue.pop_front()
            .expect("Tested by while clause");
        if curr_point == dest {
            return Ok(curr_dist)
        }
        if visited.contains(&curr_point) {
            continue
        }
        let neighbors = get_neighbors(nodes, curr_point, min_size)
            .into_iter()
            .map(|n| (n, curr_dist+1));
        for neighbor in neighbors {
            dequeue.push_back(neighbor);
        }
        visited.insert(curr_point);
    }
    Err(AoCError::NoSolutionFoundError(format!(
        "No path between empty ({}, {}) and data source ({}, {}) found.",
        src.0, src.1, dest.0, dest.1)))
}

/// Returns a list of the adjacent nodes to the given node u with used memory smaller than u's size.
fn get_neighbors(nodes: &NodeArray, point: Point, min_size: u16) -> Vec<Point> {
    let mut neighbors = vec![];
    if point.0 > 0 {
        if let Some(neighbor) = nodes.get(&(point.0-1, point.1)) {
            if neighbor.used <= min_size {
                neighbors.push(neighbor.point);
            }
        }
    }
    if point.1 > 0 {
        if let Some(neighbor) = nodes.get(&(point.0, point.1-1)) {
            if neighbor.used <= min_size {
                neighbors.push(neighbor.point);
            }
        }
    }
    if let Some(neighbor) = nodes.get(&(point.0+1, point.1)) {
        if neighbor.used <= min_size {
            neighbors.push(neighbor.point);
        }
    }
    if let Some(neighbor) = nodes.get(&(point.0, point.1+1)) {
        if neighbor.used <= min_size {
            neighbors.push(neighbor.point);
        }
    }
    neighbors
}

/// Calculates the swaps necessary to move the data from data source to destination, providing a two
/// wide direct path (with no corners or diagonal parts) between them exists and the empty node is
/// adjacent to the source and between the two nodes.
fn calculate_data_transfer_steps(src: Point, dest: Point, empty: Point) -> usize {
    assert_eq!(src.0, empty.0+1);
    assert_eq!(src.1, empty.1);
    assert_eq!(src.1, dest.1);
    assert!(dest.0 < src.0);

    let dist = src.0 - dest.0;

    // moving x steps requires x (empty, src) swaps and x-1 empty move-around
    1 + (dist-1) * (1+4)
}

type Point = (usize, usize);

type NodeArray<'a> = HashMap<Point, &'a Node>;

#[derive(Clone, Copy)]
struct Node {
    point: Point,
    size: u16,
    used: u16,
    available: u16,
}

impl Node {
    /// Parses a df output line and transforms it into a node object containing all the relevant
    /// data.
    /// Throws an error if the line is malformed.
    pub fn parse(line: &str) -> Result<Self, AoCError<String>> {
        let words = line.split_whitespace().collect::<Vec<_>>();
        if words.len() != 5 {
            return Err(AoCError::BadInputFormat("df output line malformed".to_string()))
        }
        let point = Self::parse_point(words[0])?;
        let size = Self::parse_value(words[1])?;
        let used = Self::parse_value(words[2])?;
        let available = Self::parse_value(words[3])?;

        Ok(Self{point, size, used, available})
    }

    /// Parses the coordinates encoded in a nodes name.
    /// Throws an error if the nodes name is malformed.
    fn parse_point(str: &str) -> Result<Point, AoCError<String>> {
        let words = str.split('-').collect::<Vec<_>>();
        if words.len() != 3 {
            return Err(AoCError::BadInputFormat("Node name is malformed".to_string()))
        }
        let x = words[1][1..].parse().map_err(|e| AoCError::BadInputFormat(format!(
            "Parsing node name x coordinate failed, expected number, found '{}'. {}",
            words[1], e)))?;
        let y = words[2][1..].parse().map_err(|e| AoCError::BadInputFormat(format!(
            "Parsing node name y coordinate failed, expected number, found '{}'. {}",
            words[2], e)))?;
        Ok((x, y))
    }

    /// Parses a value from a df output line (e.g. 46T or 156T).
    /// Throws an error if the given value could not be parsed.
    fn parse_value(str: &str) -> Result<u16, AoCError<String>> {
        let val = str[0..str.len()-1].parse().map_err(|e|
            AoCError::BadInputFormat(format!("Parsing value failed, expected number, found \
            '{}'. {}", str, e)))?;
        Ok(val)
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_22.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("967".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_22.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("205".to_string()));
        Ok(())
    }
}
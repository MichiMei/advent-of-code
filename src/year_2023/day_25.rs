use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use crate::errors::{AoCError, AoCResult};

pub fn part_1(input: &[String]) -> AoCResult<String> {
    let mut graph = Graph::parse(input)?;
    let most_used = graph.get_most_used_edges(3);
    for edge in most_used {
        graph.remove_edge(edge.0, edge.1);
    }
    let components = graph.component_sizes();
    if components.len() != 2 {
        return Err(AoCError::NoSolutionFoundError(
            format!("Expected two components, found {}.", components.len())))
    }
    Ok(components.iter().product::<usize>().to_string())
}

pub fn part_2(_input: &[String]) -> AoCResult<String> {
    Ok("Merry Christmas!".to_string())
}

struct Graph {
    edges: Vec<HashSet<usize>>,
    name_mapping: HashMap<String, usize>,
}

fn get_node_index(name: &str, name_map: &mut HashMap<String, usize>) -> usize {
    if let Some(index) = name_map.get(name) {
        return *index
    }
    let index = name_map.len();
    name_map.insert(name.to_string(), index);
    index
}

//impl Graph {
impl Graph {
    fn new() -> Self {
        Self {
            edges: vec![],
            name_mapping: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].insert(to);
        self.edges[to].insert(from);
    }

    fn node_count(&self) -> usize {
        self.edges.len()
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        self.edges[from].remove(&to);
        self.edges[to].remove(&from);
    }

    fn parse(input: &[String]) -> AoCResult<Self> {
        let mut name_map = HashMap::new();
        let mut graph = Graph::new();
        for line in input {
            graph.parse_line(line, &mut name_map)?;
        }
        Ok(graph)
    }

    fn parse_line(&mut self, line: &str, name_map: &mut HashMap<String, usize>) -> AoCResult<()> {
        let split = line.split(": ").collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(AoCError::BadInputFormat("Parsing line failed, ':' not found".to_string()))
        }
        let start = split[0];
        for end in split[1].split_whitespace() {
            let from = get_node_index(start, name_map);
            let to = get_node_index(end, name_map);
            self.add_edge(from, to);
        }
        Ok(())
    }

    fn component_sizes(&self) -> Vec<usize> {
        let mut visited = vec![false; self.node_count()];
        let mut res = vec![];

        while let Some(start) = visited.iter().enumerate()
            .find(|(_, visited)| !**visited) {
            let mut count = 0;
            let mut pq = vec![start.0];
            while let Some(current) = pq.pop() {
                if visited[current] {
                    continue;
                }
                visited[current] = true;
                count += 1;
                //for other in self.neighbor_iter(current)
                for other in self.edges[current].iter() {
                    if !visited[*other] {
                        pq.push(*other);
                    }
                }
            }
            res.push(count);
        }
        res
    }

    fn get_most_used_edges(&self, amount: usize) -> Vec<(usize, usize)> {
        let edge_uses = self.get_all_edge_uses();
        let mut sorted = edge_uses.into_iter().collect::<Vec<_>>();
        sorted.sort_by_key(|(_, uses)| *uses);
        let start = sorted.len()-amount;
        sorted[start..].iter().map(|(edge, _)| *edge).collect()
    }

    fn get_all_edge_uses(&self) -> HashMap<(usize, usize), usize> {
        let mut res = HashMap::new();
        for start in 0..self.node_count() {
           let tmp = self.shortest_paths(start);
            for (edge, uses) in tmp {
                if let Some(prev_uses) = res.get(&edge) {
                    res.insert(edge, uses+prev_uses);
                } else {
                    res.insert(edge, uses);
                }
            }

        }
        res
    }

    /// Returns a mapping of edges to the number of uses
    fn shortest_paths(&self, start: usize) -> HashMap<(usize, usize), usize> {
        let node_data = self.dijkstra(start);
        let mut res = HashMap::new();
        for (mut node, data) in node_data.iter().enumerate() {
            if let Some((mut prev, _)) = data {
                while node != prev {
                    let edge = (min(node, prev), max(node, prev));
                    if let Some(uses) = res.get(&edge) {
                        res.insert(edge, uses+1);
                    } else {
                        res.insert(edge, 1usize);
                    }
                    prev = node;
                    node = node_data[node].expect("no None can be in a path").0;
                }
            }
        }
        res
    }

    fn dijkstra(&self, start: usize) -> Vec<Option<(usize, usize)>> {
        let mut node_data = vec![None; self.node_count()];
        let mut pq = VecDeque::new();
        pq.push_back((start, start, 0usize));
        while let Some((node, prev, length)) = pq.pop_front() {
            if let Some((_, length1)) = &node_data[node] {
                assert!(length >= *length1);
                continue
            }
            node_data[node] = Some((prev, length));
            assert!(node < self.node_count());
            //for target in self.neighbor_iter(node).expect("Verified by assert") {
            for target in self.edges[node].iter() {
                if node_data[*target].is_none() {
                    pq.push_back((*target, node, length+1));
                }
            }
        }
        node_data
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "jqt: rhn xhk nvd".to_string(),
            "rsh: frs pzl lsr".to_string(),
            "xhk: hfx".to_string(),
            "cmg: qnr nvd lhk bvb".to_string(),
            "rhn: xhk bvb hfx".to_string(),
            "bvb: xhk hfx".to_string(),
            "pzl: lsr hfx nvd".to_string(),
            "qnr: nvd".to_string(),
            "ntq: jqt hfx bvb xhk".to_string(),
            "nvd: lhk".to_string(),
            "lsr: lhk".to_string(),
            "rzs: qnr cmg lsr rsh".to_string(),
            "frs: qnr lhk lsr".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let input = get_example_input();
        assert_eq!(part_1(&input), Ok("54".to_string()));
    }

    #[test]
    fn check_input_part_1() -> AoCResult<()> {
        let input = get_input(2023, 25)?;
        assert_eq!(part_1(&input), Ok("532891".to_string()));
        Ok(())
    }
}
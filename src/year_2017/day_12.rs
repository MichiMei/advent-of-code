use std::collections::HashSet;
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let graph = Graph::parse(input)?;
    Ok(graph.get_component_size(0).to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let graph = Graph::parse(input)?;
    Ok(graph.get_component_count().to_string())
}

struct Graph {
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn parse(input: &Vec<String>) -> Result<Self, AoCError<String>> {
        let mut edges = vec![];
        for line in input {
            let (index, neighbors) = Self::parse_line(line)?;
            if index >= edges.len() {
                edges.resize(index+1, vec![]);
            }
            edges[index].extend(neighbors);
        }
        Ok(Self{edges})
    }

    fn parse_line(line: &str) -> Result<(usize, Vec<usize>), AoCError<String>> {
        let words = line.split(" <-> ").collect::<Vec<_>>();
        let index = words[0].parse()
            .map_err(|e| AoCError::BadInputFormat(
                format!("Parsing index failed. Expected number, found '{}'. {}", words[0], e)))?;
        let mut edges = vec![];
        for word in words[1].split(", ") {
            edges.push(word.parse()
                .map_err(|e| AoCError::BadInputFormat(
                    format!("Parsing edge failed. Expected number, found '{}'. {}", word, e)))?);
        }
        Ok((index, edges))
    }

    fn get_component_size(&self, node: usize) -> usize {
        let mut set = HashSet::new();
        set.insert(node);
        let mut queue = vec![node];
        while !queue.is_empty() {
            let next = queue.pop().expect("Was checked by while");
            if let Some(neighbors) = self.edges.get(next) {
                for neighbor in neighbors {
                    if !set.contains(neighbor) {
                        set.insert(*neighbor);
                        queue.push(*neighbor);
                    }
                }
            }
        }
        set.len()
    }

    fn get_component_count(&self) -> usize {
        let mut remaining = (0..self.edges.len()).collect::<HashSet<_>>();
        let mut components = 0;
        while !remaining.is_empty() {
            let next = *remaining.iter().next().expect("Was checked by while");
            remaining.remove(&next);
            let mut queue = vec![next];
            while !queue.is_empty() {
                let next = queue.pop().expect("Was checked by while");
                for neighbors in self.edges[next].iter() {
                    if remaining.remove(neighbors) {
                        queue.push(*neighbors);
                    }
                }
            }
            components += 1;
        }
        components
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "0 <-> 2".to_string(),
            "1 <-> 1".to_string(),
            "2 <-> 0, 3, 4".to_string(),
            "3 <-> 2, 4".to_string(),
            "4 <-> 2, 3, 6".to_string(),
            "5 <-> 6".to_string(),
            "6 <-> 4, 5".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let v = get_example_input();
        assert_eq!(part_1(&v), Ok("6".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_12.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("306".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = get_example_input();
        assert_eq!(part_2(&v), Ok("2".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2017/input_day_12.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("200".to_string()));
        Ok(())
    }
}
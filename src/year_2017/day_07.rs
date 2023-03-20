use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let tree = create_tree(input)?;
    Ok(tree.name.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let root = create_tree(input)?;
    root.check_weight()?
        .map(|corrected_weight| corrected_weight.to_string())
        .ok_or_else(|| AoCError::NoSolutionFoundError(
            "No node with wrong weight was found.".to_string()))
}

fn create_tree(input: &[String]) -> Result<TreeElement, AoCError<String>> {
    let mut remaining = input.iter().collect::<Vec<_>>();
    let mut nodes = HashMap::new();
    while !remaining.is_empty() {
        let mut impossible = vec![];
        for line in remaining {
            let res = TreeElement::parse(line, nodes)?;
            nodes = match res {
                Ok(nodes_ret) => nodes_ret,
                Err(nodes_ret) => {
                    impossible.push(line);
                    nodes_ret
                }
            }
        }
        remaining = impossible;
    }
    if nodes.len() != 1 {
        Err(AoCError::NoSolutionFoundError(format!("Building the tree failed. Found {} roots.", nodes.len())))
    } else {
        Ok(nodes.into_iter().next().expect("Length was tested to be 1").1)
    }

}

type Nodes<'a> = HashMap<&'a str, TreeElement<'a>>;

struct TreeElement<'a> {
    name: &'a str,
    weight: usize,
    children: Vec<TreeElement<'a>>,
}

impl<'a> TreeElement<'a> {
    fn parse(line: &'a str, mut nodes: HashMap<&'a str, TreeElement<'a>>)
             -> Result<Result<Nodes<'a>, Nodes<'a>>, AoCError<String>> {
        let words = line.split_whitespace()
            .map(|word| {
                if word.ends_with(',') {
                    &word[0..word.len()-1]
                } else {
                    word
                }
            })
            .collect::<Vec<_>>();
        if words.len() < 2 {
            return Err(AoCError::BadInputFormat(format!(
                "Parsing node failed, expected '<name> (<weight>) [-> <child0>{{, <childX>}}]'. \
                Found: '{}'", line)))
        }
        let name = words[0];
        let weight = words[1][1..words[1].len()-1].parse()
            .map_err(|e| AoCError::BadInputFormat(format!(
                "Parsing weight failed. Expected number, found '{}'. {}", words[1], e)))?;
        if words.len() == 2 {
            nodes.insert(name, Self{name, weight, children: vec![]});
            return Ok(Ok(nodes))
        }
        if words.len() == 3 {
            return Err(AoCError::BadInputFormat(format!(
                "Parsing node failed, expected '<name> (<weight>) [-> <child0>{{, <childX>}}]'. \
                Found: '{}'", line)))
        }
        let mut all_contained = true;
        for child_name in words[3..].iter() {
            if !nodes.contains_key(child_name) {
                all_contained = false;
            }
        }
        if !all_contained {
            return Ok(Err(nodes))
        }
        let children = words[3..].iter()
            .map(|child_name| nodes.remove(child_name)
                .expect("Containment was tested"))
            .collect::<Vec<_>>();
        nodes.insert(name, Self{name, weight, children});
        Ok(Ok(nodes))
    }

    fn check_weight(&self) -> Result<Option<usize>, AoCError<String>> {
        if self.children.is_empty() {
            return Ok(None)
        }
        for child in self.children.iter() {
            if let Some(corrected_weight) = child.check_weight()? {
                return Ok(Some(corrected_weight))
            }
        }
        if let Some(corrected_weight) = self.check_children_weights()? {
            return Ok(Some(corrected_weight))
        }
        Ok(None)
    }

    fn check_children_weights(&self) -> Result<Option<usize>, AoCError<String>> {
        let mut weights = HashMap::new();
        for child in self.children.iter() {
            let count = weights.get(
                &child.get_recursive_weight()).unwrap_or(&0usize);
            weights.insert(child.get_recursive_weight(), count+1);
        }
        if weights.len() == 1 {
            return Ok(None)
        }
        if weights.len() > 2 {
            return Err(AoCError::MultipleSolutionsFoundError(
                "Multiple nodes have wrong weight".to_string()))
        }
        let right_rec = weights.iter()
            .max_by(|(_, c0), (_, c1)| c0.cmp(c1))
            .expect("Map was tested to be not empty").0;
        let wrong_rec = weights.iter()
            .min_by(|(_, c0), (_, c1)| c0.cmp(c1))
            .expect("Map was tested to be not empty").0;
        let wrong_weight = self.children.iter()
            .map(|child| (child.get_recursive_weight(), child.weight))
            .find(|(weight_rec, _)| weight_rec.eq(wrong_rec))
            .expect("One can and must match this wrong weight").1;
        Ok(Some(wrong_weight+right_rec-wrong_rec))
    }

    fn get_recursive_weight(&self) -> usize {
        self.weight + self.children.iter()
            .map(|child| child.get_recursive_weight())
            .sum::<usize>()
    }
}

impl<'a> Display for TreeElement<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = format!("{} ({}) -> ", self.name, self.weight);
        for child in self.children.iter() {
            str = format!("{} {}, ", str, child.name);
        }
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "pbga (66)".to_string(),
            "xhth (57)".to_string(),
            "ebii (61)".to_string(),
            "havc (66)".to_string(),
            "ktlj (57)".to_string(),
            "fwft (72) -> ktlj, cntj, xhth".to_string(),
            "qoyq (66)".to_string(),
            "padx (45) -> pbga, havc, qoyq".to_string(),
            "tknk (41) -> ugml, padx, fwft".to_string(),
            "jptl (61)".to_string(),
            "ugml (68) -> gyxo, ebii, jptl".to_string(),
            "gyxo (61)".to_string(),
            "cntj (57)".to_string(),
        ]
    }

    #[test]
    fn check_examples_part_1() {
        let v = get_example_input();
        assert_eq!(part_1(&v), Ok("tknk".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 7)?;
        assert_eq!(part_1(&input), Ok("eugwuhl".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = get_example_input();
        assert_eq!(part_2(&v), Ok("60".to_string()));
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2017, 7)?;
        assert_eq!(part_2(&input), Ok("420".to_string()));
        Ok(())
    }
}
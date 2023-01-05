pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;

pub mod test {
    use std::collections::{HashMap, HashSet};
    use std::fmt::{Display, Formatter};

    pub fn run(input: &Vec<String>) {
        rewrite_rules(&input[0..input.len()-2], &input[input.len()-1]);
    }

    fn rewrite_rules(vec: &[String], goal_str: &String) {
        let mut map = HashMap::new();
        let mut rules = vec![];
        let mut sources = HashSet::new();
        for line in vec {
            let rule_str = parse_rule(line);
            let rule = Rule::from(rule_str, &mut map);
            sources.insert(rule.src);
            rules.push(rule);
        }
        let goal = parse_goal(goal_str, &map);

        let mut elem_count = vec![];

        for rule in rules {
            println!("{}", rule.print(&sources));
        }
        println!();
        for elem in goal {
            print!("{}", usize_to_char(elem, &sources));
        }
        println!()
    }

    fn parse_rule(str: &str) -> (String, Vec<String>) {
        let words: Vec<&str> = str.split(" => ").collect();
        assert_eq!(words.len(), 2);
        let src = words[0].to_string();
        let mut dest = vec![];
        let mut iter = words[1].chars().peekable();
        while let Some(c) = iter.next() {
            let mut tmp = vec![c];
            if let Some(c) = iter.peek() {
                if c.is_lowercase() {
                    tmp.push(iter.next().unwrap());
                }
                assert!(iter.peek().is_none() || iter.peek().unwrap().is_uppercase());
            }
            dest.push(tmp.iter().collect());
        }
        (src, dest)
    }

    fn parse_goal(str: &str, map: &HashMap<String, usize>) -> Vec<usize> {
        let mut res = vec![];
        let mut iter = str.chars().peekable();

        while let Some(c) = iter.next() {
            let mut tmp = vec![c];
            if let Some(c) = iter.peek() {
                if c.is_lowercase() {
                    tmp.push(iter.next().unwrap());
                }
                assert!(iter.peek().is_none() || iter.peek().unwrap().is_uppercase());
            }
            let name: String = tmp.iter().collect();
            res.push(*map.get(&name).unwrap())
        }

        res
    }

    struct Rule {
        src: usize,
        dest: Vec<usize>,
    }

    impl Rule {
        fn from((src_str, dest_str): (String, Vec<String>), map: &mut HashMap<String, usize>) -> Self {
            let src = match map.get(&src_str) {
                None => {
                    let index = map.len();
                    map.insert(src_str, index);
                    index
                }
                Some(index) => *index,
            };
            let mut dest = vec![];
            for elem in dest_str {
                dest.push(match map.get(&elem) {
                    None => {
                        let index = map.len();
                        map.insert(elem, index);
                        index
                    }
                    Some(index) => *index,
                });
            }
            Self{src, dest}
        }

        fn print(&self, sources: &HashSet<usize>) -> String {
            let mut res = format!("{} => ", usize_to_char(self.src, sources));
            for elem in self.dest.iter() {
               res = format!("{}{}", res, usize_to_char(*elem, sources));
            }
            res
        }
    }

    fn usize_to_char(i: usize, sources: &HashSet<usize>) -> char {
        let u8 = i as u8 + if sources.contains(&i) {
            'A' as u8
        } else {
            'a' as u8
        };
        u8 as char
    }

}
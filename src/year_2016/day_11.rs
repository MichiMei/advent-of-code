use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use crate::errors::AoCError;

pub fn part_1(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let building = Building::from_input(input)?;

    find_minimal_solution(building).map(|result| result.to_string())
}

pub fn part_2(input: &Vec<String>) -> Result<String, AoCError<String>> {
    let additional_equip_strs = [
        "a elerium generator",
        "a elerium-compatible microchip",
        "a dilithium generator",
        "a dilithium-compatible microchip",
    ];
    let additional_equips = additional_equip_strs.iter()
        .map(|equip| Floor::parse_equipment(equip))
        .collect::<Result<Vec<_>, _>>()?;

    let mut building = Building::from_input(input)?;
    assert!(building.floors[0].add(&additional_equips));

    find_minimal_solution(building).map(|result| result.to_string())
}

fn find_minimal_solution(building: Building) -> Result<usize, AoCError<String>> {
    let mut cache = Cache::new();
    cache.cache_item(&building);
    let mut dequeue = VecDeque::new();
    dequeue.push_back((building, 0usize));

    while !dequeue.is_empty() {
        let (current, step) = dequeue.pop_front()
            .expect("Dequeue was tested to not be empty");
        if current.is_finished() {
            return Ok(step)
        }
        let possible_next = current.step();
        for elem in possible_next {
            if !cache.is_cached(&elem) {
                cache.cache_item(&elem);
                dequeue.push_back((elem, step + 1));
            }
        }
    }

    Err(AoCError::NoSolutionFoundError(
        "The algorithm got stuck with no further moves possible".to_string()))
}

struct Cache {
    cache: HashSet<(u8, Vec<(u8, u8)>)>,
}

impl Cache {
    pub fn new() -> Self {
        Self{cache: HashSet::new()}
    }

    pub fn cache_item(&mut self, building: &Building) -> bool {
        let cache_item = Self::building_to_cache_item(building);
        self.cache.insert(cache_item)
    }

    pub fn is_cached(&self, building: &Building) -> bool {
        let cache_item = Self::building_to_cache_item(building);
        self.cache.contains(&cache_item)
    }

    fn building_to_cache_item(building: &Building) -> (u8, Vec<(u8, u8)>) {
        let mut generator_map = HashMap::new();
        let mut microchip_map = HashMap::new();
        for (floor_index, floor) in building.floors.iter().enumerate() {
            for equipment in floor.equipments.iter() {
                match equipment {
                    Equipment::Generator(name) =>
                        assert_eq!(generator_map.insert(name, floor_index), None),
                    Equipment::Microchip(name) =>
                        assert_eq!(microchip_map.insert(name, floor_index), None),
                }
            }
        }
        let mut pairs = vec![];
        for (gen_name, gen_index) in generator_map {
            let micro_index = microchip_map.get(gen_name)
                .expect("A microchip for a generator is missing");
            pairs.push((gen_index as u8, *micro_index as u8));
        }
        pairs.sort_unstable();
        (building.elevator as u8, pairs)
    }
}

#[derive(Clone)]
struct Building {
    elevator: usize,
    floors: Vec<Floor>,
}

impl Building {
    pub fn from_input(input: &Vec<String>) -> Result<Self, AoCError<String>> {
        let mut floors = vec![Floor::default(); 4];
        for line in input {
            let (index, floor) = Floor::from_line(line)?;
            if index == 0 || index > 4 {
                return Err(AoCError::BadInputFormat(
                    format!("Only floors 1 to 4 supported. Found {}.", index)))
            }
            floors[index-1] = floor;
        }
        let elevator = 0;

        Ok(Self{elevator, floors})
    }

    pub fn is_valid(&self) -> bool {
        for floor in self.floors.iter() {
            if !floor.is_valid() {
                return false
            }
        }
        true
    }

    pub fn step(&self) -> Vec<Building> {
        let mut possible_buildings = vec![];
        let current_floor = self.elevator;
        let possible_removes = self.floors[current_floor].get_possible_steps();
        for possible_remove in possible_removes {
            if current_floor > 0 {
                let mut building = self.clone();
                if building.floors[current_floor].remove(&possible_remove) &&
                    building.floors[current_floor-1].add(&possible_remove) {
                    building.elevator = current_floor-1;
                    assert!(building.is_valid());
                    possible_buildings.push(building);
                }
            }
            if current_floor < 3 {
                let mut building = self.clone();
                if building.floors[current_floor].remove(&possible_remove) &&
                    building.floors[current_floor+1].add(&possible_remove) {
                    building.elevator = current_floor+1;
                    assert!(building.is_valid());
                    possible_buildings.push(building);
                }
            }
        }
        possible_buildings
    }

    pub fn is_finished(&self) -> bool {
        if self.elevator != 3 {
            return false
        }
        for floor in self.floors[0..3].iter() {
            if !floor.equipments.is_empty() {
                return false
            }
        }
        true
    }
}

impl Display for Building {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for (index, floor) in self.floors.iter().enumerate().rev() {
            str = format!("{}F{}", str, index+1);
            if self.elevator == index {
                str = format!("{}\tE", str);
            } else {
                str = format!("{}\t.", str);
            }

            str = format!("{}{}", str, floor);

            str = format!("{}\n", str);
        }

        write!(f, "{}", str)
    }
}

#[derive(Clone, Default)]
struct Floor {
    equipments: HashSet<Equipment>,
}

impl Floor {
    pub fn from_line(mut line: &str) -> Result<(usize, Self), AoCError<String>> {
        if !line.starts_with("The ") {
            return Err(AoCError::BadInputFormat("Expected a line like: 'The <index> floor contains \
                <list of equipment>'".to_string()))
        }
        line = &line[4..];

        let word_end = line.find(' ').ok_or_else(|| AoCError::BadInputFormat("Expected a \
            line like: 'The <index> floor contains <list of equipment>'".to_string()))?;
        let floor_index = Self::parse_floor_index(&line[0..word_end]).ok_or_else(
            || AoCError::BadInputFormat("Expected a line like: 'The <index> floor contains <list \
                of equipment>'".to_string()))?;
        line = &line[word_end+1..];

        if !line.starts_with("floor contains ") {
            return Err(AoCError::BadInputFormat("Expected a line like: 'The <index> floor contains \
                [<list of equipment>|nothing relevant].'".to_string()))
        }
        line = &line[15..];

        let mut equipments = HashSet::new();

        if line == "nothing relevant." {
            line = &line[0..0];
        }

        while let Some((equipment, remaining)) = Self::split_list(line) {
            line = remaining;
            equipments.insert(Self::parse_equipment(equipment)?);
        }

        let floor = Floor{equipments};

        Ok((floor_index, floor))
    }

    pub fn is_valid(&self) -> bool {
        let mut contains_generator = false;
        let mut contains_unprotected_microchip = false;
        for elem in self.equipments.iter() {
            match elem {
                Equipment::Generator(_) => contains_generator = true,
                Equipment::Microchip(name) => {
                    if !self.equipments.contains(&Equipment::Generator(name.to_string())) {
                        contains_unprotected_microchip = true;
                    }
                }
            }
        }
        !(contains_generator && contains_unprotected_microchip)
    }

    pub fn get_possible_steps(&self) -> Vec<Vec<Equipment>> {
        let mut possible_removes = vec![];
        for elem0 in self.equipments.iter() {
            possible_removes.push(vec![elem0.clone()]);
            for elem1 in self.equipments.iter() {
                if elem0 >= elem1 {
                    continue
                }
                possible_removes.push(vec![elem0.clone(), elem1.clone()]);
            }
        }
        possible_removes
    }

    pub fn remove(&mut self, removes: &Vec<Equipment>) -> bool {
        for elem in removes {
            if !self.equipments.remove(elem) {
                return false
            }
        }
        self.is_valid()
    }

    pub fn add(&mut self, additions: &Vec<Equipment>) -> bool {
        for elem in additions {
            if !self.equipments.insert(elem.clone()) {
                return false
            }
        }
        self.is_valid()
    }

    fn parse_floor_index(str: &str) -> Option<usize> {
        match str {
            "first" => Some(1),
            "second" => Some(2),
            "third" => Some(3),
            "fourth" => Some(4),
            _ => None,
        }
    }

    fn parse_equipment(str: &str) -> Result<Equipment, AoCError<String>> {
        let (a, content) = str.split_at(2);
        if a != "a " {
            return Err(AoCError::BadInputFormat(
                "Equipment should start with 'a '".to_string()))
        }

        let name = &content[0..2];

        let equipment = if str.contains("generator") {
            Equipment::Generator(name.to_string())
        } else if str.contains("microchip") {
            Equipment::Microchip(name.to_string())
        } else {
            return Err(AoCError::BadInputFormat(format!("Unknown equipment-type in '{}'", str)))
        };

        Ok(equipment)
    }

    fn split_list(str: &str) -> Option<(&str, &str)> {
        if str.is_empty() {
            return None
        }
        if let Some(index) = str.find(", and ") {
            let remaining = &str[0..index];
            let element = &str[index+6..];
            return Some((element, remaining))
        }
        if let Some(index) = str.find(" and ") {
            let element = &str[0..index];
            let remaining = &str[index+5..];
            return Some((element, remaining))
        }
        if let Some(index) = str.find(", ") {
            let element = &str[0..index];
            let remaining = &str[index+2..];
            return Some((element, remaining))
        }
        Some((str, &str[0..0]))
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for elem in self.equipments.iter() {
            str = format!("{}\t{}", str, elem);
        }
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Equipment {
    Generator(String),
    Microchip(String),
}

impl Display for Equipment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Equipment::Generator(name) => {
                write!(f, "{}G", name)
            }
            Equipment::Microchip(name) => {
                write!(f, "{}M", name)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "The first floor contains a hydrogen-compatible microchip and a lithium-compatible \
            microchip.".to_string(),
            "The second floor contains a hydrogen generator.".to_string(),
            "The third floor contains a lithium generator.".to_string(),
            "The fourth floor contains nothing relevant.".to_string(),
        ];

        assert_eq!(part_1(&v), Ok("11".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_11.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("47".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_11.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("71".to_string()));
        Ok(())
    }
}
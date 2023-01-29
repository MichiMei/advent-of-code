use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Character {
    pub(crate) hit_points: usize,
    damage: usize,
    pub(crate) armor: usize,
}

impl Character {
    pub fn new(hit_points: usize, damage: usize, armor: usize) -> Self {
        Self{hit_points, damage, armor}
    }

    pub fn from_input(input: &[String]) -> Option<Self> {
        if input.len() < 2 || input.len() > 3 {
            return None
        }
        let hit_points = Self::parse_val(&input[0])?;
        let damage = Self::parse_val(&input[1])?;
        let armor = if input.len() == 2 {
            0
        } else {
            Self::parse_val(&input[2])?
        };
        Some(Self::new(hit_points, damage, armor))
    }

    pub fn add_item(&mut self, item: &Item) {
        self.damage += item.damage;
        self.armor += item.armor;
    }

    pub fn attacked_by(&mut self, attacker: &Character) -> bool {
        let real_damage = attacker.damage.saturating_sub(self.armor);
        self.hit_points = self.hit_points.saturating_sub(real_damage);
        if self.hit_points == 0 {
            return false
        }
        true
    }

    fn parse_val(line: &str) -> Option<usize> {
        let words: Vec<&str> = line.split(": ").collect();
        if words.len() != 2 {
            return None
        }
        words[1].parse().ok()
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "hp: {}\tdmg: {}\tarmor: {}", self.hit_points, self.damage, self.armor)
    }
}

pub struct Item {
    pub(crate) cost: usize,
    pub(crate) damage: usize,
    pub(crate) armor: usize,
}
use std::cmp::{max, min};
use crate::year_2015::lib_2015::{Character, Item};

pub fn part_1(input: &[String]) -> Result<String, &str> {
    let boss = Character::from_input(input).ok_or(ERR_INPUT_MALFORMED)?;
    let player = Character::new(100, 0, 0);

    let cost = try_all_items(player, boss, minimum, true).ok_or(ERR_NO_POSSIBILITY_FOUND)?;

    Ok(cost.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, &str> {
    let boss = Character::from_input(input).ok_or(ERR_INPUT_MALFORMED)?;
    let player = Character::new(100, 0, 0);

    let cost = try_all_items(player, boss, maximum, false).ok_or(ERR_NO_POSSIBILITY_FOUND)?;

    Ok(cost.to_string())
}

type Comparator = fn(Option<usize>, Option<usize>) -> Option<usize>;

fn minimum(f: Option<usize>, s: Option<usize>) -> Option<usize> {
    if f.is_none() {
        return s
    }
    if s.is_none() {
        return f
    }
    Some(min(f.unwrap(), s.unwrap()))
}

fn maximum(f: Option<usize>, s: Option<usize>) -> Option<usize> {
    if f.is_none() {
        return s
    }
    if s.is_none() {
        return f
    }
    Some(max(f.unwrap(), s.unwrap()))
}

fn try_all_items(player: Character, boss: Character, compare: Comparator, player_wins: bool) -> Option<usize> {
    let mut min_cost = None;
    for weapon in WEAPONS {
        let mut new_player = player.clone();
        new_player.add_item(&weapon);
        let res = try_all_armors(new_player, boss.clone(), weapon.cost, compare, player_wins);
        min_cost = compare(min_cost, res);
    }
    min_cost
}

fn try_all_armors(player: Character, boss: Character, cost: usize, compare: Comparator, player_wins: bool) -> Option<usize> {
    let mut min_cost = try_all_rings(player.clone(), boss.clone(), cost, compare, player_wins);
    for armor in ARMORS {
        let mut new_player = player.clone();
        new_player.add_item(&armor);
        let res = try_all_rings(new_player, boss.clone(), cost+armor.cost, compare, player_wins);
        min_cost = compare(min_cost, res);
    }
    min_cost
}

fn try_all_rings(player: Character, boss: Character, cost: usize, compare: Comparator, player_wins: bool) -> Option<usize> {
    // try no ring
    let mut min_cost = if fight(player.clone(), boss.clone(), ) == player_wins {
        Some(cost)
    } else {
        None
    };
    for left_index in 0..RINGS.len() {
        let left_ring = &RINGS[left_index];

        // try 1 ring
        let mut new_player = player.clone();
        new_player.add_item(left_ring);
        let new_cost = cost+left_ring.cost;
        if fight(new_player, boss) == player_wins {
            min_cost = compare(min_cost, Some(new_cost));
        }

        for right_index in left_index+1..RINGS.len() {
            let right_ring = &RINGS[right_index];

            // try 2 rings
            let mut new_player = new_player.clone();
            new_player.add_item(right_ring);
            let new_cost = new_cost+right_ring.cost;
            if fight(new_player, boss) == player_wins {
                min_cost = compare(min_cost, Some(new_cost));
            }
        }
    }
    min_cost
}

fn fight(mut first: Character, mut second: Character) -> bool {
    loop {
        if !second.attacked_by(&first) {
            return true
        }
        if !first.attacked_by(&second) {
            return false
        }
    }
}

const WEAPONS: [Item; 5] = [
    Item{cost: 8, damage: 4, armor: 0},
    Item{cost: 10, damage: 5, armor: 0},
    Item{cost: 25, damage: 6, armor: 0},
    Item{cost: 40, damage: 7, armor: 0},
    Item{cost: 74, damage: 8, armor: 0},
];

const ARMORS: [Item; 5] = [
    Item{cost: 13, damage: 0, armor: 1},
    Item{cost: 31, damage: 0, armor: 2},
    Item{cost: 53, damage: 0, armor: 3},
    Item{cost: 75, damage: 0, armor: 4},
    Item{cost: 102, damage: 0, armor: 5},
];

const RINGS: [Item; 6] = [
    Item{cost: 25, damage: 1, armor: 0},
    Item{cost: 50, damage: 2, armor: 0},
    Item{cost: 100, damage: 3, armor: 0},
    Item{cost: 20, damage: 0, armor: 1},
    Item{cost: 40, damage: 0, armor: 2},
    Item{cost: 80, damage: 0, armor: 3},
];

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";
const ERR_NO_POSSIBILITY_FOUND: &str = "No possible solution was found";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let player = Character::new(8, 5, 5);
        let boss = Character::new(12, 7, 2);

        assert!(fight(player, boss));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_21.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("111".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_21.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("188".to_string()));
        Ok(())
    }
}
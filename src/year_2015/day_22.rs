use std::collections::HashSet;
use crate::year_2015::lib_2015::Character;

pub fn part_1(input: &[String]) -> Result<String, &str> {
    let player = Character::new(50, 0, 0);
    let boss = Character::from_input(input).ok_or(ERR_INPUT_MALFORMED)?;
    let round = Round::new(player, boss, 500, false);
    let res = get_min_mana(round).ok_or(ERR_NO_POSSIBILITY_FOUND)?;
    Ok(res.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, &str> {
    let player = Character::new(50, 0, 0);
    let boss = Character::from_input(input).ok_or(ERR_INPUT_MALFORMED)?;
    let round = Round::new(player, boss, 500, true);
    let res = get_min_mana(round).ok_or(ERR_NO_POSSIBILITY_FOUND)?;
    Ok(res.to_string())
}

fn get_min_mana(initial: Round) -> Option<usize> {
    let mut current = HashSet::new();
    let mut next = HashSet::new();
    current.insert(initial);

    let mut minimum = None;

    while !current.is_empty() {
        //println!("starting with {} parallel realities", current.len());
        for current in current {
            if current.finished() {
                if current.player_won() && (minimum.is_none() || current.mana_spend < minimum.unwrap()) {
                    minimum = Some(current.mana_spend);
                }
                continue
            }

            if minimum.is_some() && current.mana_spend >= minimum.unwrap() {
                continue
            }

            next.extend(current.play_round());

        }

        current = next;
        next = HashSet::new();
    }

    minimum
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Round {
    player: Wizard,
    boss: Character,
    mana_spend: usize,
    players_turn: bool,
    dbg: String,
    hard_mode: bool,
}

impl Round {
    fn new(player: Character, boss: Character, mana: usize, hard_mode: bool) -> Self {
        let player = Wizard::from_character(player, mana);
        Self{player, boss, mana_spend: 0, players_turn: true, dbg: String::new(), hard_mode}
    }

    fn finished(&self) -> bool {
        self.player_won() || self.boss_won()
    }

    fn player_won(&self) -> bool {
        self.boss.hit_points == 0
    }

    fn boss_won(&self) -> bool {
        self.player.player.hit_points == 0
    }

    fn play_round(mut self) -> HashSet<Self> {
        if self.hard_mode && self.players_turn {
            self.player.player.hit_points -= 1;
            if self.boss_won() {
                let mut res = HashSet::new();
                res.insert(self);
                return res
            }
        }

        self.apply_effects();
        if self.finished() {
            let mut res = HashSet::new();
            res.insert(self);
            return res
        }

        if self.players_turn {
            self.play_player_round()
        } else {
            self.play_boss_round()
        }
    }

    fn play_boss_round(mut self) -> HashSet<Self> {
        self.player.player.attacked_by(&self.boss);
        self.players_turn = !self.players_turn;
        self.dbg = format!("{} b", self.dbg);
        let mut res = HashSet::new();
        res.insert(self);
        res
    }

    fn play_player_round(mut self) -> HashSet<Self> {
        let mut res = HashSet::new();

        let mut tmp = self.clone();
        if tmp.player.magic_missile(&mut tmp.boss, &mut tmp.mana_spend) {
            tmp.players_turn = !tmp.players_turn;
            tmp.dbg = format!("{} p(m)", tmp.dbg);
            res.insert(tmp);
        }

        let mut tmp = self.clone();
        if tmp.player.drain(&mut tmp.boss, &mut tmp.mana_spend) {
            tmp.players_turn = !tmp.players_turn;
            tmp.dbg = format!("{} p(d)", tmp.dbg);
            res.insert(tmp);
        }

        let mut tmp = self.clone();
        if tmp.player.shield(&mut tmp.mana_spend) {
            tmp.players_turn = !tmp.players_turn;
            tmp.dbg = format!("{} p(s)", tmp.dbg);
            res.insert(tmp);
        }

        let mut tmp = self.clone();
        if tmp.player.poison(&mut tmp.mana_spend) {
            tmp.players_turn = !tmp.players_turn;
            tmp.dbg = format!("{} p(p)", tmp.dbg);
            res.insert(tmp);
        }

        if self.player.recharge(&mut self.mana_spend) {
            self.players_turn = !self.players_turn;
            self.dbg = format!("{} p(r)", self.dbg);
            res.insert(self);
        }

        res
    }

    fn apply_effects(&mut self) {
        match self.player.effect_durations[SHIELD] {
            0 => self.player.player.armor = 0,
            _ => self.player.effect_durations[SHIELD] -= 1,
        }

        if self.player.effect_durations[POISON] > 0 {
            self.player.apply_poison(&mut self.boss);
        }

        if self.player.effect_durations[RECHARGE] > 0 {
            self.player.apply_recharge();
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Wizard {
    mana: usize,
    effect_durations: [usize; 3],
    player: Character,
}

impl Wizard {
    fn from_character(player: Character, mana: usize) -> Self {
        Wizard {mana, effect_durations: [0; 3], player}
    }

    fn magic_missile(&mut self, boss: &mut Character, mana_spend: &mut usize) -> bool {
        if self.mana >= 53 {
            self.mana -= 53;
            *mana_spend += 53;
            if boss.hit_points > 4 {
                boss.hit_points -= 4;
            } else {
                boss.hit_points = 0;
            }
            return true
        }
        false
    }

    fn drain(&mut self, boss: &mut Character, mana_spend: &mut usize) -> bool {
        if self.mana >= 73 {
            self.mana -= 73;
            *mana_spend += 73;
            self.player.hit_points += 2;
            if boss.hit_points > 2 {
                boss.hit_points -= 2;
            } else {
                boss.hit_points = 0;
            }
            return true
        }
        false
    }

    fn shield(&mut self, mana_spend: &mut usize) -> bool {
        if self.mana >= 113 && self.effect_durations[SHIELD] == 0{
            self.mana -= 113;
            *mana_spend += 113;
            self.effect_durations[SHIELD] = 6;
            self.player.armor = 7;
            return true
        }
        false
    }

    fn poison(&mut self, mana_spend: &mut usize) -> bool {
        if self.mana >= 173 && self.effect_durations[POISON] == 0{
            self.mana -= 173;
            *mana_spend += 173;
            self.effect_durations[POISON] = 6;
            return true
        }
        false
    }

    fn recharge(&mut self, mana_spend: &mut usize) -> bool {
        if self.mana >= 229 && self.effect_durations[RECHARGE] == 0{
            self.mana -= 229;
            *mana_spend += 229;
            self.effect_durations[RECHARGE] = 5;
            return true
        }
        false
    }

    fn apply_poison(&mut self, boss: &mut Character) {
        self.effect_durations[POISON] -= 1;
        if boss.hit_points > 3 {
            boss.hit_points -= 3;
        } else {
            boss.hit_points = 0;
        }
    }

    fn apply_recharge(&mut self) {
        self.effect_durations[RECHARGE] -= 1;
        self.mana += 101;
    }
}

const SHIELD: usize = 0;
const POISON: usize = 1;
const RECHARGE: usize = 2;

const ERR_INPUT_MALFORMED: &str = "Input string is malformed";
const ERR_NO_POSSIBILITY_FOUND: &str = "No possible solution was found";

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let player = Character::new(10, 0, 0);
        let boss = Character::new(13, 8, 0);
        let round = Round::new(player, boss, 250, false);

        assert_eq!(get_min_mana(round), Some(226));

        let player = Character::new(10, 0, 0);
        let boss = Character::new(14, 8, 0);
        let round = Round::new(player, boss, 250, false);

        assert_eq!(get_min_mana(round), Some(641));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_22.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("953".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_22.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("1289".to_string()));
        Ok(())
    }
}
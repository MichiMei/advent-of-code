use std::cmp::max;
use crate::errors::AoCError;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let recipe = Recipe::from(input)?;
    let res = find_optimum(&recipe, &mut vec![]);
    Ok(res.to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let recipe = Recipe::from(input)?;
    let res = find_optimum_500_calories(&recipe, &mut vec![]);
    Ok(res.to_string())
}

fn find_optimum(recipe: &Recipe, amounts: &mut Vec<i32>) -> i32 {
    if amounts.len() == recipe.properties.len()-1 {
        let sum: i32 = amounts.iter().sum();
        let remaining = 100-sum;
        amounts.push(remaining);
        let res = recipe.calc_score(amounts);
        amounts.pop();
        return res
    }

    let sum: i32 = amounts.iter().sum();
    let remaining = 100-sum;
    let index = amounts.len();
    amounts.push(0);

    let mut maximum = 0;
    for amount in 0..=remaining {
        amounts[index] = amount;
        let res = find_optimum(recipe, amounts);
        maximum = max(maximum, res);
    }

    amounts.pop();

    maximum
}

fn find_optimum_500_calories(recipe: &Recipe, amounts: &mut Vec<i32>) -> i32 {
    if amounts.len() == recipe.properties.len()-1 {
        let sum: i32 = amounts.iter().sum();
        let remaining = 100-sum;
        amounts.push(remaining);
        let res = if recipe.get_calories(amounts) == 500 {
            recipe.calc_score(amounts)
        } else {
            0
        };
        amounts.pop();
        return res
    }

    let sum: i32 = amounts.iter().sum();
    let remaining = 100-sum;
    let index = amounts.len();
    amounts.push(0);

    let mut maximum = 0;
    for amount in 0..=remaining {
        amounts[index] = amount;
        let res = find_optimum_500_calories(recipe, amounts);
        maximum = max(maximum, res);
    }

    amounts.pop();

    maximum
}

struct Recipe {
    properties: Vec<[i32; 5]>,
}

impl Recipe {
    fn from(input: &[String]) -> Result<Self, AoCError<String>> {
        let mut properties = vec![];
        for line in input {
           properties.push(parse_ingredient(line)?);
        }

        Ok(Self{properties})
    }

    fn calc_score(&self, amounts: &[i32]) -> i32 {
        let mut scores = self.get_prop_scores(amounts);

        let mut product = 1;
        for index in 0..scores.len()-1 {
            if scores[index] < 0 {
                scores[index] = 0;
            }
            product *= scores[index];
        }

        product
    }

    fn get_calories(&self, amounts: &[i32]) -> i32 {
        let scores = self.get_prop_scores(amounts);
        scores[4]
    }

    fn get_prop_scores(&self, amounts: &[i32]) -> [i32; 5] {
        let mut scores = [0; 5];
        for (prop_index, property) in self.properties.iter().enumerate() {
            for index in 0..scores.len() {
                scores[index] += amounts[prop_index]*property[index];
            }
        }
        scores
    }
}

fn parse_ingredient(line: &str) -> Result<[i32; 5], AoCError<String>> {
    let words: Vec<&str> = line.split(' ').collect();
    if words.len() != 11 {
        return Err(AoCError::BadInputFormat(
            format!("Unexpected input line.\nExpected '<name>: capacity <val>, durability <val>, \
            flavor <val>, texture <val>, calories <val>'.\nFound '{}'", line)
        ))
    }
    let capacity = words[2];
    let capacity = &capacity[0..capacity.len()-1].
        parse().map_err(|_| AoCError::BadInputFormat(
        format!("Unexpected input line.\nExpected '<name>: capacity <val>, durability <val>, \
            flavor <val>, texture <val>, calories <val>'.\nFound '{}'", line)
    ))?;

    let durability = words[4];
    let durability = &durability[0..durability.len()-1].
        parse().map_err(|_| AoCError::BadInputFormat(
        format!("Unexpected input line.\nExpected '<name>: capacity <val>, durability <val>, \
            flavor <val>, texture <val>, calories <val>'.\nFound '{}'", line)
    ))?;

    let flavor = words[6];
    let flavor = &flavor[0..flavor.len()-1].
        parse().map_err(|_| AoCError::BadInputFormat(
        format!("Unexpected input line.\nExpected '<name>: capacity <val>, durability <val>, \
            flavor <val>, texture <val>, calories <val>'.\nFound '{}'", line)
    ))?;

    let texture = words[8];
    let texture = &texture[0..texture.len()-1].
        parse().map_err(|_| AoCError::BadInputFormat(
        format!("Unexpected input line.\nExpected '<name>: capacity <val>, durability <val>, \
            flavor <val>, texture <val>, calories <val>'.\nFound '{}'", line)
    ))?;

    let calories = words[10].parse::<i32>().map_err(|_| AoCError::BadInputFormat(
        format!("Unexpected input line.\nExpected '<name>: capacity <val>, durability <val>, \
            flavor <val>, texture <val>, calories <val>'.\nFound '{}'", line)
    ))?;

    Ok([*capacity, *durability, *flavor, *texture, calories])
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_recipe_calc_score() -> Result<(), AoCError<String>> {
        let v = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8".to_string(),
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".to_string()
        ];
        let recipe = Recipe::from(&v)?;
        assert_eq!(recipe.calc_score(&[44, 56]), 62842880);
        Ok(())
    }

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8".to_string(),
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".to_string()
        ];
        assert_eq!(part_1(&v), Ok("62842880".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_15.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("18965440".to_string()));
        Ok(())
    }

    #[test]
    fn check_examples_part_2() {
        let v = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8".to_string(),
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".to_string()
        ];
        assert_eq!(part_2(&v), Ok("57600000".to_string()));
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2015/input_day_15.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("15862900".to_string()));
        Ok(())
    }
}
use crate::errors::AoCError;
use crate::year_2016::lib_2016::assembunny::AssembunnySimulator;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut sim = AssembunnySimulator::from_input(input)?;
    sim.set_registers([7, 0, 0, 0]);
    sim.optimize();
    let registers = sim.run();

    Ok(registers[0].to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut sim = AssembunnySimulator::from_input(input)?;
    sim.set_registers([12, 0, 0, 0]);
    sim.optimize();
    let registers = sim.run();

    Ok(registers[0].to_string())
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "cpy 2 a".to_string(),
            "tgl a".to_string(),
            "tgl a".to_string(),
            "tgl a".to_string(),
            "cpy 1 a".to_string(),
            "dec a".to_string(),
            "dec a".to_string(),
        ];

        assert_eq!(part_1(&v), Ok("3".to_string()));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 23)?;
        assert_eq!(part_1(&input), Ok("12654".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 23)?;
        assert_eq!(part_2(&input), Ok("479009214".to_string()));
        Ok(())
    }
}
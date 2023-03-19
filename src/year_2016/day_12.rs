use crate::errors::AoCError;
use crate::year_2016::lib_2016::assembunny::AssembunnySimulator;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut sim = AssembunnySimulator::from_input(input)?;
    let registers = sim.run();

    Ok(registers[0].to_string())
}

pub fn part_2(input: &[String]) -> Result<String, AoCError<String>> {
    let mut sim = AssembunnySimulator::from_input(input)?;
    sim.set_registers([0, 0, 1, 0]);
    let registers = sim.run();

    Ok(registers[0].to_string())
}

#[cfg(test)]
mod test {
    use crate::read_lines_untrimmed_from_file;
    use super::*;

    #[test]
    fn check_examples_part_1() {
        let v = vec![
            "cpy 41 a".to_string(),
            "inc a".to_string(),
            "inc a".to_string(),
            "dec a".to_string(),
            "jnz a 2".to_string(),
            "dec a".to_string(),
        ];

        assert_eq!(part_1(&v), Ok("42".to_string()));
    }

    #[test]
    fn check_input_part_1() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_12.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_1(&input), Ok("318007".to_string()));
        Ok(())
    }

    #[test]
    fn check_input_part_2() -> std::io::Result<()> {
        let input_name = "input/year_2016/input_day_12.txt";
        let input = read_lines_untrimmed_from_file(input_name)?;

        assert_eq!(part_2(&input), Ok("9227661".to_string()));
        Ok(())
    }
}
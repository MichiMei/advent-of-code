use crate::errors::AoCError;
use crate::year_2016::lib_2016::assembunny::AssembunnySimulator;

pub fn part_1(input: &[String]) -> Result<String, AoCError<String>> {
    let mut input_int = 0;
    loop {
        println!("testing {}", input_int);
        let mut sim = AssembunnySimulator::from_input(input)?;
        sim.optimize();
        sim.set_registers([input_int, 0, 0, 0]);
        let (_, output, loop_indicator) = sim.run_until_loop();

        if let Some(loop_indicator) = loop_indicator {
            if check_output_validity(output, loop_indicator) {
                break
            }
        }
        input_int += 1;
    }

    Ok(input_int.to_string())
}

pub fn part_2(_: &[String]) -> Result<String, AoCError<String>> {
    Ok("Merry Christmas!".to_string())
}

fn check_output_validity(output: &[i32], loop_indicator: usize) -> bool {
    if output.len() < 2 {
        return false
    }
    if output.chunks(2)
        .filter(|chunk| !(chunk.len() == 2 && chunk[0] == 0 && chunk[1] == 1))
        .filter(|chunk| !(chunk.len() == 1 && chunk[0] == 0))
        .count() != 0 {
        return false
    }
    if output[loop_indicator..].len()%2 != 0 {
        return false
    }
    if output[loop_indicator..].chunks(2)
        .filter(|chunk| !(chunk.len() == 2 && chunk[0] != chunk[1]))
        .count() != 0 {
        return false
    }
    true
}

#[cfg(test)]
mod test {
    use crate::input::get_input;
    use super::*;

    #[test]
    fn check_check_output_validity() {
        let v = [0,1,0,1,0,1,0,1,0,1,0,1];
        assert!(check_output_validity(&v, 0));

        let v = [0,1,0,1,0,1,0,1,0,1,0,1,0];
        assert!(!check_output_validity(&v, 0));

        let v = [0,1,0,1,0,0,1,0,1,0,1,0,1];
        assert!(!check_output_validity(&v, 0));
    }

    #[test]
    fn check_input_part_1() -> Result<(), AoCError<String>> {
        let input = get_input(2016, 25)?;
        assert_eq!(part_1(&input), Ok("182".to_string()));
        Ok(())
    }
}
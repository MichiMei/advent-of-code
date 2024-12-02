use advent_of_code::{year_2015, year_2016, year_2017, year_2023, year_2024};
use advent_of_code::errors::AoCError;
use advent_of_code::input::get_input;


#[allow(dead_code)]
fn run_2015() -> Result<(), AoCError<String>> {
    use year_2015::*;
    let year = 2015;

    // day 01
    let input = get_input(year, 1)?;
    println!("y{}-d01_p1: {}", year, day_01::part_1(&input).unwrap());
    println!("y{}-d01_p2: {}", year, day_01::part_2(&input).unwrap());

    // day02
    let input = get_input(year, 2)?;
    println!("y{}-d02_p1: {}", year, day_02::part_1(&input).unwrap());
    println!("y{}-d02_p2: {}", year, day_02::part_2(&input).unwrap());

    // day03
    let input = get_input(year, 3)?;
    println!("y{}-d03_p1: {}", year, day_03::part_1(&input).unwrap());
    println!("y{}-d03_p2: {}", year, day_03::part_2(&input).unwrap());

    // day04
    let input = get_input(year, 4)?;
    println!("y{}-d04_p1: {}", year, day_04::part_1(&input).unwrap());
    println!("y{}-d04_p2: {}", year, day_04::part_2(&input).unwrap());

    // day05
    let input = get_input(year, 5)?;
    println!("y{}-d05_p1: {}", year, day_05::part_1(&input).unwrap());
    println!("y{}-d05_p2: {}", year, day_05::part_2(&input).unwrap());

    // day06
    let input = get_input(year, 6)?;
    println!("y{}-d06_p1: {}", year, day_06::part_1(&input).unwrap());
    println!("y{}-d06_p2: {}", year, day_06::part_2(&input).unwrap());

    // day07
    let input = get_input(year, 7)?;
    println!("y{}-d07_p1: {}", year, day_07::part_1(&input).unwrap());
    println!("y{}-d07_p2: {}", year, day_07::part_2(&input).unwrap());

    // day08
    let input = get_input(year, 8)?;
    println!("y{}-d08_p1: {}", year, day_08::part_1(&input).unwrap());
    println!("y{}-d08_p2: {}", year, day_08::part_2(&input).unwrap());

    // day09
    let input = get_input(year, 9)?;
    println!("y{}-d09_p1: {}", year, day_09::part_1(&input).unwrap());
    println!("y{}-d09_p2: {}", year, day_09::part_2(&input).unwrap());

    // day10
    let input = get_input(year, 10)?;
    println!("y{}-d10_p1: {}", year, day_10::part_1(&input).unwrap());
    println!("y{}-d10_p2: {}", year, day_10::part_2(&input).unwrap());

    // day11
    let input = get_input(year, 11)?;
    println!("y{}-d11_p1: {}", year, day_11::part_1(&input).unwrap());
    println!("y{}-d11_p2: {}", year, day_11::part_2(&input).unwrap());

    // day12
    let input = get_input(year, 12)?;
    println!("y{}-d12_p1: {}", year, day_12::part_1(&input).unwrap());
    println!("y{}-d12_p2: {}", year, day_12::part_2(&input).unwrap());

    // day13
    let input = get_input(year, 13)?;
    println!("y{}-d13_p1: {}", year, day_13::part_1(&input).unwrap());
    println!("y{}-d13_p2: {}", year, day_13::part_2(&input).unwrap());

    // day14
    let input = get_input(year, 14)?;
    println!("y{}-d14_p1: {}", year, day_14::part_1(&input).unwrap());
    println!("y{}-d14_p2: {}", year, day_14::part_2(&input).unwrap());

    // day15
    let input = get_input(year, 15)?;
    println!("y{}-d15_p1: {}", year, day_15::part_1(&input).unwrap());
    println!("y{}-d15_p2: {}", year, day_15::part_2(&input).unwrap());

    // day16
    let input = get_input(year, 16)?;
    println!("y{}-d16_p1: {}", year, day_16::part_1(&input).unwrap());
    println!("y{}-d16_p2: {}", year, day_16::part_2(&input).unwrap());

    // day17
    let input = get_input(year, 17)?;
    println!("y{}-d17_p1: {}", year, day_17::part_1(&input).unwrap());
    println!("y{}-d17_p2: {}", year, day_17::part_2(&input).unwrap());

    // day18
    let input = get_input(year, 18)?;
    println!("y{}-d18_p1: {}", year, day_18::part_1(&input).unwrap());
    println!("y{}-d18_p2: {}", year, day_18::part_2(&input).unwrap());

    // day19
    let input = get_input(year, 19)?;
    println!("y{}-d19_p1: {}", year, day_19::part_1(&input).unwrap());
    println!("y{}-d19_p2: {}", year, day_19::part_2(&input).unwrap());

    // day20
    let input = get_input(year, 20)?;
    println!("y{}-d20_p1: {}", year, day_20::part_1(&input).unwrap());
    println!("y{}-d20_p2: {}", year, day_20::part_2(&input).unwrap());

    // day21
    let input = get_input(year, 21)?;
    println!("y{}-d21_p1: {}", year, day_21::part_1(&input).unwrap());
    println!("y{}-d21_p2: {}", year, day_21::part_2(&input).unwrap());

    // day22
    let input = get_input(year, 22)?;
    println!("y{}-d22_p1: {}", year, day_22::part_1(&input).unwrap());
    println!("y{}-d22_p2: {}", year, day_22::part_2(&input).unwrap());

    // day23
    let input = get_input(year, 23)?;
    println!("y{}-d23_p1: {}", year, day_23::part_1(&input).unwrap());
    println!("y{}-d23_p2: {}", year, day_23::part_2(&input).unwrap());

    // day24
    let input = get_input(year, 24)?;
    println!("y{}-d24_p1: {}", year, day_24::part_1(&input).unwrap());
    println!("y{}-d24_p2: {}", year, day_24::part_2(&input).unwrap());

    // day25
    let input = get_input(year, 25)?;
    println!("y{}-d25_p1: {}", year, day_25::part_1(&input).unwrap());
    println!("y{}-d25_p2: {}", year, day_25::part_2(&input).unwrap());

    Ok(())
}

#[allow(dead_code)]
fn run_2016() -> Result<(), AoCError<String>> {
    use year_2016::*;
    let year = 2016;

    // day 01
    let input = get_input(year, 1)?;
    println!("y{}-d01_p1: {}", year, day_01::part_1(&input).unwrap());
    println!("y{}-d01_p2: {}", year, day_01::part_2(&input).unwrap());
    
    // day02
    let input = get_input(year, 2)?;
    println!("y{}-d02_p1: {}", year, day_02::part_1(&input).unwrap());
    println!("y{}-d02_p2: {}", year, day_02::part_2(&input).unwrap());

    // day03
    let input = get_input(year, 3)?;
    println!("y{}-d03_p1: {}", year, day_03::part_1(&input).unwrap());
    println!("y{}-d03_p2: {}", year, day_03::part_2(&input).unwrap());

    // day04
    let input = get_input(year, 4)?;
    println!("y{}-d04_p1: {}", year, day_04::part_1(&input).unwrap());
    println!("y{}-d04_p2: {}", year, day_04::part_2(&input).unwrap());

    // day05
    let input = get_input(year, 5)?;
    println!("y{}-d05_p1: {}", year, day_05::part_1(&input).unwrap());
    println!("y{}-d05_p2: {}", year, day_05::part_2(&input).unwrap());

    // day06
    let input = get_input(year, 6)?;
    println!("y{}-d06_p1: {}", year, day_06::part_1(&input).unwrap());
    println!("y{}-d06_p2: {}", year, day_06::part_2(&input).unwrap());

    // day07
    let input = get_input(year, 7)?;
    println!("y{}-d07_p1: {}", year, day_07::part_1(&input).unwrap());
    println!("y{}-d07_p2: {}", year, day_07::part_2(&input).unwrap());
    
    // day08
    let input = get_input(year, 8)?;
    println!("y{}-d08_p1: {}", year, day_08::part_1(&input).unwrap());
    println!("y{}-d08_p2: {}", year, day_08::part_2(&input).unwrap());

    // day09
    let input = get_input(year, 9)?;
    println!("y{}-d09_p1: {}", year, day_09::part_1(&input).unwrap());
    println!("y{}-d09_p2: {}", year, day_09::part_2(&input).unwrap());

    // day10
    let input = get_input(year, 10)?;
    println!("y{}-d10_p1: {}", year, day_10::part_1(&input).unwrap());
    println!("y{}-d10_p2: {}", year, day_10::part_2(&input).unwrap());

    // day11
    let input = get_input(year, 11)?;
    println!("y{}-d11_p1: {}", year, day_11::part_1(&input).unwrap());
    println!("y{}-d11_p2: {}", year, day_11::part_2(&input).unwrap());

    // day12
    let input = get_input(year, 12)?;
    println!("y{}-d12_p1: {}", year, day_12::part_1(&input).unwrap());
    println!("y{}-d12_p2: {}", year, day_12::part_2(&input).unwrap());

    // day13
    let input = get_input(year, 13)?;
    println!("y{}-d13_p1: {}", year, day_13::part_1(&input).unwrap());
    println!("y{}-d13_p2: {}", year, day_13::part_2(&input).unwrap());

    // day14
    let input = get_input(year, 14)?;
    println!("y{}-d14_p1: {}", year, day_14::part_1(&input).unwrap());
    println!("y{}-d14_p2: {}", year, day_14::part_2(&input).unwrap());

    // day15
    let input = get_input(year, 15)?;
    println!("y{}-d15_p1: {}", year, day_15::part_1(&input).unwrap());
    println!("y{}-d15_p2: {}", year, day_15::part_2(&input).unwrap());

    // day16
    let input = get_input(year, 16)?;
    println!("y{}-d16_p1: {}", year, day_16::part_1(&input).unwrap());
    println!("y{}-d16_p2: {}", year, day_16::part_2(&input).unwrap());

    // day17
    let input = get_input(year, 17)?;
    println!("y{}-d17_p1: {}", year, day_17::part_1(&input).unwrap());
    println!("y{}-d17_p2: {}", year, day_17::part_2(&input).unwrap());

    // day18
    let input = get_input(year, 18)?;
    println!("y{}-d18_p1: {}", year, day_18::part_1(&input).unwrap());
    println!("y{}-d18_p2: {}", year, day_18::part_2(&input).unwrap());

    // day19
    let input = get_input(year, 19)?;
    println!("y{}-d19_p1: {}", year, day_19::part_1(&input).unwrap());
    println!("y{}-d19_p2: {}", year, day_19::part_2(&input).unwrap());

    // day20
    let input = get_input(year, 20)?;
    println!("y{}-d20_p1: {}", year, day_20::part_1(&input).unwrap());
    println!("y{}-d20_p2: {}", year, day_20::part_2(&input).unwrap());

    // day21
    let input = get_input(year, 21)?;
    println!("y{}-d21_p1: {}", year, day_21::part_1(&input).unwrap());
    println!("y{}-d21_p2: {}", year, day_21::part_2(&input).unwrap());

    // day22
    let input = get_input(year, 22)?;
    println!("y{}-d22_p1: {}", year, day_22::part_1(&input).unwrap());
    println!("y{}-d22_p2: {}", year, day_22::part_2(&input).unwrap());

    // day23
    let input = get_input(year, 23)?;
    println!("y{}-d23_p1: {}", year, day_23::part_1(&input).unwrap());
    println!("y{}-d23_p2: {}", year, day_23::part_2(&input).unwrap());

    // day24
    let input = get_input(year, 24)?;
    println!("y{}-d24_p1: {}", year, day_24::part_1(&input).unwrap());
    println!("y{}-d24_p2: {}", year, day_24::part_2(&input).unwrap());

    // day25
    let input = get_input(year, 25)?;
    println!("y{}-d25_p1: {}", year, day_25::part_1(&input).unwrap());
    println!("y{}-d25_p2: {}", year, day_25::part_2(&input).unwrap());

    Ok(())
}

#[allow(dead_code)]
fn run_2017() -> Result<(), AoCError<String>> {
    use year_2017::*;
    let year = 2017;

    // day 01
    let input = get_input(year, 1)?;
    println!("y{}-d01_p1: {}", year, day_01::part_1(&input).unwrap());
    println!("y{}-d01_p2: {}", year, day_01::part_2(&input).unwrap());

    // day02
    let input = get_input(year, 2)?;
    println!("y{}-d02_p1: {}", year, day_02::part_1(&input).unwrap());
    println!("y{}-d02_p2: {}", year, day_02::part_2(&input).unwrap());

    // day03
    let input = get_input(year, 3)?;
    println!("y{}-d03_p1: {}", year, day_03::part_1(&input).unwrap());
    println!("y{}-d03_p2: {}", year, day_03::part_2(&input).unwrap());

    // day04
    let input = get_input(year, 4)?;
    println!("y{}-d04_p1: {}", year, day_04::part_1(&input).unwrap());
    println!("y{}-d04_p2: {}", year, day_04::part_2(&input).unwrap());

    // day05
    let input = get_input(year, 5)?;
    println!("y{}-d05_p1: {}", year, day_05::part_1(&input).unwrap());
    println!("y{}-d05_p2: {}", year, day_05::part_2(&input).unwrap());

    // day06
    let input = get_input(year, 6)?;
    println!("y{}-d06_p1: {}", year, day_06::part_1(&input).unwrap());
    println!("y{}-d06_p2: {}", year, day_06::part_2(&input).unwrap());

    // day07
    let input = get_input(year, 7)?;
    println!("y{}-d07_p1: {}", year, day_07::part_1(&input).unwrap());
    println!("y{}-d07_p2: {}", year, day_07::part_2(&input).unwrap());

    // day08
    let input = get_input(year, 8)?;
    println!("y{}-d08_p1: {}", year, day_08::part_1(&input).unwrap());
    println!("y{}-d08_p2: {}", year, day_08::part_2(&input).unwrap());

    // day09
    let input = get_input(year, 9)?;
    println!("y{}-d09_p1: {}", year, day_09::part_1(&input).unwrap());
    println!("y{}-d09_p2: {}", year, day_09::part_2(&input).unwrap());

    // day10
    let input = get_input(year, 10)?;
    println!("y{}-d10_p1: {}", year, day_10::part_1(&input).unwrap());
    println!("y{}-d10_p2: {}", year, day_10::part_2(&input).unwrap());

    // day11
    let input = get_input(year, 11)?;
    println!("y{}-d11_p1: {}", year, day_11::part_1(&input).unwrap());
    println!("y{}-d11_p2: {}", year, day_11::part_2(&input).unwrap());

    // day12
    let input = get_input(year, 12)?;
    println!("y{}-d12_p1: {}", year, day_12::part_1(&input).unwrap());
    println!("y{}-d12_p2: {}", year, day_12::part_2(&input).unwrap());

    // day13
    let input = get_input(year, 13)?;
    println!("y{}-d13_p1: {}", year, day_13::part_1(&input).unwrap());
    println!("y{}-d13_p2: {}", year, day_13::part_2(&input).unwrap());

    // day14
    let input = get_input(year, 14)?;
    println!("y{}-d14_p1: {}", year, day_14::part_1(&input).unwrap());
    println!("y{}-d14_p2: {}", year, day_14::part_2(&input).unwrap());

    // day15
    let input = get_input(year, 15)?;
    println!("y{}-d15_p1: {}", year, day_15::part_1(&input).unwrap());
    println!("y{}-d15_p2: {}", year, day_15::part_2(&input).unwrap());

    // day16
    let input = get_input(year, 16)?;
    println!("y{}-d16_p1: {}", year, day_16::part_1(&input).unwrap());
    println!("y{}-d16_p2: {}", year, day_16::part_2(&input).unwrap());

    // day17
    let input = get_input(year, 17)?;
    println!("y{}-d17_p1: {}", year, day_17::part_1(&input).unwrap());
    println!("y{}-d17_p2: {}", year, day_17::part_2(&input).unwrap());

    // day18
    let input = get_input(year, 18)?;
    println!("y{}-d18_p1: {}", year, day_18::part_1(&input).unwrap());
    println!("y{}-d18_p2: {}", year, day_18::part_2(&input).unwrap());

    // day19
    let input = get_input(year, 19)?;
    println!("y{}-d19_p1: {}", year, day_19::part_1(&input).unwrap());
    println!("y{}-d19_p2: {}", year, day_19::part_2(&input).unwrap());

    // day20
    let input = get_input(year, 20)?;
    println!("y{}-d20_p1: {}", year, day_20::part_1(&input).unwrap());
    println!("y{}-d20_p2: {}", year, day_20::part_2(&input).unwrap());

    // day21
    let input = get_input(year, 21)?;
    println!("y{}-d21_p1: {}", year, day_21::part_1(&input).unwrap());
    println!("y{}-d21_p2: {}", year, day_21::part_2(&input).unwrap());

    // day22
    let input = get_input(year, 22)?;
    println!("y{}-d22_p1: {}", year, day_22::part_1(&input).unwrap());
    println!("y{}-d22_p2: {}", year, day_22::part_2(&input).unwrap());
/*
    // day23
    let input = get_input(year, 23)?;
    println!("y{}-d23_p1: {}", year, day_23::part_1(&input).unwrap());
    println!("y{}-d23_p2: {}", year, day_23::part_2(&input).unwrap());

    // day24
    let input = get_input(year, 24)?;
    println!("y{}-d24_p1: {}", year, day_24::part_1(&input).unwrap());
    println!("y{}-d24_p2: {}", year, day_24::part_2(&input).unwrap());

    // day25
    let input = get_input(year, 25)?;
    println!("y{}-d25_p1: {}", year, day_25::part_1(&input).unwrap());
    println!("y{}-d25_p2: {}", year, day_25::part_2(&input).unwrap());
*/
    Ok(())
}

#[allow(dead_code)]
fn run_2023() -> Result<(), AoCError<String>> {
    use year_2023::*;
    let year = 2023;

    // day 01
    let input = get_input(year, 1)?;
    println!("y{}-d01_p1: {}", year, day_01::part_1(&input).unwrap());
    println!("y{}-d01_p2: {}", year, day_01::part_2(&input).unwrap());

    // day02
    let input = get_input(year, 2)?;
    println!("y{}-d02_p1: {}", year, day_02::part_1(&input).unwrap());
    println!("y{}-d02_p2: {}", year, day_02::part_2(&input).unwrap());

    // day03
    let input = get_input(year, 3)?;
    println!("y{}-d03_p1: {}", year, day_03::part_1(&input).unwrap());
    println!("y{}-d03_p2: {}", year, day_03::part_2(&input).unwrap());

    // day04
    let input = get_input(year, 4)?;
    println!("y{}-d04_p1: {}", year, day_04::part_1(&input).unwrap());
    println!("y{}-d04_p2: {}", year, day_04::part_2(&input).unwrap());

    // day05
    let input = get_input(year, 5)?;
    println!("y{}-d05_p1: {}", year, day_05::part_1(&input).unwrap());
    println!("y{}-d05_p2: {}", year, day_05::part_2(&input).unwrap());

    // day06
    let input = get_input(year, 6)?;
    println!("y{}-d06_p1: {}", year, day_06::part_1(&input).unwrap());
    println!("y{}-d06_p2: {}", year, day_06::part_2(&input).unwrap());

    // day07
    let input = get_input(year, 7)?;
    println!("y{}-d07_p1: {}", year, day_07::part_1(&input).unwrap());
    println!("y{}-d07_p2: {}", year, day_07::part_2(&input).unwrap());

    // day08
    let input = get_input(year, 8)?;
    println!("y{}-d08_p1: {}", year, day_08::part_1(&input).unwrap());
    println!("y{}-d08_p2: {}", year, day_08::part_2(&input).unwrap());

    // day09
    let input = get_input(year, 9)?;
    println!("y{}-d09_p1: {}", year, day_09::part_1(&input).unwrap());
    println!("y{}-d09_p2: {}", year, day_09::part_2(&input).unwrap());

    // day10
    let input = get_input(year, 10)?;
    println!("y{}-d10_p1: {}", year, day_10::part_1(&input).unwrap());
    println!("y{}-d10_p2: {}", year, day_10::part_2(&input).unwrap());

    // day11
    let input = get_input(year, 11)?;
    println!("y{}-d11_p1: {}", year, day_11::part_1(&input).unwrap());
    println!("y{}-d11_p2: {}", year, day_11::part_2(&input).unwrap());

    // day12
    let input = get_input(year, 12)?;
    println!("y{}-d12_p1: {}", year, day_12::part_1(&input).unwrap());
    println!("y{}-d12_p2: {}", year, day_12::part_2(&input).unwrap());

    // day13
    let input = get_input(year, 13)?;
    println!("y{}-d13_p1: {}", year, day_13::part_1(&input).unwrap());
    println!("y{}-d13_p2: {}", year, day_13::part_2(&input).unwrap());

    // day14
    let input = get_input(year, 14)?;
    println!("y{}-d14_p1: {}", year, day_14::part_1(&input).unwrap());
    println!("y{}-d14_p2: {}", year, day_14::part_2(&input).unwrap());

    // day15
    let input = get_input(year, 15)?;
    println!("y{}-d15_p1: {}", year, day_15::part_1(&input).unwrap());
    println!("y{}-d15_p2: {}", year, day_15::part_2(&input).unwrap());

    // day16
    let input = get_input(year, 16)?;
    println!("y{}-d16_p1: {}", year, day_16::part_1(&input).unwrap());
    println!("y{}-d16_p2: {}", year, day_16::part_2(&input).unwrap());

    // day17
    let input = get_input(year, 17)?;
    println!("y{}-d17_p1: {}", year, day_17::part_1(&input).unwrap());
    println!("y{}-d17_p2: {}", year, day_17::part_2(&input).unwrap());

    // day18
    let input = get_input(year, 18)?;
    println!("y{}-d18_p1: {}", year, day_18::part_1(&input).unwrap());
    println!("y{}-d18_p2: {}", year, day_18::part_2(&input).unwrap());

    // day19
    let input = get_input(year, 19)?;
    println!("y{}-d19_p1: {}", year, day_19::part_1(&input).unwrap());
    println!("y{}-d19_p2: {}", year, day_19::part_2(&input).unwrap());

    // day20
    let input = get_input(year, 20)?;
    println!("y{}-d20_p1: {}", year, day_20::part_1(&input).unwrap());
    println!("y{}-d20_p2: {}", year, day_20::part_2(&input).unwrap());

    // day21
    let input = get_input(year, 21)?;
    println!("y{}-d21_p1: {}", year, day_21::part_1(&input).unwrap());
    println!("y{}-d21_p2: {}", year, day_21::part_2(&input).unwrap());

    // day22
    let input = get_input(year, 22)?;
    println!("y{}-d22_p1: {}", year, day_22::part_1(&input).unwrap());
    println!("y{}-d22_p2: {}", year, day_22::part_2(&input).unwrap());

    // day23
    let input = get_input(year, 23)?;
    println!("y{}-d23_p1: {}", year, day_23::part_1(&input).unwrap());
    println!("y{}-d23_p2: {}", year, day_23::part_2(&input).unwrap());

    // day24
    let input = get_input(year, 24)?;
    println!("y{}-d24_p1: {}", year, day_24::part_1(&input).unwrap());
    println!("y{}-d24_p2: {}", year, day_24::part_2(&input).unwrap());

    // day25
    let input = get_input(year, 25)?;
    println!("y{}-d25_p1: {}", year, day_25::part_1(&input).unwrap());
    println!("y{}-d25_p2: {}", year, day_25::part_2(&input).unwrap());

    Ok(())
}

fn run_2024() -> Result<(), AoCError<String>> {
    use year_2024::*;
    let year = 2024;

    // day 01
    let input = get_input(year, 1)?;
    println!("y{}-d01_p1: {}", year, day_01::part_1(&input).unwrap());
    println!("y{}-d01_p2: {}", year, day_01::part_2(&input).unwrap());
/*
    // day02
    let input = get_input(year, 2)?;
    println!("y{}-d02_p1: {}", year, day_02::part_1(&input).unwrap());
    println!("y{}-d02_p2: {}", year, day_02::part_2(&input).unwrap());

    // day03
    let input = get_input(year, 3)?;
    println!("y{}-d03_p1: {}", year, day_03::part_1(&input).unwrap());
    println!("y{}-d03_p2: {}", year, day_03::part_2(&input).unwrap());

    // day04
    let input = get_input(year, 4)?;
    println!("y{}-d04_p1: {}", year, day_04::part_1(&input).unwrap());
    println!("y{}-d04_p2: {}", year, day_04::part_2(&input).unwrap());

    // day05
    let input = get_input(year, 5)?;
    println!("y{}-d05_p1: {}", year, day_05::part_1(&input).unwrap());
    println!("y{}-d05_p2: {}", year, day_05::part_2(&input).unwrap());

    // day06
    let input = get_input(year, 6)?;
    println!("y{}-d06_p1: {}", year, day_06::part_1(&input).unwrap());
    println!("y{}-d06_p2: {}", year, day_06::part_2(&input).unwrap());

    // day07
    let input = get_input(year, 7)?;
    println!("y{}-d07_p1: {}", year, day_07::part_1(&input).unwrap());
    println!("y{}-d07_p2: {}", year, day_07::part_2(&input).unwrap());

    // day08
    let input = get_input(year, 8)?;
    println!("y{}-d08_p1: {}", year, day_08::part_1(&input).unwrap());
    println!("y{}-d08_p2: {}", year, day_08::part_2(&input).unwrap());

    // day09
    let input = get_input(year, 9)?;
    println!("y{}-d09_p1: {}", year, day_09::part_1(&input).unwrap());
    println!("y{}-d09_p2: {}", year, day_09::part_2(&input).unwrap());

    // day10
    let input = get_input(year, 10)?;
    println!("y{}-d10_p1: {}", year, day_10::part_1(&input).unwrap());
    println!("y{}-d10_p2: {}", year, day_10::part_2(&input).unwrap());

    // day11
    let input = get_input(year, 11)?;
    println!("y{}-d11_p1: {}", year, day_11::part_1(&input).unwrap());
    println!("y{}-d11_p2: {}", year, day_11::part_2(&input).unwrap());

    // day12
    let input = get_input(year, 12)?;
    println!("y{}-d12_p1: {}", year, day_12::part_1(&input).unwrap());
    println!("y{}-d12_p2: {}", year, day_12::part_2(&input).unwrap());

    // day13
    let input = get_input(year, 13)?;
    println!("y{}-d13_p1: {}", year, day_13::part_1(&input).unwrap());
    println!("y{}-d13_p2: {}", year, day_13::part_2(&input).unwrap());

    // day14
    let input = get_input(year, 14)?;
    println!("y{}-d14_p1: {}", year, day_14::part_1(&input).unwrap());
    println!("y{}-d14_p2: {}", year, day_14::part_2(&input).unwrap());

    // day15
    let input = get_input(year, 15)?;
    println!("y{}-d15_p1: {}", year, day_15::part_1(&input).unwrap());
    println!("y{}-d15_p2: {}", year, day_15::part_2(&input).unwrap());

    // day16
    let input = get_input(year, 16)?;
    println!("y{}-d16_p1: {}", year, day_16::part_1(&input).unwrap());
    println!("y{}-d16_p2: {}", year, day_16::part_2(&input).unwrap());

    // day17
    let input = get_input(year, 17)?;
    println!("y{}-d17_p1: {}", year, day_17::part_1(&input).unwrap());
    println!("y{}-d17_p2: {}", year, day_17::part_2(&input).unwrap());

    // day18
    let input = get_input(year, 18)?;
    println!("y{}-d18_p1: {}", year, day_18::part_1(&input).unwrap());
    println!("y{}-d18_p2: {}", year, day_18::part_2(&input).unwrap());

    // day19
    let input = get_input(year, 19)?;
    println!("y{}-d19_p1: {}", year, day_19::part_1(&input).unwrap());
    println!("y{}-d19_p2: {}", year, day_19::part_2(&input).unwrap());

    // day20
    let input = get_input(year, 20)?;
    println!("y{}-d20_p1: {}", year, day_20::part_1(&input).unwrap());
    println!("y{}-d20_p2: {}", year, day_20::part_2(&input).unwrap());

    // day21
    let input = get_input(year, 21)?;
    println!("y{}-d21_p1: {}", year, day_21::part_1(&input).unwrap());
    println!("y{}-d21_p2: {}", year, day_21::part_2(&input).unwrap());

    // day22
    let input = get_input(year, 22)?;
    println!("y{}-d22_p1: {}", year, day_22::part_1(&input).unwrap());
    println!("y{}-d22_p2: {}", year, day_22::part_2(&input).unwrap());

    // day23
    let input = get_input(year, 23)?;
    println!("y{}-d23_p1: {}", year, day_23::part_1(&input).unwrap());
    println!("y{}-d23_p2: {}", year, day_23::part_2(&input).unwrap());

    // day24
    let input = get_input(year, 24)?;
    println!("y{}-d24_p1: {}", year, day_24::part_1(&input).unwrap());
    println!("y{}-d24_p2: {}", year, day_24::part_2(&input).unwrap());

    // day25
    let input = get_input(year, 25)?;
    println!("y{}-d25_p1: {}", year, day_25::part_1(&input).unwrap());
    println!("y{}-d25_p2: {}", year, day_25::part_2(&input).unwrap());
*/
    Ok(())
}

/*fn run_20XX() -> Result<(), AoCError<String>> {   // TODO add year
    use year_20XX::*;   // TODO add year
    let year = 20XX;  // TODO add year

    // day 01
    let input = get_input(year, 1)?;
    println!("y{}-d01_p1: {}", year, day_01::part_1(&input).unwrap());
    println!("y{}-d01_p2: {}", year, day_01::part_2(&input).unwrap());

    // day02
    let input = get_input(year, 2)?;
    println!("y{}-d02_p1: {}", year, day_02::part_1(&input).unwrap());
    println!("y{}-d02_p2: {}", year, day_02::part_2(&input).unwrap());

    // day03
    let input = get_input(year, 3)?;
    println!("y{}-d03_p1: {}", year, day_03::part_1(&input).unwrap());
    println!("y{}-d03_p2: {}", year, day_03::part_2(&input).unwrap());

    // day04
    let input = get_input(year, 4)?;
    println!("y{}-d04_p1: {}", year, day_04::part_1(&input).unwrap());
    println!("y{}-d04_p2: {}", year, day_04::part_2(&input).unwrap());

    // day05
    let input = get_input(year, 5)?;
    println!("y{}-d05_p1: {}", year, day_05::part_1(&input).unwrap());
    println!("y{}-d05_p2: {}", year, day_05::part_2(&input).unwrap());

    // day06
    let input = get_input(year, 6)?;
    println!("y{}-d06_p1: {}", year, day_06::part_1(&input).unwrap());
    println!("y{}-d06_p2: {}", year, day_06::part_2(&input).unwrap());

    // day07
    let input = get_input(year, 7)?;
    println!("y{}-d07_p1: {}", year, day_07::part_1(&input).unwrap());
    println!("y{}-d07_p2: {}", year, day_07::part_2(&input).unwrap());

    // day08
    let input = get_input(year, 8)?;
    println!("y{}-d08_p1: {}", year, day_08::part_1(&input).unwrap());
    println!("y{}-d08_p2: {}", year, day_08::part_2(&input).unwrap());

    // day09
    let input = get_input(year, 9)?;
    println!("y{}-d09_p1: {}", year, day_09::part_1(&input).unwrap());
    println!("y{}-d09_p2: {}", year, day_09::part_2(&input).unwrap());

    // day10
    let input = get_input(year, 10)?;
    println!("y{}-d10_p1: {}", year, day_10::part_1(&input).unwrap());
    println!("y{}-d10_p2: {}", year, day_10::part_2(&input).unwrap());

    // day11
    let input = get_input(year, 11)?;
    println!("y{}-d11_p1: {}", year, day_11::part_1(&input).unwrap());
    println!("y{}-d11_p2: {}", year, day_11::part_2(&input).unwrap());

    // day12
    let input = get_input(year, 12)?;
    println!("y{}-d12_p1: {}", year, day_12::part_1(&input).unwrap());
    println!("y{}-d12_p2: {}", year, day_12::part_2(&input).unwrap());

    // day13
    let input = get_input(year, 13)?;
    println!("y{}-d13_p1: {}", year, day_13::part_1(&input).unwrap());
    println!("y{}-d13_p2: {}", year, day_13::part_2(&input).unwrap());

    // day14
    let input = get_input(year, 14)?;
    println!("y{}-d14_p1: {}", year, day_14::part_1(&input).unwrap());
    println!("y{}-d14_p2: {}", year, day_14::part_2(&input).unwrap());

    // day15
    let input = get_input(year, 15)?;
    println!("y{}-d15_p1: {}", year, day_15::part_1(&input).unwrap());
    println!("y{}-d15_p2: {}", year, day_15::part_2(&input).unwrap());

    // day16
    let input = get_input(year, 16)?;
    println!("y{}-d16_p1: {}", year, day_16::part_1(&input).unwrap());
    println!("y{}-d16_p2: {}", year, day_16::part_2(&input).unwrap());

    // day17
    let input = get_input(year, 17)?;
    println!("y{}-d17_p1: {}", year, day_17::part_1(&input).unwrap());
    println!("y{}-d17_p2: {}", year, day_17::part_2(&input).unwrap());

    // day18
    let input = get_input(year, 18)?;
    println!("y{}-d18_p1: {}", year, day_18::part_1(&input).unwrap());
    println!("y{}-d18_p2: {}", year, day_18::part_2(&input).unwrap());

    // day19
    let input = get_input(year, 19)?;
    println!("y{}-d19_p1: {}", year, day_19::part_1(&input).unwrap());
    println!("y{}-d19_p2: {}", year, day_19::part_2(&input).unwrap());

    // day20
    let input = get_input(year, 20)?;
    println!("y{}-d20_p1: {}", year, day_20::part_1(&input).unwrap());
    println!("y{}-d20_p2: {}", year, day_20::part_2(&input).unwrap());

    // day21
    let input = get_input(year, 21)?;
    println!("y{}-d21_p1: {}", year, day_21::part_1(&input).unwrap());
    println!("y{}-d21_p2: {}", year, day_21::part_2(&input).unwrap());

    // day22
    let input = get_input(year, 22)?;
    println!("y{}-d22_p1: {}", year, day_22::part_1(&input).unwrap());
    println!("y{}-d22_p2: {}", year, day_22::part_2(&input).unwrap());

    // day23
    let input = get_input(year, 23)?;
    println!("y{}-d23_p1: {}", year, day_23::part_1(&input).unwrap());
    println!("y{}-d23_p2: {}", year, day_23::part_2(&input).unwrap());

    // day24
    let input = get_input(year, 24)?;
    println!("y{}-d24_p1: {}", year, day_24::part_1(&input).unwrap());
    println!("y{}-d24_p2: {}", year, day_24::part_2(&input).unwrap());

    // day25
    let input = get_input(year, 25)?;
    println!("y{}-d25_p1: {}", year, day_25::part_1(&input).unwrap());
    println!("y{}-d25_p2: {}", year, day_25::part_2(&input).unwrap());

    Ok(())
}*/

fn main() -> Result<(), AoCError<String>> {
    // run_2015()?;
    // run_2016()?;
    // run_2017()?;
    // run_2023()?;
    run_2024()?;

    Ok(())
}

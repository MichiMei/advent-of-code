use std::io;
use advent_of_code::{read_lines_untrimmed_from_file, year_2015, year_2016, year_2017};


#[allow(dead_code)]
fn run_2015() -> io::Result<()> {
    use year_2015::*;
    let year = "2015";
    let input_folder = format!("input/year_{}/", year);

    // day 01
    let input_name = input_folder.to_string() + "/input_day_01.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d01_p1: {}", year, day_01::part_1(&input).unwrap());
    println!("y{}-d01_p2: {}", year, day_01::part_2(&input).unwrap());

    // day02
    let input_name = input_folder.to_string() + "/input_day_02.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d02_p1: {}", year, day_02::part_1(&input).unwrap());
    println!("y{}-d02_p2: {}", year, day_02::part_2(&input).unwrap());

    // day03
    let input_name = input_folder.to_string() + "/input_day_03.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d03_p1: {}", year, day_03::part_1(&input).unwrap());
    println!("y{}-d03_p2: {}", year, day_03::part_2(&input).unwrap());

    // day04
    let input_name = input_folder.to_string() + "/input_day_04.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d04_p1: {}", year, day_04::part_1(&input).unwrap());
    println!("y{}-d04_p2: {}", year, day_04::part_2(&input).unwrap());

    // day05
    let input_name = input_folder.to_string() + "/input_day_05.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d05_p1: {}", year, day_05::part_1(&input).unwrap());
    println!("y{}-d05_p2: {}", year, day_05::part_2(&input).unwrap());

    // day06
    let input_name = input_folder.to_string() + "/input_day_06.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d06_p1: {}", year, day_06::part_1(&input).unwrap());
    println!("y{}-d06_p2: {}", year, day_06::part_2(&input).unwrap());

    // day07
    let input_name = input_folder.to_string() + "/input_day_07.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d07_p1: {}", year, day_07::part_1(&input).unwrap());
    println!("y{}-d07_p2: {}", year, day_07::part_2(&input).unwrap());

    // day08
    let input_name = input_folder.to_string() + "/input_day_08.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d08_p1: {}", year, day_08::part_1(&input).unwrap());
    println!("y{}-d08_p2: {}", year, day_08::part_2(&input).unwrap());

    // day09
    let input_name = input_folder.to_string() + "/input_day_09.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d09_p1: {}", year, day_09::part_1(&input).unwrap());
    println!("y{}-d09_p2: {}", year, day_09::part_2(&input).unwrap());

    // day10
    let input_name = input_folder.to_string() + "/input_day_10.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d10_p1: {}", year, day_10::part_1(&input).unwrap());
    println!("y{}-d10_p2: {}", year, day_10::part_2(&input).unwrap());

    // day11
    let input_name = input_folder.to_string() + "/input_day_11.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d11_p1: {}", year, day_11::part_1(&input).unwrap());
    println!("y{}-d11_p2: {}", year, day_11::part_2(&input).unwrap());

    // day12
    let input_name = input_folder.to_string() + "/input_day_12.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d12_p1: {}", year, day_12::part_1(&input).unwrap());
    println!("y{}-d12_p2: {}", year, day_12::part_2(&input).unwrap());

    // day13
    let input_name = input_folder.to_string() + "/input_day_13.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d13_p1: {}", year, day_13::part_1(&input).unwrap());
    println!("y{}-d13_p2: {}", year, day_13::part_2(&input).unwrap());

    // day14
    let input_name = input_folder.to_string() + "/input_day_14.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d14_p1: {}", year, day_14::part_1(&input).unwrap());
    println!("y{}-d14_p2: {}", year, day_14::part_2(&input).unwrap());

    // day15
    let input_name = input_folder.to_string() + "/input_day_15.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d15_p1: {}", year, day_15::part_1(&input).unwrap());
    println!("y{}-d15_p2: {}", year, day_15::part_2(&input).unwrap());

    // day16
    let input_name = input_folder.to_string() + "/input_day_16.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d16_p1: {}", year, day_16::part_1(&input).unwrap());
    println!("y{}-d16_p2: {}", year, day_16::part_2(&input).unwrap());

    // day17
    let input_name = input_folder.to_string() + "/input_day_17.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d17_p1: {}", year, day_17::part_1(&input).unwrap());
    println!("y{}-d17_p2: {}", year, day_17::part_2(&input).unwrap());

    // day18
    let input_name = input_folder.to_string() + "/input_day_18.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d18_p1: {}", year, day_18::part_1(&input).unwrap());
    println!("y{}-d18_p2: {}", year, day_18::part_2(&input).unwrap());

    // day19
    let input_name = input_folder.to_string() + "/input_day_19.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d19_p1: {}", year, day_19::part_1(&input).unwrap());
    println!("y{}-d19_p2: {}", year, day_19::part_2(&input).unwrap());

    // day20
    let input_name = input_folder.to_string() + "/input_day_20.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d20_p1: {}", year, day_20::part_1(&input).unwrap());
    println!("y{}-d20_p2: {}", year, day_20::part_2(&input).unwrap());

    // day21
    let input_name = input_folder.to_string() + "/input_day_21.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d21_p1: {}", year, day_21::part_1(&input).unwrap());
    println!("y{}-d21_p2: {}", year, day_21::part_2(&input).unwrap());

    // day22
    let input_name = input_folder.to_string() + "/input_day_22.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d22_p1: {}", year, day_22::part_1(&input).unwrap());
    println!("y{}-d22_p2: {}", year, day_22::part_2(&input).unwrap());

    // day23
    let input_name = input_folder.to_string() + "/input_day_23.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d23_p1: {}", year, day_23::part_1(&input).unwrap());
    println!("y{}-d23_p2: {}", year, day_23::part_2(&input).unwrap());

    // day24
    let input_name = input_folder.to_string() + "/input_day_24.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d24_p1: {}", year, day_24::part_1(&input).unwrap());
    println!("y{}-d24_p2: {}", year, day_24::part_2(&input).unwrap());

    // day25
    let input_name = input_folder + "/input_day_25.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d25_p1: {}", year, day_25::part_1(&input).unwrap());
    println!("y{}-d25_p2: {}", year, day_25::part_2(&input).unwrap());

    Ok(())
}

fn run_2016() -> io::Result<()> {
    use year_2016::*;
    let year = "2016";
    let input_folder = format!("input/year_{}/", year);

    // day 01
    let input_name = input_folder.to_string() + "/input_day_01.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d01_p1: {}", year, day_01::part_1(&input).unwrap());
    println!("y{}-d01_p2: {}", year, day_01::part_2(&input).unwrap());
    
    // day02
    let input_name = input_folder.to_string() + "/input_day_02.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d02_p1: {}", year, day_02::part_1(&input).unwrap());
    println!("y{}-d02_p2: {}", year, day_02::part_2(&input).unwrap());

    // day03
    let input_name = input_folder.to_string() + "/input_day_03.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d03_p1: {}", year, day_03::part_1(&input).unwrap());
    println!("y{}-d03_p2: {}", year, day_03::part_2(&input).unwrap());

    // day04
    let input_name = input_folder.to_string() + "/input_day_04.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d04_p1: {}", year, day_04::part_1(&input).unwrap());
    println!("y{}-d04_p2: {}", year, day_04::part_2(&input).unwrap());

    // day05
    let input_name = input_folder.to_string() + "/input_day_05.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d05_p1: {}", year, day_05::part_1(&input).unwrap());
    println!("y{}-d05_p2: {}", year, day_05::part_2(&input).unwrap());

    // day06
    let input_name = input_folder.to_string() + "/input_day_06.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d06_p1: {}", year, day_06::part_1(&input).unwrap());
    println!("y{}-d06_p2: {}", year, day_06::part_2(&input).unwrap());

    // day07
    let input_name = input_folder.to_string() + "/input_day_07.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d07_p1: {}", year, day_07::part_1(&input).unwrap());
    println!("y{}-d07_p2: {}", year, day_07::part_2(&input).unwrap());
    
    // day08
    let input_name = input_folder.to_string() + "/input_day_08.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d08_p1: {}", year, day_08::part_1(&input).unwrap());
    println!("y{}-d08_p2: {}", year, day_08::part_2(&input).unwrap());

    // day09
    let input_name = input_folder.to_string() + "/input_day_09.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d09_p1: {}", year, day_09::part_1(&input).unwrap());
    println!("y{}-d09_p2: {}", year, day_09::part_2(&input).unwrap());

    // day10
    let input_name = input_folder.to_string() + "/input_day_10.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d10_p1: {}", year, day_10::part_1(&input).unwrap());
    println!("y{}-d10_p2: {}", year, day_10::part_2(&input).unwrap());

    // day11
    let input_name = input_folder.to_string() + "/input_day_11.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d11_p1: {}", year, day_11::part_1(&input).unwrap());
    println!("y{}-d11_p2: {}", year, day_11::part_2(&input).unwrap());

    // day12
    let input_name = input_folder.to_string() + "/input_day_12.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d12_p1: {}", year, day_12::part_1(&input).unwrap());
    println!("y{}-d12_p2: {}", year, day_12::part_2(&input).unwrap());

    // day13
    let input_name = input_folder.to_string() + "/input_day_13.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d13_p1: {}", year, day_13::part_1(&input).unwrap());
    println!("y{}-d13_p2: {}", year, day_13::part_2(&input).unwrap());

    // day14
    let input_name = input_folder.to_string() + "/input_day_14.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d14_p1: {}", year, day_14::part_1(&input).unwrap());
    println!("y{}-d14_p2: {}", year, day_14::part_2(&input).unwrap());

    // day15
    let input_name = input_folder.to_string() + "/input_day_15.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d15_p1: {}", year, day_15::part_1(&input).unwrap());
    println!("y{}-d15_p2: {}", year, day_15::part_2(&input).unwrap());

    // day16
    let input_name = input_folder.to_string() + "/input_day_16.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d16_p1: {}", year, day_16::part_1(&input).unwrap());
    println!("y{}-d16_p2: {}", year, day_16::part_2(&input).unwrap());

    // day17
    let input_name = input_folder.to_string() + "/input_day_17.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d17_p1: {}", year, day_17::part_1(&input).unwrap());
    println!("y{}-d17_p2: {}", year, day_17::part_2(&input).unwrap());

    // day18
    let input_name = input_folder.to_string() + "/input_day_18.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d18_p1: {}", year, day_18::part_1(&input).unwrap());
    println!("y{}-d18_p2: {}", year, day_18::part_2(&input).unwrap());

    // day19
    let input_name = input_folder.to_string() + "/input_day_19.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d19_p1: {}", year, day_19::part_1(&input).unwrap());
    println!("y{}-d19_p2: {}", year, day_19::part_2(&input).unwrap());

    // day20
    let input_name = input_folder.to_string() + "/input_day_20.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d20_p1: {}", year, day_20::part_1(&input).unwrap());
    println!("y{}-d20_p2: {}", year, day_20::part_2(&input).unwrap());

    // day21
    let input_name = input_folder.to_string() + "/input_day_21.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d21_p1: {}", year, day_21::part_1(&input).unwrap());
    println!("y{}-d21_p2: {}", year, day_21::part_2(&input).unwrap());

    // day22
    let input_name = input_folder.to_string() + "/input_day_22.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d22_p1: {}", year, day_22::part_1(&input).unwrap());
    println!("y{}-d22_p2: {}", year, day_22::part_2(&input).unwrap());

    // day23
    let input_name = input_folder.to_string() + "/input_day_23.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d23_p1: {}", year, day_23::part_1(&input).unwrap());
    println!("y{}-d23_p2: {}", year, day_23::part_2(&input).unwrap());

    // day24
    let input_name = input_folder.to_string() + "/input_day_24.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d24_p1: {}", year, day_24::part_1(&input).unwrap());
    println!("y{}-d24_p2: {}", year, day_24::part_2(&input).unwrap());

    // day25
    let input_name = input_folder + "/input_day_25.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d25_p1: {}", year, day_25::part_1(&input).unwrap());
    println!("y{}-d25_p2: {}", year, day_25::part_2(&input).unwrap());

    Ok(())
}

fn run_2017() -> io::Result<()> {
    use year_2017::*;
    let year = "2017";
    let input_folder = format!("input/year_{}/", year);

    // day 01
    let input_name = input_folder.to_string() + "/input_day_01.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d01_p1: {}", year, day_01::part_1(&input).unwrap());
    println!("y{}-d01_p2: {}", year, day_01::part_2(&input).unwrap());

    // day02
    let input_name = input_folder.to_string() + "/input_day_02.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d02_p1: {}", year, day_02::part_1(&input).unwrap());
    println!("y{}-d02_p2: {}", year, day_02::part_2(&input).unwrap());

    // day03
    let input_name = input_folder.to_string() + "/input_day_03.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d03_p1: {}", year, day_03::part_1(&input).unwrap());
    println!("y{}-d03_p2: {}", year, day_03::part_2(&input).unwrap());

    // day04
    let input_name = input_folder.to_string() + "/input_day_04.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d04_p1: {}", year, day_04::part_1(&input).unwrap());
    println!("y{}-d04_p2: {}", year, day_04::part_2(&input).unwrap());

    // day05
    let input_name = input_folder.to_string() + "/input_day_05.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d05_p1: {}", year, day_05::part_1(&input).unwrap());
    println!("y{}-d05_p2: {}", year, day_05::part_2(&input).unwrap());

    // day06
    let input_name = input_folder.to_string() + "/input_day_06.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d06_p1: {}", year, day_06::part_1(&input).unwrap());
    println!("y{}-d06_p2: {}", year, day_06::part_2(&input).unwrap());

    // day07
    let input_name = input_folder.to_string() + "/input_day_07.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d07_p1: {}", year, day_07::part_1(&input).unwrap());
    println!("y{}-d07_p2: {}", year, day_07::part_2(&input).unwrap());

    // day08
    let input_name = input_folder.to_string() + "/input_day_08.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d08_p1: {}", year, day_08::part_1(&input).unwrap());
    println!("y{}-d08_p2: {}", year, day_08::part_2(&input).unwrap());

    // day09
    let input_name = input_folder.to_string() + "/input_day_09.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d09_p1: {}", year, day_09::part_1(&input).unwrap());
    println!("y{}-d09_p2: {}", year, day_09::part_2(&input).unwrap());

    // day10
    let input_name = input_folder.to_string() + "/input_day_10.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d10_p1: {}", year, day_10::part_1(&input).unwrap());
    println!("y{}-d10_p2: {}", year, day_10::part_2(&input).unwrap());

    // day11
    let input_name = input_folder.to_string() + "/input_day_11.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d11_p1: {}", year, day_11::part_1(&input).unwrap());
    println!("y{}-d11_p2: {}", year, day_11::part_2(&input).unwrap());

    // day12
    let input_name = input_folder.to_string() + "/input_day_12.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d12_p1: {}", year, day_12::part_1(&input).unwrap());
    println!("y{}-d12_p2: {}", year, day_12::part_2(&input).unwrap());

    // day13
    let input_name = input_folder.to_string() + "/input_day_13.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d13_p1: {}", year, day_13::part_1(&input).unwrap());
    println!("y{}-d13_p2: {}", year, day_13::part_2(&input).unwrap());

    // day14
    let input_name = input_folder.to_string() + "/input_day_14.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d14_p1: {}", year, day_14::part_1(&input).unwrap());
    println!("y{}-d14_p2: {}", year, day_14::part_2(&input).unwrap());

    // day15
    let input_name = input_folder.to_string() + "/input_day_15.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d15_p1: {}", year, day_15::part_1(&input).unwrap());
    println!("y{}-d15_p2: {}", year, day_15::part_2(&input).unwrap());
/*
    // day16
    let input_name = input_folder.to_string() + "/input_day_16.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d16_p1: {}", year, day_16::part_1(&input).unwrap());
    println!("y{}-d16_p2: {}", year, day_16::part_2(&input).unwrap());

    // day17
    let input_name = input_folder.to_string() + "/input_day_17.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d17_p1: {}", year, day_17::part_1(&input).unwrap());
    println!("y{}-d17_p2: {}", year, day_17::part_2(&input).unwrap());

    // day18
    let input_name = input_folder.to_string() + "/input_day_18.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d18_p1: {}", year, day_18::part_1(&input).unwrap());
    println!("y{}-d18_p2: {}", year, day_18::part_2(&input).unwrap());

    // day19
    let input_name = input_folder.to_string() + "/input_day_19.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d19_p1: {}", year, day_19::part_1(&input).unwrap());
    println!("y{}-d19_p2: {}", year, day_19::part_2(&input).unwrap());

    // day20
    let input_name = input_folder.to_string() + "/input_day_20.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d20_p1: {}", year, day_20::part_1(&input).unwrap());
    println!("y{}-d20_p2: {}", year, day_20::part_2(&input).unwrap());

    // day21
    let input_name = input_folder.to_string() + "/input_day_21.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d21_p1: {}", year, day_21::part_1(&input).unwrap());
    println!("y{}-d21_p2: {}", year, day_21::part_2(&input).unwrap());

    // day22
    let input_name = input_folder.to_string() + "/input_day_22.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d22_p1: {}", year, day_22::part_1(&input).unwrap());
    println!("y{}-d22_p2: {}", year, day_22::part_2(&input).unwrap());

    // day23
    let input_name = input_folder.to_string() + "/input_day_23.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d23_p1: {}", year, day_23::part_1(&input).unwrap());
    println!("y{}-d23_p2: {}", year, day_23::part_2(&input).unwrap());

    // day24
    let input_name = input_folder.to_string() + "/input_day_24.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d24_p1: {}", year, day_24::part_1(&input).unwrap());
    println!("y{}-d24_p2: {}", year, day_24::part_2(&input).unwrap());

    // day25
    let input_name = input_folder.to_string() + "/input_day_25.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d25_p1: {}", year, day_25::part_1(&input).unwrap());
    println!("y{}-d25_p2: {}", year, day_25::part_2(&input).unwrap());
*/
    Ok(())
}

/*fn run_20XX() -> io::Result<()> {   // TODO add year
    use year_20XX::*;   // TODO add year
    let year = "20XX";  // TODO add year
    let input_folder = format!("input/year_{}/", year);

    // day 01
    let input_name = input_folder.to_string() + "/input_day_01.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d01_p1: {}", year, day_01::part_1(&input).unwrap());
    println!("y{}-d01_p2: {}", year, day_01::part_2(&input).unwrap());

    // day02
    let input_name = input_folder.to_string() + "/input_day_02.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d02_p1: {}", year, day_02::part_1(&input).unwrap());
    println!("y{}-d02_p2: {}", year, day_02::part_2(&input).unwrap());

    // day03
    let input_name = input_folder.to_string() + "/input_day_03.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d03_p1: {}", year, day_03::part_1(&input).unwrap());
    println!("y{}-d03_p2: {}", year, day_03::part_2(&input).unwrap());

    // day04
    let input_name = input_folder.to_string() + "/input_day_04.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d04_p1: {}", year, day_04::part_1(&input).unwrap());
    println!("y{}-d04_p2: {}", year, day_04::part_2(&input).unwrap());

    // day05
    let input_name = input_folder.to_string() + "/input_day_05.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d05_p1: {}", year, day_05::part_1(&input).unwrap());
    println!("y{}-d05_p2: {}", year, day_05::part_2(&input).unwrap());

    // day06
    let input_name = input_folder.to_string() + "/input_day_06.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d06_p1: {}", year, day_06::part_1(&input).unwrap());
    println!("y{}-d06_p2: {}", year, day_06::part_2(&input).unwrap());

    // day07
    let input_name = input_folder.to_string() + "/input_day_07.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d07_p1: {}", year, day_07::part_1(&input).unwrap());
    println!("y{}-d07_p2: {}", year, day_07::part_2(&input).unwrap());

    // day08
    let input_name = input_folder.to_string() + "/input_day_08.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d08_p1: {}", year, day_08::part_1(&input).unwrap());
    println!("y{}-d08_p2: {}", year, day_08::part_2(&input).unwrap());

    // day09
    let input_name = input_folder.to_string() + "/input_day_09.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d09_p1: {}", year, day_09::part_1(&input).unwrap());
    println!("y{}-d09_p2: {}", year, day_09::part_2(&input).unwrap());

    // day10
    let input_name = input_folder.to_string() + "/input_day_10.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d10_p1: {}", year, day_10::part_1(&input).unwrap());
    println!("y{}-d10_p2: {}", year, day_10::part_2(&input).unwrap());

    // day11
    let input_name = input_folder.to_string() + "/input_day_11.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d11_p1: {}", year, day_11::part_1(&input).unwrap());
    println!("y{}-d11_p2: {}", year, day_11::part_2(&input).unwrap());

    // day12
    let input_name = input_folder.to_string() + "/input_day_12.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d12_p1: {}", year, day_12::part_1(&input).unwrap());
    println!("y{}-d12_p2: {}", year, day_12::part_2(&input).unwrap());

    // day13
    let input_name = input_folder.to_string() + "/input_day_13.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d13_p1: {}", year, day_13::part_1(&input).unwrap());
    println!("y{}-d13_p2: {}", year, day_13::part_2(&input).unwrap());

    // day14
    let input_name = input_folder.to_string() + "/input_day_14.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d14_p1: {}", year, day_14::part_1(&input).unwrap());
    println!("y{}-d14_p2: {}", year, day_14::part_2(&input).unwrap());

    // day15
    let input_name = input_folder.to_string() + "/input_day_15.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d15_p1: {}", year, day_15::part_1(&input).unwrap());
    println!("y{}-d15_p2: {}", year, day_15::part_2(&input).unwrap());

    // day16
    let input_name = input_folder.to_string() + "/input_day_16.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d16_p1: {}", year, day_16::part_1(&input).unwrap());
    println!("y{}-d16_p2: {}", year, day_16::part_2(&input).unwrap());

    // day17
    let input_name = input_folder.to_string() + "/input_day_17.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d17_p1: {}", year, day_17::part_1(&input).unwrap());
    println!("y{}-d17_p2: {}", year, day_17::part_2(&input).unwrap());

    // day18
    let input_name = input_folder.to_string() + "/input_day_18.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d18_p1: {}", year, day_18::part_1(&input).unwrap());
    println!("y{}-d18_p2: {}", year, day_18::part_2(&input).unwrap());

    // day19
    let input_name = input_folder.to_string() + "/input_day_19.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d19_p1: {}", year, day_19::part_1(&input).unwrap());
    println!("y{}-d19_p2: {}", year, day_19::part_2(&input).unwrap());

    // day20
    let input_name = input_folder.to_string() + "/input_day_20.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d20_p1: {}", year, day_20::part_1(&input).unwrap());
    println!("y{}-d20_p2: {}", year, day_20::part_2(&input).unwrap());

    // day21
    let input_name = input_folder.to_string() + "/input_day_21.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d21_p1: {}", year, day_21::part_1(&input).unwrap());
    println!("y{}-d21_p2: {}", year, day_21::part_2(&input).unwrap());

    // day22
    let input_name = input_folder.to_string() + "/input_day_22.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d22_p1: {}", year, day_22::part_1(&input).unwrap());
    println!("y{}-d22_p2: {}", year, day_22::part_2(&input).unwrap());

    // day23
    let input_name = input_folder.to_string() + "/input_day_23.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d23_p1: {}", year, day_23::part_1(&input).unwrap());
    println!("y{}-d23_p2: {}", year, day_23::part_2(&input).unwrap());

    // day24
    let input_name = input_folder.to_string() + "/input_day_24.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d24_p1: {}", year, day_24::part_1(&input).unwrap());
    println!("y{}-d24_p2: {}", year, day_24::part_2(&input).unwrap());

    // day25
    let input_name = input_folder.to_string() + "/input_day_25.txt";
    let input = read_lines_untrimmed_from_file(&input_name)?;
    println!("y{}-d25_p1: {}", year, day_25::part_1(&input).unwrap());
    println!("y{}-d25_p2: {}", year, day_25::part_2(&input).unwrap());

    Ok(())
}*/

fn main() -> io::Result<()> {
    //run_2015()
    // run_2016()
    run_2017()
}

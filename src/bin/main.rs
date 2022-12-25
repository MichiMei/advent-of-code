use std::io;
use advent_of_code::{read_lines_untrimmed_from_file, year_2015};



fn main() -> io::Result<()> {
    run_2015()
}

fn run_2015() -> io::Result<()> {
    // day 01
    let input_name = "input/year_2015/input_day_01.txt";
    let input = read_lines_untrimmed_from_file(input_name)?;
    println!("y2015-d01_p1: {}", year_2015::day_01::part_1(&input).unwrap());
    println!("y2015-d01_p2: {}", year_2015::day_01::part_2(&input).unwrap());

    // day02
    let input_name = "input/year_2015/input_day_02.txt";
    let input = read_lines_untrimmed_from_file(input_name)?;
    println!("y2015-d02_p1: {}", year_2015::day_02::part_1(&input).unwrap());
    println!("y2015-d02_p2: {}", year_2015::day_02::part_2(&input).unwrap());

    // day03
    let input_name = "input/year_2015/input_day_03.txt";
    let input = read_lines_untrimmed_from_file(input_name)?;
    println!("y2015-d03_p1: {}", year_2015::day_03::part_1(&input).unwrap());
    println!("y2015-d03_p2: {}", year_2015::day_03::part_2(&input).unwrap());

    /*
    // dayXX    // TODO
    let input_name = "input/year_2015/input_day_XX.txt";    // TODO
    let input = read_lines_untrimmed_from_file(input_name)?;
    println!("y2015-d03_p1: {}", year_2015::day_XX::part_1(&input).unwrap());   // TODO
    println!("y2015-d03_p2: {}", year_2015::day_XX::part_2(&input).unwrap());   // TODO
     */

    Ok(())
}
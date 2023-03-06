use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code::read_lines_untrimmed_from_file;
use advent_of_code::year_2016::*;

const INPUT_FOLDER: &str = "input/year_2016/";

fn day_01_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_01.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 01 part 1", |b| b.iter(|| day_01::part_1(&input)));
}

fn day_01_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_01.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 01 part 2", |b| b.iter(|| day_01::part_2(&input)));
}

fn day_02_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_02.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 02 part 1", |b| b.iter(|| day_02::part_1(&input)));
}

fn day_02_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_02.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 02 part 2", |b| b.iter(|| day_02::part_2(&input)));
}

fn day_03_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_03.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 03 part 1", |b| b.iter(|| day_03::part_1(&input)));
}

fn day_03_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_03.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 03 part 2", |b| b.iter(|| day_03::part_2(&input)));
}

fn day_04_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_04.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 04 part 1", |b| b.iter(|| day_04::part_1(&input)));
}

fn day_04_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_04.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 04 part 2", |b| b.iter(|| day_04::part_2(&input)));
}

fn day_05_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_05.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 05 part 1", |b| b.iter(|| day_05::part_1(&input)));
}

fn day_05_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_05.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 05 part 2", |b| b.iter(|| day_05::part_2(&input)));
}

fn day_06_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_06.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 06 part 1", |b| b.iter(|| day_06::part_1(&input)));
}

fn day_06_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_06.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 06 part 2", |b| b.iter(|| day_06::part_2(&input)));
}

fn day_07_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_07.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 07 part 1", |b| b.iter(|| day_07::part_1(&input)));
}

fn day_07_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_XX.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 07 part 2", |b| b.iter(|| day_07::part_2(&input)));
}

fn day_08_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_08.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 08 part 1", |b| b.iter(|| day_08::part_1(&input)));
}

fn day_08_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_08.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 08 part 2", |b| b.iter(|| day_08::part_2(&input)));
}

fn day_09_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_09.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 09 part 1", |b| b.iter(|| day_09::part_1(&input)));
}

fn day_09_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_09.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 09 part 2", |b| b.iter(|| day_09::part_2(&input)));
}

fn day_10_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_10.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 10 part 1", |b| b.iter(|| day_10::part_1(&input)));
}

fn day_10_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_10.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 10 part 2", |b| b.iter(|| day_10::part_2(&input)));
}

fn day_11_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_11.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 11 part 1", |b| b.iter(|| day_11::part_1(&input)));
}

fn day_11_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_11.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 11 part 2", |b| b.iter(|| day_11::part_2(&input)));
}

fn day_12_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_12.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 12 part 1", |b| b.iter(|| day_12::part_1(&input)));
}

fn day_12_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_12.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 12 part 2", |b| b.iter(|| day_12::part_2(&input)));
}

fn day_13_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_13.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 13 part 1", |b| b.iter(|| day_13::part_1(&input)));
}

fn day_13_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_13.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 13 part 2", |b| b.iter(|| day_13::part_2(&input)));
}

fn day_14_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_14.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 14 part 1", |b| b.iter(|| day_14::part_1(&input)));
}

fn day_14_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_14.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 14 part 2", |b| b.iter(|| day_14::part_2(&input)));
}

fn day_15_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_15.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 15 part 1", |b| b.iter(|| day_15::part_1(&input)));
}

fn day_15_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_15.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 15 part 2", |b| b.iter(|| day_15::part_2(&input)));
}

fn day_16_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_16.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 16 part 1", |b| b.iter(|| day_16::part_1(&input)));
}

fn day_16_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_16.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 16 part 2", |b| b.iter(|| day_16::part_2(&input)));
}

fn day_17_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_17.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 17 part 1", |b| b.iter(|| day_17::part_1(&input)));
}

fn day_17_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_17.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 17 part 2", |b| b.iter(|| day_17::part_2(&input)));
}

fn day_18_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_18.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 18 part 1", |b| b.iter(|| day_18::part_1(&input)));
}

fn day_18_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_18.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 18 part 2", |b| b.iter(|| day_18::part_2(&input)));
}

fn day_19_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_19.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 19 part 1", |b| b.iter(|| day_19::part_1(&input)));
}

fn day_19_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_19.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 19 part 2", |b| b.iter(|| day_19::part_2(&input)));
}

fn day_20_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_20.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 20 part 1", |b| b.iter(|| day_20::part_1(&input)));
}

fn day_20_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_20.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 20 part 2", |b| b.iter(|| day_20::part_2(&input)));
}

fn day_21_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_21.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 21 part 1", |b| b.iter(|| day_21::part_1(&input)));
}

fn day_21_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_21.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 21 part 2", |b| b.iter(|| day_21::part_2(&input)));
}

fn day_22_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_22.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 22 part 1", |b| b.iter(|| day_22::part_1(&input)));
}

fn day_22_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "22.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 22 part 2", |b| b.iter(|| day_22::part_2(&input)));
}

fn day_23_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_23.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 23 part 1", |b| b.iter(|| day_23::part_1(&input)));
}

fn day_23_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_23.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 23 part 2", |b| b.iter(|| day_23::part_2(&input)));
}

fn day_24_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_24.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 24 part 1", |b| b.iter(|| day_24::part_1(&input)));
}

fn day_24_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_24.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 24 part 2", |b| b.iter(|| day_24::part_2(&input)));
}

fn day_25_part_1_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_25.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 25 part 1", |b| b.iter(|| day_25::part_1(&input)));
}

fn day_25_part_2_benchmark(c: &mut Criterion) {
    let input_name = INPUT_FOLDER.to_string() + "input_day_25.txt";
    let input = read_lines_untrimmed_from_file(&input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 25 part 2", |b| b.iter(|| day_25::part_2(&input)));
}

criterion_group!(benches_2016,
    day_01_part_1_benchmark,
    day_01_part_2_benchmark,
    day_02_part_1_benchmark,
    day_02_part_2_benchmark,
    day_03_part_1_benchmark,
    day_03_part_2_benchmark,
    day_04_part_1_benchmark,
    day_04_part_2_benchmark,
    day_05_part_1_benchmark,
    day_05_part_2_benchmark,
    day_06_part_1_benchmark,
    day_06_part_2_benchmark,
    day_07_part_1_benchmark,
    day_07_part_2_benchmark,
    day_08_part_1_benchmark,
    day_08_part_2_benchmark,
    day_09_part_1_benchmark,
    day_09_part_2_benchmark,
    day_10_part_1_benchmark,
    day_10_part_2_benchmark,
    day_11_part_1_benchmark,
    day_11_part_2_benchmark,
    day_12_part_1_benchmark,
    day_12_part_2_benchmark,
    day_13_part_1_benchmark,
    day_13_part_2_benchmark,
    day_14_part_1_benchmark,
    day_14_part_2_benchmark,
    day_15_part_1_benchmark,
    day_15_part_2_benchmark,
    day_16_part_1_benchmark,
    day_16_part_2_benchmark,
    day_17_part_1_benchmark,
    day_17_part_2_benchmark,
    day_18_part_1_benchmark,
    day_18_part_2_benchmark,
    day_19_part_1_benchmark,
    day_19_part_2_benchmark,
    day_20_part_1_benchmark,
    day_20_part_2_benchmark,
    day_21_part_1_benchmark,
    day_21_part_2_benchmark,
    day_22_part_1_benchmark,
    day_22_part_2_benchmark,
    day_23_part_1_benchmark,
    day_23_part_2_benchmark,
    day_24_part_1_benchmark,
    day_24_part_2_benchmark,
    day_25_part_1_benchmark,
    day_25_part_2_benchmark
);

criterion_main!(benches_2016);
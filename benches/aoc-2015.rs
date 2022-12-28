use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code::read_lines_untrimmed_from_file;
use advent_of_code::year_2015::*;


fn day_01_part_1_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_01.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 01 part 1", |b| b.iter(|| day_01::part_1(&input)));
}

fn day_01_part_2_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_01.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 01 part 2", |b| b.iter(|| day_01::part_2(&input)));
}

fn day_02_part_1_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_02.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 02 part 1", |b| b.iter(|| day_02::part_1(&input)));
}

fn day_02_part_2_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_02.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 02 part 2", |b| b.iter(|| day_02::part_2(&input)));
}

fn day_03_part_1_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_03.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 03 part 1", |b| b.iter(|| day_03::part_1(&input)));
}

fn day_03_part_2_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_03.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 03 part 2", |b| b.iter(|| day_03::part_2(&input)));
}

fn day_04_part_1_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_04.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 04 part 1", |b| b.iter(|| day_04::part_1(&input)));
}

fn day_04_part_2_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_04.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 04 part 2", |b| b.iter(|| day_04::part_2(&input)));
}

fn day_05_part_1_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_05.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 05 part 1", |b| b.iter(|| day_05::part_1(&input)));
}

fn day_05_part_2_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_05.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 05 part 2", |b| b.iter(|| day_05::part_2(&input)));
}

fn day_06_part_1_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_06.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 06 part 1", |b| b.iter(|| day_06::part_1(&input)));
}

fn day_06_part_2_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_06.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 06 part 2", |b| b.iter(|| day_06::part_2(&input)));
}

fn day_07_part_1_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_07.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 07 part 1", |b| b.iter(|| day_07::part_1(&input)));
}

fn day_07_part_2_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_XX.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 07 part 2", |b| b.iter(|| day_07::part_2(&input)));
}

fn day_08_part_1_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_08.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 08 part 1", |b| b.iter(|| day_08::part_1(&input)));
}

fn day_08_part_2_benchmark(c: &mut Criterion) {
    let input_name = "input/year_2015/input_day_08.txt";
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day 08 part 2", |b| b.iter(|| day_08::part_2(&input)));
}

/*
fn day_XX_part_1_benchmark(c: &mut Criterion) { // TODO
    let input_name = "input/year_2015/input_day_XX.txt";    // TODO
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day XX part 1", |b| b.iter(|| day_XX::part_1(&input))); // TODO 2x
}

fn day_XX_part_2_benchmark(c: &mut Criterion) { // TODO
    let input_name = "input/year_2015/input_day_XX.txt";    // TODO
    let input = read_lines_untrimmed_from_file(input_name)
        .expect("Reading file failed");

    c.bench_function("Bench day XX part 2", |b| b.iter(|| day_XX::part_2(&input))); // TODO 2x
}
 */

criterion_group!(benches,
    day_01_part_1_benchmark,
    day_01_part_2_benchmark,
    day_02_part_1_benchmark,
    day_02_part_2_benchmark,
    day_03_part_1_benchmark,
    day_03_part_2_benchmark,
    day_04_part_1_benchmark,
    day_04_part_2_benchmark

    /*
    day_XX_part_1_benchmark,    // TODO
    day_XX_part_2_benchmark     // TODO
     */
);

criterion_main!(benches);
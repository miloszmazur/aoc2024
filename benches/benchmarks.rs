use aoc2024::{day_01, day_02, day_03, day_04};

fn main() {
    divan::main();
}

#[divan::bench]
fn day_01_part_1() {
    day_01::part1(divan::black_box(include_str!("../src/day_01/input.txt")));
}

#[divan::bench]
fn day_01_part_2() {
    day_01::part2(divan::black_box(include_str!("../src/day_01/input.txt")));
}

#[divan::bench]
fn day_02_part_1() {
    day_02::part1(divan::black_box(include_str!("../src/day_01/input.txt")));
}

#[divan::bench]
fn day_02_part_2() {
    day_02::part2(divan::black_box(include_str!("../src/day_01/input.txt")));
}

#[divan::bench]
fn day_03_part_1() {
    day_03::part1(divan::black_box(include_str!("../src/day_01/input.txt")));
}

#[divan::bench]
fn day_03_part_2() {
    day_03::part2(divan::black_box(include_str!("../src/day_01/input.txt")));
}

#[divan::bench]
fn day_04_part_1() {
    day_04::part1(divan::black_box(include_str!("../src/day_01/input.txt")));
}

#[divan::bench]
fn day_04_part_2() {
    day_04::part2(divan::black_box(include_str!("../src/day_01/input.txt")));
}

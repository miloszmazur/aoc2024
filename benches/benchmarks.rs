use aoc2024::*;

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
    day_02::part1(divan::black_box(include_str!("../src/day_02/input.txt")));
}

#[divan::bench]
fn day_02_part_2() {
    day_02::part2(divan::black_box(include_str!("../src/day_02/input.txt")));
}

#[divan::bench]
fn day_03_part_1() {
    day_03::part1(divan::black_box(include_str!("../src/day_03/input.txt")));
}

#[divan::bench]
fn day_03_part_2() {
    day_03::part2(divan::black_box(include_str!("../src/day_03/input.txt")));
}

#[divan::bench]
fn day_04_part_1() {
    day_04::part1(divan::black_box(include_str!("../src/day_04/input.txt")));
}

#[divan::bench]
fn day_04_part_2() {
    day_04::part2(divan::black_box(include_str!("../src/day_04/input.txt")));
}

#[divan::bench]
fn day_05_part_1() {
    day_05::part1(divan::black_box(include_str!("../src/day_05/input.txt"))).unwrap();
}

#[divan::bench]
fn day_05_part_2() {
    day_05::part2(divan::black_box(include_str!("../src/day_05/input.txt"))).unwrap();
}

#[divan::bench]
fn day_06_part_1() {
    day_06::part1(divan::black_box(include_str!("../src/day_06/input.txt"))).unwrap();
}

#[divan::bench]
fn day_06_part_2() {
    day_06::part2::part2(divan::black_box(include_str!("../src/day_06/input.txt"))).unwrap();
}

#[divan::bench]
fn day_07_part_1() {
    day_07::part1(divan::black_box(include_str!("../src/day_07/input.txt"))).unwrap();
}

#[divan::bench]
fn day_07_part_2() {
    day_07::part2(divan::black_box(include_str!("../src/day_07/input.txt"))).unwrap();
}

#[divan::bench]
fn day_08_part_1() {
    day_08::part1::main(divan::black_box(include_str!("../src/day_08/input.txt"))).unwrap();
}

#[divan::bench]
fn day_08_part_2() {
    day_08::part2::main(divan::black_box(include_str!("../src/day_08/input.txt"))).unwrap();
}
// #[divan::bench]
// fn day_09_part_1() {
//     day_09::part1::main(divan::black_box(include_str!("../src/day_09/input.txt"))).unwrap();
// }

// #[divan::bench]
// fn day_09_part_2() {
//     day_09::part2::main(divan::black_box(include_str!("../src/day_09/input.txt"))).unwrap();
// }

#[divan::bench]
fn day_10_part_1() {
    day_10::part1::main(divan::black_box(include_str!("../src/day_10/input.txt"))).unwrap();
}

#[divan::bench]
fn day_10_part_2() {
    day_10::part2::main(divan::black_box(include_str!("../src/day_10/input.txt"))).unwrap();
}

#[divan::bench]
fn day_11_part_1() {
    day_11::part1::main(divan::black_box(include_str!("../src/day_11/input.txt"))).unwrap();
}

#[divan::bench]
fn day_11_part_1_cache() {
    day_11::part2::part1_cached(divan::black_box(include_str!("../src/day_11/input.txt"))).unwrap();
}

#[divan::bench]
fn day_11_part_2() {
    day_11::part2::main(divan::black_box(include_str!("../src/day_11/input.txt"))).unwrap();
}

#[divan::bench]
fn day_12_part_1() {
    day_12::part1::main(divan::black_box(include_str!("../src/day_12/input.txt"))).unwrap();
}

#[divan::bench]
fn day_12_part_2() {
    day_12::part2::main(divan::black_box(include_str!("../src/day_12/input.txt"))).unwrap();
}

#[divan::bench]
fn day_13_part_1() {
    day_13::part1::main(divan::black_box(include_str!("../src/day_13/input.txt"))).unwrap();
}

#[divan::bench]
fn day_13_part_2() {
    day_13::part2::main(divan::black_box(include_str!("../src/day_13/input.txt"))).unwrap();
}

#[divan::bench]
fn day_14_part_1() {
    day_14::part1::main(
        divan::black_box(include_str!("../src/day_14/input.txt")),
        divan::black_box(101),
        divan::black_box(103),
    )
    .unwrap();
}

#[divan::bench]
fn day_15_part_1() {
    day_15::part1::main(divan::black_box(include_str!("../src/day_15/input.txt"))).unwrap();
}

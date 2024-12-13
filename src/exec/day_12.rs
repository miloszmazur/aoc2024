use aoc2024::day_12;

fn main() {
    let input = include_str!("../day_12/input.txt");
    println!("Part 1: {}", day_12::part1::main(input).unwrap());
    println!("Part 2: {}", day_12::part2::main(input).unwrap());
}

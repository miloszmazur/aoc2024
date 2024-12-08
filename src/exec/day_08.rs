use aoc2024::day_08;

fn main() {
    let input = include_str!("../day_08/input.txt");
    println!("Part 1: {}", day_08::part1::main(input).unwrap());
    println!("Part 2: {}", day_08::part2::main(input).unwrap());
}

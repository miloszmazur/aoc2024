use aoc2024::day_09;

fn main() {
    let input = include_str!("../day_09/input.txt");
    println!("Part 1: {}", day_09::part1::main(input).unwrap());
    println!("Part 2: {}", day_09::part2::main(input).unwrap());
}

use aoc2024::day_11;

fn main() {
    let input = include_str!("../day_10/input.txt");
    println!("Part 1: {}", day_11::part1::main(input).unwrap());
    println!("Part 2: {}", day_11::part2::main(input).unwrap());
}

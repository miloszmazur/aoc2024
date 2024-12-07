use aoc2024::day_06;

fn main() {
    let input = include_str!("../day_06/input.txt");
    println!("Part 1: {}", day_06::part1(input).unwrap());
    println!("Part 2: {}", day_06::part2::part2(input).unwrap());
}

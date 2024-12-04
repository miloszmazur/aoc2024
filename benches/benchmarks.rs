use day_01;

fn main() {
    divan::main();
}

#[divan::bench]
fn dupa() {
    day_01::part1(divan::black_box(include_str!("../day_01/input.txt")));
}

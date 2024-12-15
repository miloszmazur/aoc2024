#[allow(non_snake_case)]
use anyhow::Result;
use nalgebra::matrix;
use rayon::prelude::*;
use regex::Regex;
use std::{ops, u128};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Point(i128, i128);

impl ops::Sub<&Point> for &Point {
    type Output = Point;
    fn sub(self, other: &Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl ops::Add<&Point> for &Point {
    type Output = Point;
    fn add(self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Point {
    fn times(&self, count: i128) -> Point {
        Point(self.0 * count, self.1 * count)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct ClawMachine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

impl ClawMachine {
    fn min_tokens_to_reach_prize(&self) -> Option<u128> {
        // 80*94 + 40*22 = 8400
        // 80*34 + 40*67 = 5400
        //
        // [94, 22; 24 67] [x0= 80 ;x1 =  40] = [8400 ; 5400]
        let mat_a = matrix![
            self.button_a.0 as f64, self.button_b.0 as f64;
            self.button_a.1 as f64 ,self.button_b.1 as f64];

        let det_a = mat_a.determinant();

        let mat_a0 = matrix![
                self.prize.0 as f64, self.button_b.0 as f64;
                self.prize.1 as f64 ,self.button_b.1 as f64];

        let mat_a1 = matrix![
                self.button_a.0 as f64, self.prize.0 as f64;
                self.button_a.1 as f64 ,self.prize.1 as f64];

        let a_times = (mat_a0.determinant() / det_a) as i128;
        let b_times = (mat_a1.determinant() / det_a) as i128;

        let result_point = &self.button_a.times(a_times) + &self.button_b.times(b_times);

        if result_point == self.prize {
            Some((a_times * 3 + b_times) as u128)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Result<Vec<ClawMachine>> {
    Ok(input.split("\n\n").map(|machine| {
        let re: Regex = Regex::new(
            r"Button A: X\+(?<AX>\d+), Y\+(?<AY>\d+)\nButton B: X\+(?<BX>\d+), Y\+(?<BY>\d+)\nPrize: X=(?<PX>\d+), Y=(?<PY>\d+)",
        )
        .unwrap();
        let caps = re.captures(machine).unwrap();
        let (_, [AX, AY, BX, BY, PX, PY]) = caps.extract();

        ClawMachine {
            button_a: Point(AX.parse().unwrap(), AY.parse().unwrap()),
            button_b: Point(BX.parse().unwrap(), BY.parse().unwrap()),
            prize: Point(PX.parse::<i128>().unwrap()+10000000000000, PY.parse::<i128>().unwrap() + 10000000000000),
        }
    }).collect())
}

pub fn main(input: &str) -> Result<u128> {
    let machines = parse(input)?;
    Ok(machines
        .into_par_iter()
        .flat_map(|machine| machine.min_tokens_to_reach_prize())
        .sum::<u128>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        let result = main(input);
        assert_eq!(result.unwrap(), 280);
    }

    #[test]
    fn test_no_result() {
        let input = "Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";
        let result = main(input);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_part1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let result = main(input);
        assert_eq!(result.unwrap(), 480);
    }
}

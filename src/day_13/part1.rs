use anyhow::{Context, Result};
use nalgebra::matrix;
use regex::Regex;
use std::{ops, u64};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Point(i64, i64);

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
    fn neighbours(&self) -> [Point; 4] {
        [
            self + &Point(0, 1),
            self + &Point(1, 0),
            self + &Point(0, -1),
            self + &Point(-1, 0),
        ]
    }
    fn diagonale(&self) -> [Point; 4] {
        [
            self + &Point(-1, -1),
            self + &Point(-1, 1),
            self + &Point(1, -1),
            self + &Point(1, 1),
        ]
    }
    fn is_below(&self, other: &Point) -> bool {
        self.0 <= other.0 && self.1 <= other.1
    }

    fn times(&self, count: i64) -> Point {
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
    fn min_tokens_to_reach_prize(&self) -> Option<u64> {
        // let matrix = matrix![
        //     self.button_a.0 as f64, self.button_a.1 as f64;
        //     self.button_b.0 as f64,self.button_b.1 as f64];
        // let prize_vec = matrix![self.prize.0 as f64, self.prize.1 as f64];
        // let result = prize_vec * matrix.try_inverse()?;
        // let a_times = result[0].floor() as i64;
        // let b_times = result[1].floor() as i64;

        // let result_point = &self.button_a.times(a_times) + &self.button_b.times(b_times);

        // if result_point == self.prize
        //     && 0 < a_times
        //     && a_times <= 100
        //     && b_times > 0
        //     && b_times <= 100
        // {
        //     dbg!(result[0].floor(), result[1].floor());
        //     Some((a_times * 3 + b_times) as u64)
        // } else {
        //     None
        // }

        let mut min_cost: u64 = u64::MAX;
        let mut found = false;
        for a in 0..=100 {
            for b in 0..=100 {
                if &self.button_a.times(a) + &self.button_b.times(b) == self.prize {
                    found = true;
                    let cost = a * 3 + b;
                    dbg!("found!", cost, min_cost, (cost as u64) < min_cost);
                    if (cost as u64) < min_cost {
                        min_cost = cost as u64;
                    }
                }
            }
        }
        if found {
            Some(min_cost)
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
            prize: Point(PX.parse().unwrap(), PY.parse().unwrap()),
        }
    }).collect())
}

pub fn main(input: &str) -> Result<u64> {
    let machines = parse(input)?;
    Ok(machines
        .into_iter()
        .flat_map(|machine| machine.min_tokens_to_reach_prize())
        .sum::<u64>())
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

// 80*94 + 40*22 = 8400
// 80*34 + 40*67 = 5400
//
//
// n * [A] + m [B] = [P]
//
//
//
//
// [80] * [94, 22] = [8400]
// [40]   [34, 67]   [5400]
//
// [80   = [8400] * ([94, 22])^-1
//  40]  = [5400]   ([34, 67])

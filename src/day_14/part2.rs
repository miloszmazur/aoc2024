#[allow(non_snake_case)]
use anyhow::Result;
use regex::Regex;
use std::{ops, thread::sleep, time::Duration};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Point(i128, i128);

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

impl ops::Mul<i128> for &Point {
    type Output = Point;

    fn mul(self, rhs: i128) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

struct Robot {
    position: Point,
    velocity: Point,
}
impl Robot {
    fn position_after_seconds(&self, seconds: i128, area_size: &Point) -> Point {
        let new_position = &self.position + &(&self.velocity * seconds);
        let scaled_result = Point(new_position.0 % area_size.0, new_position.1 % area_size.1);
        // scaled_result
        let abs_result = Point(
            if scaled_result.0 < 0 {
                area_size.0 + scaled_result.0
            } else {
                scaled_result.0
            },
            if scaled_result.1 < 0 {
                area_size.1 + scaled_result.1
            } else {
                scaled_result.1
            },
        );
        abs_result
    }
}

fn parse(input: &str) -> Result<Vec<Robot>> {
    Ok(input
        .lines()
        .map(|machine| {
            // Fix regex so it accounts for negative signs
            let re: Regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
            let caps = re.captures(machine).unwrap();
            let (_, [PX, PY, VX, VY]) = caps.extract();

            Robot {
                position: Point(PX.parse().unwrap(), PY.parse().unwrap()),
                velocity: Point(VX.parse().unwrap(), VY.parse().unwrap()),
            }
        })
        .collect())
}

pub fn pre_main(
    input: &str,
    arena_dims_x: i128,
    arena_deims_y: i128,
    seconds: i128,
    robots: &Vec<Robot>,
) -> Result<u64> {
    let arena_dims = Point(arena_dims_x, arena_deims_y);
    let robot_positions: Vec<Point> = robots
        .into_iter()
        .map(|robot| robot.position_after_seconds(seconds, &arena_dims))
        .collect();

    let mut map = vec![0].repeat((arena_dims.0 as usize * arena_dims.1 as usize) + 1);
    robot_positions.iter().for_each(|pos| {
        map[(pos.0 + pos.1 * arena_dims.0) as usize] =
            map[(pos.0 + pos.1 * arena_dims.0) as usize] + 1
    });

    for i in 0..arena_dims.1 {
        for j in 0..arena_dims.0 {
            if i == arena_dims.1 / 2 && j == arena_dims.0 / 2 {
                print!("{seconds}");
                continue;
            }
            if i == arena_dims.1 / 2 || j == arena_dims.0 / 2 {
                print!(" ");
                continue;
            }
            let asd = map[(i * arena_dims.0 + j) as usize];
            let asd = if asd == 0 { "." } else { &asd.to_string() };
            print!("{}", asd)
        }
        print!("\n");
    }
    let quadrants = robot_positions
        .into_iter()
        .fold([0_u64; 4], |mut acc, robot_pos| {
            if robot_pos.0 == arena_dims.0 / 2 || robot_pos.1 == arena_dims.1 / 2 {
                return acc;
            }

            if robot_pos.0 < arena_dims.0 / 2 {
                if robot_pos.1 < arena_dims.1 / 2 {
                    acc[0] += 1;
                } else {
                    acc[1] += 1;
                }
            } else {
                if robot_pos.1 < arena_dims.1 / 2 {
                    acc[2] += 1;
                } else {
                    acc[3] += 1
                }
            }
            acc
        });
    // dbg!(&quadrants);

    Ok(quadrants.iter().product())
}

pub fn main(input: &str, arena_dims_x: i128, arena_deims_y: i128) -> Result<u64> {
    let robots = parse(input)?;
    for i in 0..10000 {
        pre_main(input, arena_dims_x, arena_deims_y, i, &robots);
        // println!("{i}");
        // sleep(Duration::from_millis(50));
    }

    Ok(2137)
}

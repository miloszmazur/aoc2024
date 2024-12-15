use anyhow::{Error, Result};
use std::{ops, path::Display};

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

impl ops::Add<&Direction> for &Point {
    type Output = Point;
    fn add(self, other: &Direction) -> Point {
        match other {
            Direction::North => self + &Point(0, 1),
            Direction::East => self + &Point(1, 0),
            Direction::South => self + &Point(0, -1),
            Direction::West => self + &Point(-1, 0),
        }
    }
}

impl ops::Mul<i128> for &Point {
    type Output = Point;

    fn mul(self, rhs: i128) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Map {
    map: Vec<Vec<char>>,
    robot_position: Point,
    width: usize,
    height: usize,
    instructions: Vec<Direction>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Map {
    fn execute(&mut self) {
        self.instructions
            .clone()
            .iter()
            .enumerate()
            .for_each(|(index, instruction)| {
                self.move_robot(instruction);
            });
    }

    fn move_robot(&mut self, direction: &Direction) {
        let next_step = &self.robot_position + direction;
        let next_step_val = self.get_value(&next_step);
        match next_step_val {
            '.' => {
                self.set_value(&next_step, '@');
                self.set_value(&self.robot_position.clone(), '.');
                self.robot_position = next_step;
            }
            'O' => {
                // move boxes conditionally
                let free_space = (PointIterator {
                    direction: *direction,
                    position: next_step,
                    width: self.width,
                    height: self.height,
                })
                .take_while(|pos| self.get_value(&pos) != '#')
                .find(|pos| self.get_value(&pos) == '.');

                if let Some(next_dot) = free_space {
                    // move current box
                    self.set_value(&next_dot, 'O');
                    self.set_value(&next_step, '@');
                    self.set_value(&self.robot_position.clone(), '.');
                    self.robot_position = next_step;
                }
            }
            '#' => {
                // just chill
            }
            dupa => {
                panic!("unexpected next step: {dupa}")
            }
        }
    }

    fn get_value(&self, point: &Point) -> char {
        self.map[point.1 as usize][point.0 as usize]
    }

    fn set_value(&mut self, point: &Point, value: char) {
        self.map[point.1 as usize][point.0 as usize] = value;
    }

    fn get_boxes(&self) -> Vec<Point> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(yindex, line)| {
                line.iter().enumerate().filter_map(move |(xindex, value)| {
                    if *value == 'O' {
                        Some(Point(xindex as i128, yindex as i128))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct PointIterator {
    direction: Direction,
    position: Point,
    width: usize,
    height: usize,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next_position = &self.position + &self.direction;
        if next_position.0 < 0
            || next_position.1 < 0
            || next_position.0 >= self.width as i128
            || next_position.1 >= self.height as i128
        {
            None
        } else {
            self.position = next_position;
            Some(next_position)
        }
    }
}

pub fn main(input: &str) -> Result<i128> {
    let mut map = parse(input)?;

    // println!("{map}");
    map.execute();
    // println!("{map}");
    Ok(map.get_boxes().iter().map(|bb| bb.1 * 100 + bb.0).sum())
}

fn parse(input: &str) -> Result<Map> {
    let (map_lines, instructions_list) = input
        .split_once("\n\n")
        .ok_or_else(|| Error::msg("Invalid input format"))?;
    let instructions: Vec<Direction> = instructions_list
        .lines()
        .flat_map(|line| {
            line.chars().map(|charizard| match charizard {
                '^' => Direction::South,
                '>' => Direction::East,
                'v' => Direction::North,
                '<' => Direction::West,
                _ => panic!("wat"),
            })
        })
        .collect();

    let mut robot_pos = Point(-1, -1);
    let board = map_lines
        .lines()
        .enumerate()
        .map(|(yindex, line)| {
            line.chars()
                .enumerate()
                .inspect(|(xindex, char)| {
                    if char == &'@' {
                        robot_pos.0 = *xindex as i128;
                        robot_pos.1 = yindex as i128;
                    }
                })
                .map(|(xindex, char)| char)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if robot_pos.0 == -1 || robot_pos.1 == -1 {
        Err(Error::msg("failed to find robot :("))
    } else {
        let height = board.len();
        let width = board[0].len();
        Ok(Map {
            map: board,
            robot_position: robot_pos,
            width,
            height,
            instructions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let result = main(input);
        assert_eq!(result.unwrap(), 10092);
    }
}

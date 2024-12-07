use anyhow::{Context, Ok, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashSet;
use std::fmt;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_the_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

type PositionWithDirection = (Position, Direction);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position(usize, usize);

impl Position {
    fn add_direction(&self, direction: &Direction) -> Option<Position> {
        match direction {
            Direction::Up => {
                let (value, overflow) = self.0.overflowing_sub(1);
                if overflow {
                    return None;
                }
                Some(Position(value, self.1))
            }
            Direction::Down => Some(Position(self.0 + 1, self.1)),
            Direction::Left => {
                let (value, overflow) = self.1.overflowing_sub(1);
                if overflow {
                    return None;
                }
                Some(Position(self.0, value))
            }
            Direction::Right => Some(Position(self.0, self.1 + 1)),
        }
    }
}

#[derive(Clone)]
struct GuardPatrol {
    current_direction: Direction,
    board: Vec<Vec<char>>,
    guard_location: Position,
}

impl GuardPatrol {
    fn process_patrol(&mut self) {
        while let Some(next_position) = self.guard_location.add_direction(&self.current_direction) {
            if !self.within_patrol_area(&next_position) {
                break;
            }
            if self.will_collide(&next_position) {
                self.current_direction = self.current_direction.to_the_right();
                continue;
            }
            self.board[self.guard_location.0][self.guard_location.1] = 'X';
            self.guard_location = next_position;
        }
        self.board[self.guard_location.0][self.guard_location.1] = 'X';
    }

    fn within_patrol_area(&self, next_position: &Position) -> bool {
        next_position.0 < self.board.len() && next_position.1 < self.board[0].len()
    }

    fn will_collide(&self, next_position: &Position) -> bool {
        let value = self.board[next_position.0][next_position.1];
        value == '#' || value == 'O'
    }

    fn count_steps(&self) -> usize {
        self.board
            .iter()
            .map(|line| {
                line.iter()
                    .fold(0, |acc, value| if *value == 'X' { acc + 1 } else { acc })
            })
            .sum()
    }

    fn add_obstacle(&mut self, position: &Position) {
        self.board[position.0][position.1] = 'O';
    }

    fn check_loop(&mut self) -> bool {
        let mut visited_positions: FxHashSet<PositionWithDirection> = FxHashSet::default();

        while let Some(next_position) = self.guard_location.add_direction(&self.current_direction) {
            if !self.within_patrol_area(&next_position) {
                break;
            }
            let current_location_and_direction =
                (self.guard_location.clone(), self.current_direction.clone());

            if self.will_collide(&next_position) {
                if visited_positions.contains(&current_location_and_direction) {
                    return true;
                } else {
                    visited_positions.insert(current_location_and_direction);
                }
                self.current_direction = self.current_direction.to_the_right();
                continue;
            }
            self.guard_location = next_position;
        }
        // self.board[self.guard_location.0][self.guard_location.1] = 'X';

        false
    }
}

impl ToString for GuardPatrol {
    fn to_string(&self) -> String {
        let pretty_board: String = self
            .board
            .iter()
            .map(|chars| String::from_iter(chars))
            .collect::<Vec<String>>()
            .join("\n");
        pretty_board
    }
}

impl fmt::Debug for GuardPatrol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pretty_board: String = self
            .board
            .iter()
            .map(|chars| String::from_iter(chars))
            .collect::<Vec<String>>()
            .join("\n");
        f.write_str("-------\n")?;
        f.write_str(&pretty_board)?;
        f.write_str("\n")?;
        f.write_fmt(format_args!(
            "Position: ({}, {})",
            &self.guard_location.0, &self.guard_location.1
        ))?;
        f.write_str("\n")
    }
}

fn parse_input(input: &str) -> Result<GuardPatrol> {
    let board: Vec<Vec<char>> = input.lines().map(|f| f.chars().collect()).collect();
    let initial_guard_position = board
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == '^').map(|j| Position(i, j)))
        .context("failed to find guard position :(");

    Ok(GuardPatrol {
        board,
        current_direction: Direction::Up,
        guard_location: initial_guard_position?,
    })
}

pub fn part1(input: &str) -> Result<usize> {
    let mut patrol = parse_input(input)?;
    patrol.process_patrol();
    Ok(patrol.count_steps())
}

pub fn part2(input: &str) -> Result<usize> {
    let initial_patrol = parse_input(input)?;
    let mut patrol = initial_patrol.clone();
    patrol.process_patrol();
    let all_steps_positions: Vec<Position> = patrol
        .board
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, character)| {
                if *character == 'X' {
                    Some(Position(y, x))
                } else {
                    None
                }
            })
        })
        .collect();

    let result = all_steps_positions
        .par_iter()
        .filter(|obstacle_position| {
            let mut new_patrol = initial_patrol.clone();
            new_patrol.add_obstacle(&obstacle_position);
            new_patrol.check_loop()
        })
        .count();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let result = part2(input);
        assert_eq!(result.unwrap(), 6);
    }
}

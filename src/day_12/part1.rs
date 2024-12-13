#![allow(warnings)]
use anyhow::Result;
use rustc_hash::FxHashSet;
use std::{collections::VecDeque, ops};

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
            self + &Point(1, 0),
            self + &Point(0, 1),
            self + &Point(-1, 0),
            self + &Point(0, -1),
        ]
    }
}

struct Map {
    board: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
}
impl Map {
    fn within_map(&self, point: &Point) -> bool {
        !(point.0 < 0
            || point.1 < 0
            || point.0 >= self.board.len() as i64
            || point.1 >= self.board[0].len() as i64)
    }

    fn get_crop_id(&self, point: &Point) -> char {
        self.board[point.1 as usize][point.0 as usize]
    }
    fn get_all_areas(&mut self) -> Vec<Area> {
        let mut areas = Vec::new();
        while let Some(unvisited_point) = self.get_next_unvisited_point() {
            let crop_id = self.get_crop_id(&unvisited_point);
            let area = self.create_area(crop_id, unvisited_point);
            areas.push(area);
        }
        areas
    }

    fn get_next_unvisited_point(&self) -> Option<Point> {
        self.visited
            .iter()
            .enumerate()
            .filter_map(|(yindex, line)| {
                let xindex = line.iter().position(|val| !val)?;
                Some(Point(xindex as i64, yindex as i64))
            })
            .next()
    }

    fn create_area(&mut self, id: char, p0: Point) -> Area {
        let mut members = Vec::new();
        let mut pointz: VecDeque<Point> = VecDeque::new();
        pointz.push_back(p0);

        while let Some(point) = pointz.pop_back() {
            if !self.within_map(&point) || self.get_crop_id(&point) != id || self.visited[point.1 as usize][point.0 as usize] {
                continue
            }
            let neighbours = point.neighbours();
            members.push(point);
            pointz.extend(neighbours);
            self.visited[point.1 as usize][point.0 as usize] = true;

        }

        Area {
            id,
            members
        }
    }
}

fn parse(input: &str) -> Result<Map> {
    let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = board[0].len();
    let height = board.len();
    let line = vec![false; width];
    let visited = vec![line; height];
    Ok(Map { board, visited })
}

#[derive(Debug)]
struct Area {
    id: char,
    members: Vec<Point>,
}

impl Area {
    fn price(&self, board: &Map) -> u64 {
        self.area() * self.perimeter(board)
    }

    fn area(&self) -> u64 {
        self.members.len() as u64
    }

    fn perimeter(&self, board: &Map) -> u64 {
        self.members
            .iter()
            .map(|point| {
                point
                    .neighbours()
                    .iter()
                    .filter(|neighbour| {
                        !board.within_map(neighbour) || board.get_crop_id(neighbour) != self.id
                    })
                    .count() as u64
            })
            .sum()
    }
}

pub fn main(input: &str) -> Result<u64> {
    let mut mapp = parse(input)?;
    let areas = mapp
        .get_all_areas();
    // dbg!(&areas);
    Ok(areas
        .iter()
        .map(|area| area.price(&mapp))
        .sum())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_small_area() {
//         let input = "OOOOO
// OXOXO
// OOOOO
// OXOXO
// OOOOO";
//         let result = parse(input).unwrap().get_all_areas()[0].area();
//         assert_eq!(result, 21)
//     }
//     #[test]
//     fn test_small_permeter() {
//         let input = "OOOOO
// OXOXO
// OOOOO
// OXOXO
// OOOOO";
//         let mut mapp = parse(input).unwrap();
//         let result = mapp.get_all_areas()[0].perimeter(&mapp);
//         assert_eq!(result, )
//     }
//
//     #[test]
//     fn test_p1() {
//         let input = "RRRRIICCFF
// RRRRIICCCF
// VVRRRCCFFF
// VVRCCCJFFF
// VVVVCJJCFE
// VVIVCCJJEE
// VVIIICJJEE
// MIIIIIJJEE
// MIIISIJEEE
// MMMISSJEEE";
//         let result = main(input);
//         assert_eq!(result.unwrap(), 1930);
//     }
// }

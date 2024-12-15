#![allow(warnings)]
use anyhow::Result;
use rustc_hash::{FxHashMap, FxHashSet};
use std::arch::aarch64::int64x1_t;
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
        if self.within_map(point) {
            self.board[point.1 as usize][point.0 as usize]
        } else {
            '&'
        }
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
            if !self.within_map(&point)
                || self.get_crop_id(&point) != id
                || self.visited[point.1 as usize][point.0 as usize]
            {
                continue;
            }
            let neighbours = point.neighbours();
            members.push(point);
            pointz.extend(neighbours);
            self.visited[point.1 as usize][point.0 as usize] = true;
        }

        Area { id, members }
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
        let area = self.area();
        let sides = self.sides(board);
        // println!(
        //     "-----\nID: {:?}\nSides: {:?}\nArea: {:?}",
        //     self.id, sides, area
        // );

        area * sides
    }

    fn area(&self) -> u64 {
        self.members.len() as u64
    }

    // fn sides_gowno(&self, board: &Map) -> u64 {
    //     let mut WALLS = 0;
    //     let mut visited_fields: FxHashMap<Point, [bool; 4]> = Default::default();
    //     let first_point = self.members.iter().min_by_key(|f| f.1).unwrap();
    //     // N, E, S, W
    //     let mut point_stack: VecDeque<(Point, [bool; 4])> = VecDeque::new();
    //     point_stack.push_back((*first_point, [false, false, false, false]));
    //     while let Some((point, parent_walls)) = point_stack.pop_back() {
    //         if visited_fields.contains_key(&point) {
    //             let previously_built_walls = visited_fields.get(&point).unwrap();
    //             let false_walls = previously_built_walls
    //                 .iter()
    //                 .zip(parent_walls)
    //                 .filter(|(a, b)| *b && **a)
    //                 .count();
    //             WALLS -= false_walls as u64;
    //             let new_built_walls = [
    //                 parent_walls[0] || previously_built_walls[0],
    //                 parent_walls[1] || previously_built_walls[1],
    //                 parent_walls[2] || previously_built_walls[2],
    //                 parent_walls[3] || previously_built_walls[3],
    //             ];
    //             visited_fields.insert(point, new_built_walls);
    //             continue;
    //         }
    //
    //         // let [N, E, S, W] = num_walls;
    //         let neighbours = point.neighbours();
    //         let needed_walls =
    //             neighbours.map(|n| !board.within_map(&n) || board.get_crop_id(&n) != self.id);
    //
    //         // if !N && new_walls[0] {
    //         //     WALLS += 1;
    //         // }
    //         // if !E && new_walls[1] {
    //         //     WALLS += 1;
    //         // }
    //         // if !S && new_walls[2] {
    //         //     WALLS += 1;
    //         // }
    //         // if !W && new_walls[3] {
    //         //     WALLS += 1;
    //         // }
    //
    //         let mut built_walls: [bool; 4] = Default::default();
    //
    //         for index in 0..4 {
    //             built_walls[index] = !parent_walls[index] && needed_walls[index];
    //         }
    //
    //         WALLS += built_walls.iter().filter(|a| **a).count() as u64;
    //
    //         visited_fields.insert(point, needed_walls.clone());
    //         let map = neighbours
    //             .into_iter()
    //             .filter(|p| board.within_map(p) && board.get_crop_id(p) == self.id)
    //             .map(|p| (p, needed_walls.clone()));
    //         point_stack.extend(map)
    //     }
    //     WALLS
    // }

    fn sides(&self, board: &Map) -> u64 {
        self.members
            .iter()
            .map(|member| {
                let neighbours = member.neighbours();
                let n_map = neighbours.map(|neighbour| board.get_crop_id(&neighbour));
                let match_map = n_map.map(|val| val != self.id);
                let other_n = n_map.into_iter().filter(|val| *val != self.id).count();
                // if other_n == 0 {
                //     let diagonales = member.diagonale();
                //     let diagz = diagonales.iter().filter(|diag| board.get_crop_id(&diag) != self.id).count();
                //     return diagz as u64;
                // };
                // if other_n == 1 {
                //     return 0;
                // }
                if other_n == 3 {
                    return 2;
                }
                if other_n == 4 {
                    return 4;
                }
                let mut corners = 0;

                // czy jestem wklesly?
                let [SW, NW, SE, NE] = member.diagonale();
                if !match_map[0] && !match_map[1] && board.get_crop_id(&NE) != self.id {
                    corners += 1;
                }
                if !match_map[1] && !match_map[2] && board.get_crop_id(&SE) != self.id {
                    corners += 1;
                }
                if !match_map[2] && !match_map[3] && board.get_crop_id(&SW) != self.id {
                    corners += 1;
                }
                if !match_map[3] && !match_map[0] && board.get_crop_id(&NW) != self.id {
                    corners += 1;
                }

                if match_map[0] && match_map[2] || match_map[1] && match_map[3] {
                    return 0;
                }
                // let mut corcener_sum = 0;
                if other_n == 2 && match_map[0] && match_map[1]
                    || match_map[1] && match_map[2]
                    || match_map[2] && match_map[3]
                    || match_map[3] && match_map[0]
                {
                    corners += 1;
                }

                corners as u64
            })
            .inspect(|asd| {
                let a = asd;
            })
            .sum()
    }
}

pub fn main(input: &str) -> Result<u64> {
    let mut mapp = parse(input)?;
    let areas = mapp.get_all_areas();
    Ok(areas.iter().map(|area| area.price(&mapp)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn test_small_area() {
    //         let input = "AAAA
    // BBCD
    // BBCC
    // EEEC";
    //         let result = main(input);
    //         assert_eq!(result.unwrap(), 80)
    //     }
    // #[test]
    //     fn test_small_sides() {
    //         let input = "OOOOO
    // OXOXO
    // OOOOO
    // OXOXO
    // OOOOO";
    //         let mut mapp = parse(input).unwrap();
    //         let result = mapp.get_all_areas()[0].sides(&mapp);
    //         assert_eq!(result, 36)
    //     }
    //
    #[test]
    fn test_p2() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let result = main(input);
        assert_eq!(result.unwrap(), 1206);
    }
}

use std::{fmt, ops};

use anyhow::{Ok, Result};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

struct SignalMap {
    board: Vec<Vec<char>>,
    antennas: FxHashMap<char, Vec<Point>>,
    map_size: Point,
}

impl SignalMap {
    fn get_all_legal_antinodes(&mut self) -> FxHashSet<Point> {
        let ant: Vec<char> = self.antennas.keys().cloned().collect();
        ant.iter()
            .flat_map(|antenna_freq| self.get_legal_antinodes(&antenna_freq.clone()))
            .collect()
    }
    fn get_legal_antinodes(&mut self, antena: &char) -> FxHashSet<Point> {
        let mut legal_antinodes: FxHashSet<Point> = Default::default();
        let antena_points = &self.antennas[antena];
        for i in 0..antena_points.len() {
            for j in (i + 1)..antena_points.len() {
                legal_antinodes.insert(antena_points[i].clone());
                legal_antinodes.insert(antena_points[j].clone());
                let curr = &antena_points[i];
                let next = &antena_points[j];
                let distance = curr - next;
                let mut antinode = curr + &distance;
                while self.antinode_position_legal(&antinode) {
                    let &Point(y, x) = &antinode;
                    self.board[y as usize][x as usize] = '#';
                    legal_antinodes.insert(antinode.clone());
                    antinode = &antinode + &distance;
                }

                let mut antinode2 = next - &distance;
                while self.antinode_position_legal(&antinode2) {
                    let &Point(y, x) = &antinode2;
                    self.board[y as usize][x as usize] = '#';
                    legal_antinodes.insert(antinode2.clone());
                    antinode2 = &antinode2 - &distance;
                }
            }
        }
        legal_antinodes
    }

    fn antinode_position_legal(&self, point: &Point) -> bool {
        let &Point(y, x) = point;
        !(y < 0 || x < 0 || x >= self.map_size.1 || y >= self.map_size.0)
    }
}

impl fmt::Debug for SignalMap {
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
        f.write_str("\n")
    }
}

fn parse(input: &str) -> Result<SignalMap> {
    let mut freq_map = FxHashMap::default();
    let mut max_yindex = 0;
    let mut max_xindex = 0;

    let board = input
        .lines()
        .enumerate()
        .map(|(yindex, line)| {
            max_yindex = yindex;
            line.chars()
                .enumerate()
                .map(|(xindex, character)| {
                    max_xindex = xindex;
                    if character != '.' {
                        freq_map
                            .entry(character)
                            .or_insert(Vec::new())
                            .push(Point(yindex as i64, xindex as i64))
                    }
                    character
                })
                .collect()
        })
        .collect();

    Ok(SignalMap {
        board,
        antennas: freq_map,
        map_size: Point(max_xindex as i64 + 1, max_yindex as i64 + 1),
    })
}

pub fn main(input: &str) -> Result<usize> {
    let mut mapp = parse(input)?;
    let result = Ok(mapp.get_all_legal_antinodes().iter().count());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
.......#....
........A...
.........A..
............
............";
        let result = main(input);
        assert_eq!(result.unwrap(), 34);
    }
}

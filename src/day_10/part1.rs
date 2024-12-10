use anyhow::Result;
use rustc_hash::FxHashSet;
use std::{collections::VecDeque, ops};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
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

struct Map {
    board: Vec<Vec<i64>>,
}

impl Map {
    fn all_cheerios(&self) -> Vec<Point> {
        self.board
            .iter()
            .enumerate()
            .flat_map(|(yindex, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(xindex, height)| {
                        if height == &0 {
                            Some(Point(xindex as i64, yindex as i64))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Point>>()
    }

    fn num_paths_to_peaks(&self, p0: &Point) -> i64 {
        let mut cheerio_score = 0;
        let mut points_to_check = VecDeque::new();
        points_to_check.push_front(p0.clone());
        let neighbors = [Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)];
        let mut visited_nines: FxHashSet<Point> = Default::default();
        while let Some(point) = points_to_check.pop_front() {
            if self.get_height(&point) == 9 && !visited_nines.contains(&point) {
                cheerio_score += 1;
                visited_nines.insert(point.clone());
                continue;
            }
            let curr_neighbours: Vec<Point> = neighbors
                .iter()
                .map(|neighbor| &point + neighbor)
                .filter(|next_point| self.is_legal_neighbor(&point, next_point))
                .collect();
            points_to_check.extend(curr_neighbours);
        }
        cheerio_score
    }

    fn is_legal_neighbor(&self, current_point: &Point, next_point: &Point) -> bool {
        self.within_map(next_point)
            && self.get_height(next_point) == self.get_height(current_point) + 1
    }

    fn within_map(&self, point: &Point) -> bool {
        !(point.0 < 0
            || point.1 < 0
            || point.0 >= self.board.len() as i64
            || point.1 >= self.board[0].len() as i64)
    }

    fn get_height(&self, point: &Point) -> i64 {
        self.board[point.1 as usize][point.0 as usize]
    }
}

fn parse(input: &str) -> Result<Map> {
    let board = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if let Some(num) = c.to_digit(10) {
                        num as i64
                    } else {
                        -1_i64
                    }
                })
                .collect()
        })
        .collect();
    Ok(Map { board })
}

pub fn main(input: &str) -> Result<i64> {
    let mapp = parse(input)?;
    let result = mapp
        .all_cheerios()
        .iter()
        .map(|trailhead| mapp.num_paths_to_peaks(trailhead))
        .sum();
    dbg!(&result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_simple() {
        let input = "0123
1234
8765
9876";
        let result = main(input);
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_p1_harder() {
        let input = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
        let result = main(input);
        assert_eq!(result.unwrap(), 2);
    }

    // ..90..9
    // ...1.98
    // ...2..7
    // 6543456
    // 765.987
    // 876....
    // 987....

    #[test]
    fn test_p1_simple2() {
        let input = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        let result = main(input);
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn test_p1_harder2() {
        let input = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
        let result = main(input);
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn test_p1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let result = main(input);
        assert_eq!(result.unwrap(), 36);
    }
}

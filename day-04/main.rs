type WordSearch = Vec<Vec<char>>;

#[derive(Debug)]
struct CoordinatePair(i64, i64);

impl CoordinatePair {
    fn add(&self, other: &CoordinatePair) -> CoordinatePair {
        CoordinatePair(self.0 + other.0, self.1 + other.1)
    }
}

fn part1(input: &str) -> usize {
    let puzzle = parse_wordsearch(input);

    locate_char('X', &puzzle)
        .iter()
        .map(|found_x| count_xmas(&puzzle, found_x))
        .sum()
}

fn count_xmas(puzzle: &WordSearch, coord: &CoordinatePair) -> usize {
    // najpierw
    let remaining = ['M', 'A', 'S'];
    // a, potem rzeźba
    let check_me: [CoordinatePair; 8] = [
        CoordinatePair(-1, -1),
        CoordinatePair(-1, 0),
        CoordinatePair(-1, 1),
        CoordinatePair(0, -1),
        CoordinatePair(0, 1),
        CoordinatePair(1, -1),
        CoordinatePair(1, 0),
        CoordinatePair(1, 1),
    ];

    let mut sum = 0;

    for direction in check_me {
        let mut found = true;
        for i in 0..3 {
            let mut next_coordinate = coord.add(&direction);
            for _ in 1..=i {
                next_coordinate = next_coordinate.add(&direction);
            }
            if next_coordinate.0 < 0
                || next_coordinate.1 < 0
                || next_coordinate.1 >= puzzle.len() as i64
                || next_coordinate.0 >= puzzle[0].len() as i64
            {
                found = false;
                break;
            }
            if puzzle[next_coordinate.1 as usize][next_coordinate.0 as usize] != remaining[i] {
                found = false;
                break;
            }
        }
        if found {
            sum += 1;
        }
    }

    sum
}

fn locate_char(char_to_find: char, puzzle: &WordSearch) -> Vec<CoordinatePair> {
    puzzle
        .iter()
        .enumerate()
        .flat_map(|(y_index, val)| {
            val.iter()
                .enumerate()
                .filter_map(|(x_index, character)| {
                    if *character == char_to_find {
                        Some(CoordinatePair(
                            i64::try_from(x_index.clone()).unwrap(),
                            i64::try_from(y_index.clone()).unwrap(),
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<CoordinatePair>>()
        })
        .collect()
}

fn parse_wordsearch(input: &str) -> WordSearch {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part2(input: &str) -> usize {
    let puzzle = parse_wordsearch(input);

    locate_char('A', &puzzle)
        .into_iter()
        .filter(|found_x| is_x_mas(&puzzle, found_x))
        .count()
}

fn is_x_mas(puzzle: &WordSearch, coord: &CoordinatePair) -> bool {
    let check_me: [CoordinatePair; 8] = [
        CoordinatePair(-1, -1),
        CoordinatePair(-1, 0),
        CoordinatePair(-1, 1),
        CoordinatePair(0, -1),
        CoordinatePair(0, 1),
        CoordinatePair(1, -1),
        CoordinatePair(1, 0),
        CoordinatePair(1, 1),
    ];

    if coord.0 < 1
        || coord.1 < 1
        || coord.1 >= (puzzle.len() - 1) as i64
        || coord.0 >= (puzzle[0].len() - 1) as i64
    {
        return false;
    }

    let first_diagonal = [
        puzzle[(coord.1 - 1) as usize][(coord.0 - 1) as usize],
        puzzle[(coord.1 + 1) as usize][(coord.0 + 1) as usize],
    ];

    let second_diagonal = [
        puzzle[(coord.1 + 1) as usize][(coord.0 - 1) as usize],
        puzzle[(coord.1 - 1) as usize][(coord.0 + 1) as usize],
    ];

    first_diagonal.contains(&'M') & first_diagonal.contains(&'S')
        && second_diagonal.contains(&'M') & second_diagonal.contains(&'S')
}

fn main() {
    let input = include_str!("input.txt");
    println!("p1");
    dbg!(part1(input));
    println!("p2");
    dbg!(part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let var_name = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let input = var_name;
        let result = part1(input);
        assert_eq!(result, 18)
    }

    #[test]
    fn test_p2() {
        let var_name = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let input = var_name;
        let result = part2(input);
        assert_eq!(result, 9)
    }
}

use anyhow::{Context, Ok, Result};

#[derive(Debug, Eq, PartialEq)]
enum Operator {
    Mul,
    Add,
}

impl Operator {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Mul => a * b,
            Operator::Add => a + b,
        }
    }
}

#[derive(Debug)]
struct Polynomial {
    expected: i64,
    parts: Vec<i64>,
}
impl Polynomial {
    fn has_solution(&self) -> Result<bool> {
        let acc = self.parts[0];
        let cursor = 1;

        let calculate = self.solution_inner(acc, cursor);
        let result = calculate == self.expected;

        dbg!("------------");
        dbg!(&self.parts);
        dbg!(&self.expected);
        dbg!(&calculate);
        dbg!(&result);
        Ok(result)
    }

    fn solution_inner(&self, acc: i64, cursor: usize) -> i64 {
        if cursor >= self.parts.len() || acc >= self.expected {
            return acc;
        }
        let value = self.parts[cursor];
        let result = self.solution_inner(acc * value, cursor + 1);
        if result != self.expected {
            self.solution_inner(acc + value, cursor + 1)
        } else {
            result
        }
    }
}

fn parse(input: &str) -> Result<Vec<Polynomial>> {
    Ok(input
        .lines()
        .map(|line| {
            let (result, numbers) = line.split_once(":").context("malformed line")?;
            let result: i64 = result.parse()?;

            Ok(Polynomial {
                expected: result,
                parts: numbers
                    .split(" ")
                    .flat_map(|n| n.parse().ok())
                    .collect::<Vec<i64>>(),
            })
        })
        .collect::<Result<Vec<Polynomial>>>()?)
}
pub fn part1(input: &str) -> Result<i64> {
    let polyms = parse(input)?;
    let result = polyms
        .iter()
        .filter_map(|polym| {
            if polym.has_solution().ok()? {
                Some(polym.expected)
            } else {
                None
            }
        })
        .sum();
    Ok(result)
}

pub fn part2(input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = "292: 11 6 16 20";
        let result = part1(input);
        assert_eq!(result.unwrap(), 292);
    }
    #[test]
    fn test_p1() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        let result = part1(input);
        assert_eq!(result.unwrap(), 3749);
    }

    #[test]
    fn test_p2() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        let result = part2(input);
        assert_eq!(result.unwrap(), 11387);
    }
}

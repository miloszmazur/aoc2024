use anyhow::{Context, Result};
use rayon::prelude::*;

struct Stones {
    stones_list: Vec<i64>,
}

impl Iterator for Stones {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        let new_stones: Vec<i64> = self
            .stones_list
            .par_iter()
            .flat_map(|stone| match stone {
                0 => Vec::from([1]),
                dupa if (dupa.ilog10() + 1) % 2u32 == 0u32 => {
                    let digits = (dupa.ilog10() + 1);
                    Vec::from([
                        dupa / (10i64.pow(digits / 2)),
                        dupa % (10i64.pow(digits / 2)),
                    ])
                }
                _ => Vec::from([stone * 2024]),
            })
            .collect();
        self.stones_list = new_stones;
        Some(self.stones_list.clone())
    }
}

fn parse(input: &str) -> Result<Stones> {
    let list = input
        .split(" ")
        .map(|n| {
            n.trim()
                .parse::<i64>()
                .context(format!("failed to parse {n} as a number"))
        })
        .collect::<Result<Vec<i64>>>()?;
    Ok(Stones { stones_list: list })
}

pub fn main(input: &str) -> Result<usize> {
    let stones = parse(input)?;

    Ok(stones.take(25).last().context("failed to take :(")?.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blink_once() {
        let input = "0 1 10 99 999";
        let result = parse(input).unwrap().take(1).last().unwrap().len();
        assert_eq!(result, 7);
    }

    #[test]
    fn blink_twice() {
        let input = "125 17";
        let result = parse(input).unwrap().take(2).last().unwrap();
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_part1() {
        let input = "125 17";
        let result = parse(input).unwrap().take(25).last().unwrap();
        assert_eq!(result.len(), 55312);
    }
}

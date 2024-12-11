use anyhow::{Context, Result};
use rustc_hash::FxHashMap;

struct Stones {
    stones_list: Vec<u64>,
    cache: FxHashMap<(u64, u8), u64>,
}

impl Stones {
    fn calculate_depth(&mut self, depth: u8) -> u64 {
        self.stones_list
            .clone()
            .into_iter()
            .map(|val| self.inner_calculate_depth(val, depth))
            .sum()
    }

    fn inner_calculate_depth(&mut self, value: u64, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }
        if let Some(cached) = self.cache.get(&(value, depth)) {
            *cached
        } else {
            let sum = match value {
                0 => self.inner_calculate_depth(1, depth - 1),
                dupa if (dupa.ilog10() + 1) % 2u32 == 0u32 => {
                    let digits = dupa.ilog10() + 1;
                    self.inner_calculate_depth(dupa / (10u64.pow(digits / 2)), depth - 1)
                        + self.inner_calculate_depth(dupa % (10u64.pow(digits / 2)), depth - 1)
                }
                _ => self.inner_calculate_depth(value * 2024, depth - 1),
            };
            self.cache.insert((value, depth), sum);
            sum
        }
    }
}

fn parse(input: &str) -> Result<Stones> {
    let list = input
        .split(" ")
        .map(|n| {
            n.trim()
                .parse::<u64>()
                .context(format!("failed to parse {n} as a number"))
        })
        .collect::<Result<Vec<u64>>>()?;
    Ok(Stones {
        stones_list: list,
        cache: Default::default(),
    })
}

pub fn part1_cached(input: &str) -> Result<u64> {
    let mut stones = parse(input)?;
    Ok(stones.calculate_depth(25))
}

pub fn main(input: &str) -> Result<u64> {
    let mut stones = parse(input)?;
    Ok(stones.calculate_depth(75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn blink_once() {
            let input = "0 1 10 99 999";
            let result = parse(input).unwrap().calculate_depth(1);
            dbg!(&result);

            assert_eq!(result, 7);
        }

        #[test]
        fn blink_twice() {
            let input = "125 17";
            let result = parse(input).unwrap().calculate_depth(2);
            dbg!(&result);
            assert_eq!(result, 4);
        }

        #[test]
        fn test_part1() {
            let input = "125 17";
            let result = parse(input).unwrap().calculate_depth(25);
            assert_eq!(result, 55312);
        }

        #[test]
        fn test_part2() {
            let input = "125 17";
            let result = parse(input).unwrap().calculate_depth(75);
            assert_eq!(result, 65601038650482);
        }
    }
}

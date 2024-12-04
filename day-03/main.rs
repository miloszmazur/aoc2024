use regex::{Match, Regex};

fn part1(input: &str) -> i64 {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| (&c).extract())
        .map(|(_, [mul_left, mul_right])| {
            mul_left.parse::<i64>().unwrap() * mul_right.parse::<i64>().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let re: Regex = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

    let mut do_we_do = true;
    let mut sum = 0;
    for capture in re.captures_iter(input) {
        if let Some(x) = capture.get(0) {
            let matched_str = x.as_str();
            let result = match matched_str {
                "do()" => {
                    do_we_do = true;
                    0
                }
                "don't()" => {
                    do_we_do = false;
                    0
                }
                _ => {
                    if do_we_do {
                        let (mul_left, mul_right) = (
                            get_i64_value(capture.get(2)).unwrap(),
                            get_i64_value(capture.get(3)).unwrap(),
                        );
                        mul_left * mul_right
                    } else {
                        0
                    }
                }
            };
            sum = sum + result;
        } else {
            panic!(":(")
        }
    }
    sum
}

fn get_i64_value(asd: Option<Match>) -> Option<i64> {
    asd?.as_str().parse::<i64>().ok()
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

    fn test_re() {
        let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let input = "mul(21,37)";
        let result: i64 = re
            .captures_iter(input)
            .map(|c| (&c).extract())
            .map(|(_, [mul_left, mul_right])| {
                mul_left.parse::<i64>().unwrap() * mul_right.parse::<i64>().unwrap()
            })
            .sum();
        assert_eq!(result, 21 * 37)
    }

    #[test]
    fn test_p1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = part1(input);
        assert_eq!(result, 161)
    }

    #[test]
    fn test_p2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48)
    }
}

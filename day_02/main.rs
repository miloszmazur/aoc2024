type Line = Vec<i32>;

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_safe(&parse_line(line)))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_safe_with_fault_tolerance(&parse_line(line)))
        .count()
}

fn parse_line(line: &str) -> Line {
    line.split_whitespace()
        .map(|value_str| value_str.parse::<i32>().unwrap())
        .collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let first_sign = (report[0] - report[1]).signum();
    if first_sign == 0 {
        return false;
    }
    report.windows(2).all(|pair| {
        if let [a, b] = pair {
            let diff = (a).abs_diff(*b);
            (a - b).signum() == first_sign && diff <= 3 && diff >= 1
        } else {
            panic!()
        }
    })
}

fn is_safe_with_fault_tolerance(report: &Vec<i32>) -> bool {
    is_safe(report)
        || report.iter().enumerate().any(|(index, _)| {
            let mut cloned_report = report.clone();
            cloned_report.remove(index);
            is_safe(&cloned_report)
        })
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
        let input = "\
            7 6 4 2 1\n\
            1 2 7 8 9\n\
            9 7 6 2 1\n\
            1 3 2 4 5\n\
            8 6 4 4 1\n\
            1 3 6 7 9";
        let result = part1(input);

        assert_eq!(result, 2)
    }

    #[test]
    fn test_p2() {
        let input = "\
            7 6 4 2 1\n\
            1 2 7 8 9\n\
            9 7 6 2 1\n\
            1 3 2 4 5\n\
            8 6 4 4 1\n\
            1 3 6 7 9";
        let result = part2(input);

        assert_eq!(result, 4)
    }
}

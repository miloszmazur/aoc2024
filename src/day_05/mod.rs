use std::cmp::Ordering;

use anyhow::{self, Context, Ok, Result};

type SleighUpdates = Vec<usize>;

#[derive(Debug)]
struct Predicate {
    before: usize,
    after: usize,
}

pub fn part1(input: &str) -> Result<usize> {
    let (predicates, pages) = parse_input(input)?;

    let result = pages
        .iter()
        .filter(|manual| is_safe(&predicates, &manual))
        .map(|safe_manual| get_middle(safe_manual))
        .sum();
    Ok(result)
}


fn is_safe(predicates: &[Predicate], manual: &SleighUpdates) -> bool {
    let safe_manual = make_manual_safe(predicates, &manual);

    safe_manual.iter().zip(manual).all(|(a, b)| a == b)
}

fn get_middle(manual: &SleighUpdates) -> usize {
    manual[manual.len() / 2]
}

fn parse_input(input: &str) -> Result<(Vec<Predicate>, Vec<SleighUpdates>)> {
    let mut lines = input.lines();

    let predicates = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|pred| {
            let (before, after) = pred.split_once('|').context("Failed to split predicate")?;
            Ok(Predicate {
                before: before.parse().context("Failed to parse 'before' value")?,
                after: after.parse().context("Failed to parse 'after' value")?,
            })
        })
        .collect::<Result<Vec<Predicate>>>()?;

    let manuals = lines
        .map(|line| {
            line.split(',')
                .map(|val| val.parse().context("Failed to parse manual value"))
                .collect::<Result<Vec<usize>>>()
        })
        .collect::<Result<Vec<SleighUpdates>>>()?;

    Ok((predicates, manuals))
}

pub fn part2(input: &str) -> Result<usize, anyhow::Error> {
    let (predicates, pages) = parse_input(input)?;


    let result = pages
        .iter()
        .filter(|manual| !is_safe(&predicates, manual))
        .map(|unsafe_manual| make_manual_safe(&predicates, unsafe_manual))
        .map(|safe_manual| get_middle(&safe_manual))
        .sum();
    Ok(result)
}

fn make_manual_safe(predicates: &[Predicate], unsafe_manual: &Vec<usize>) -> Vec<usize> {
    let mut safe_manual = unsafe_manual.clone();
    safe_manual.sort_by(|a, b| {
        // assume all pages are unique
        let matching_predicate = predicates.iter().find(|pred| {
            (pred.after == *a && pred.before == *b) || (pred.after == *b && pred.before == *a)
        });

        if let Some(predicate) = matching_predicate {
            if predicate.before == *b && predicate.after == *a {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Equal
        }
    });

    safe_manual
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let result = part1(input);
        assert_eq!(result.unwrap(), 143);
    }

    #[test]
    fn test_p2() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let result = part2(input);
        assert_eq!(result.unwrap(), 123);
    }
}

use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let mut g1_locs: Vec<usize> = Vec::new();
    let mut g2_locs: Vec<usize> = Vec::new();
    for line in input.lines() {
        let mut it = line.split_whitespace();
        g1_locs.push(it.next().unwrap().parse().unwrap());
        g2_locs.push(it.next().unwrap().parse().unwrap());
    }
    g1_locs.sort();
    g2_locs.sort();
    g1_locs
        .iter()
        .zip(g2_locs)
        .map(|(loc_1, loc2)| loc_1.abs_diff(loc2))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut g1_locs: Vec<usize> = Vec::new();
    let mut g2_locs: Vec<usize> = Vec::new();
    for line in input.lines() {
        let mut it = line.split_whitespace();
        g1_locs.push(it.next().unwrap().parse().unwrap());
        g2_locs.push(it.next().unwrap().parse().unwrap());
    }
    g1_locs.sort();
    g2_locs.sort();

    let mut location_frequency: HashMap<usize, usize> = HashMap::new();
    for location_id in g2_locs {
        *location_frequency.entry(location_id).or_insert(0) += 1;
    }

    g1_locs
        .iter()
        .map(|x| {
            let y = match location_frequency.get(x) {
                Some(heh) => heh,
                None => &0,
            };
            x * y
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let input = "\
            3   4\n\
            4   3\n\
            2   5\n\
            1   3\n\
            3   9\n\
            3   3";

        let result = part1(input);

        assert_eq!(result, 11)
    }

    #[test]
    fn test_p2() {
        let input = "\
            3   4\n\
            4   3\n\
            2   5\n\
            1   3\n\
            3   9\n\
            3   3";

        let result = part2(input);

        assert_eq!(result, 31)
    }
}

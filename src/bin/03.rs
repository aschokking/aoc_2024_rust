advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    // parse out the valid mul patterns
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    input.lines().map(|line| {
        re.captures_iter(line).map(|cap| {
            let a: u64 = cap[1].parse().unwrap();
            let b: u64 = cap[2].parse().unwrap();
            a * b
        }).reduce(|acc, x| acc + x).unwrap()
    }).reduce(|acc, x| acc + x).into()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

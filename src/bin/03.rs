advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    // parse out the valid mul patterns

    input.lines().map(|line| {
        part_one_process_line(line)
    }).reduce(|acc, x| acc + x).into()
}

fn part_one_process_line(line: &str) -> u64 {
    // parse out the valid mul patterns
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(line).map(|cap| {
        let a: u64 = cap[1].parse().unwrap();
        let b: u64 = cap[2].parse().unwrap();
        a * b
    }).reduce(|acc, x| acc + x).unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    input.lines().map(|line| {
        part_two_process_line(line)
    }).reduce(|acc, x| acc + x).into()
}

fn part_two_process_line(line: &str) -> u64 {
    // strip out any don't sections
    dbg!(line);
    let dont_re = Regex::new(r"don't\(\).*?(do\(\)|$)").unwrap();
    dbg!(line);
    let line = dont_re.replace_all(line, "");
    part_one_process_line(&line)
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
        let result = part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}

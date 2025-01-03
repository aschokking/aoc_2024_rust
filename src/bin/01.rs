advent_of_code::solution!(1);

use counter::Counter;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right) = parse_input(input);
    // dbg!(&left);
    // dbg!(&right);

    // Sort each list
    left.sort();
    right.sort();

    // Zip lists together and compute the difference between values
    let mut result = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        // take difference in absolute value
        result += r.abs_diff(*l);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse_input(input);

    // Make a counter for the right list with counts of occurences
    let right_counts = right.iter().collect::<Counter<_>>();
    left.into_iter()
        .fold(0, |acc, value| acc + value * right_counts[&value] as u64)
        .into()
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .fold((vec![], vec![]), |(mut a, mut b), line| {
            let (left, right) = line.split_whitespace().collect_tuple().unwrap();
            a.push(left.parse().unwrap());
            b.push(right.parse().unwrap());
            (a, b)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1721  979\n366   299\n675   1456\n";
        let (a, b) = parse_input(input);
        assert_eq!(a, vec![1721, 366, 675]);
        assert_eq!(b, vec![979, 299, 1456]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

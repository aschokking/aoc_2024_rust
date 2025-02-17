advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = parse_input(input);
    let deltas: Vec<Vec<i64>> = lines
        .iter()
        .map(|line| {
            line.windows(2)
                .map(|pair| pair[1] as i64 - pair[0] as i64)
                .collect()
        })
        .collect();
    let valid_reports = deltas
        .iter()
        .filter(|report| {
            report.iter().all(|&x| (x >= 1 && x <= 3))
                || report.iter().all(|&x| (x >= -3 && x <= -1))
        })
        .count();

    Some(valid_reports as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = parse_input(input);

    // What does it mean for there to be 1 bad value?
    // It means that there is a value that is greater than 3 or less than -3 OR of the opposite sign
    // How do we track that removing that 1 bad number will allow us to get to the end?
    // If we remove a number, we need to update the deltas to reflect that
    // We could also brute force it, see how many values are invalid under each scheme and if the number is 1 just call it good? Nah that doesn't mean that the
    // sequence without that value is valid.

    // Different idea, let's do this iteratively and just remove the bad value when we hit it
    // But how will we know which of 2 values is the bad one, especially at the start of a list? Like 3 items in, which direction should things go? Just try both?

    (lines
        .iter()
        .filter(|line| {
            let valid = part_two_valid_check(line.to_vec(), true, true) || part_two_valid_check(line.to_vec(), false, true);
            valid
        })
        .count() as u64)
        .into()
}

fn part_two_valid_check(line: Vec<u64>, ascending: bool, allow_drop: bool) -> bool {
    let mut valid = true;
    // iterate over line by index
    for (i, _value) in line.iter().enumerate() {
        // first item, no pair to compare
        if i == 0 {
            continue;
        }
        // if we've dropped a value, we need to compare the current value to the value before the dropped value
        let delta = line[i] as i64 - line[i - 1] as i64;

        if ascending && (delta > 3 || delta < 1) || !ascending && (delta < -3 || delta > -1) {
            if allow_drop {
                // try recursing to see if the list is valid without this value or the previous value
                let mut line_clone_i = line.clone();
                line_clone_i.remove(i);
                let mut line_clone_i_minus_1 = line.clone();
                line_clone_i_minus_1.remove(i - 1);
                valid = part_two_valid_check(line_clone_i, ascending, false)
                    || part_two_valid_check(line_clone_i_minus_1, ascending, false);
                return valid;
            } else {
                valid = false;
                break;
            }
        }
    }
    valid
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_remove_first() {
        let line = vec![6, 1, 3, 4];
        assert_eq!(part_two_valid_check(line.clone(), true, true), true);
    }

    #[test]
    fn test_part_two_valid_check() {
        let line = vec![1, 2, 3, 4];
        assert_eq!(part_two_valid_check(line.clone(), true, true), true);
        assert_eq!(part_two_valid_check(line.clone(), false, true), false);

        let line = vec![1, 1, 3, 4];
        assert_eq!(part_two_valid_check(line.clone(), true, true), true);
        assert_eq!(part_two_valid_check(line.clone(), false, true), false);

        let line = vec![2, 0, 4, 5, 6];
        assert_eq!(part_two_valid_check(line.clone(), true, true), true);
        assert_eq!(part_two_valid_check(line.clone(), false, true), false);

        // two issues fails
        let line = vec![1, 1, 3, 4, 4];
        assert_eq!(part_two_valid_check(line.clone(), true, true), false);
        assert_eq!(part_two_valid_check(line.clone(), false, true), false);

        let line = vec![1, 1, 1];
        assert_eq!(part_two_valid_check(line.clone(), true, true), false);
        assert_eq!(part_two_valid_check(line.clone(), false, true), false);

        let line = vec![1, 2, 4, 4];
        assert_eq!(part_two_valid_check(line.clone(), true, true), true);
        assert_eq!(part_two_valid_check(line.clone(), false, true), false);

        let line = vec![1, 4, 0, 5];
        assert_eq!(part_two_valid_check(line.clone(), true, true), true);
        assert_eq!(part_two_valid_check(line.clone(), false, true), false);

        let line = vec![4, 3, 2, 1];
        assert_eq!(part_two_valid_check(line.clone(), true, true), false);
        assert_eq!(part_two_valid_check(line.clone(), false, true), true);

        let line = vec![4, 3, 1, 1];
        assert_eq!(part_two_valid_check(line.clone(), true, true), false);
        assert_eq!(part_two_valid_check(line.clone(), false, true), true);
    }
}

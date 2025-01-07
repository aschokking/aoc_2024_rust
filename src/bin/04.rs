use std::collections::HashSet;

use advent_of_code::{parse_input, Direction, ALL_DIRECTIONS_8};
use ndarray::Array2;
use queues::*;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    // turn input into a matrix of letters
    let map = parse_input(input);
    // iterate over the map, everytime we hit an 'x' search the 8 directions for an mas
    // if we find one, increment the count
    let mut count = 0;
    for ((x, y), c) in map.indexed_iter() {
        //println!("Checking {}, {} which is: {}", x, y, c);
        if *c != 'X' {
            continue;
        }

        for dir in ALL_DIRECTIONS_8.iter() {
            if search_for_mas(&map, x, y, *dir) {
                count += 1;
            }
        }
    }

    Some(count)
}

fn search_for_mas(map: &Array2<char>, x: usize, y: usize, dir: Direction) -> bool {
    let mut cur_x = x;
    let mut cur_y = y;
    let mut q: Queue<char> = queue!['M', 'A', 'S'];

    loop {
        cur_x = (cur_x as i32 + dir.0) as usize;
        cur_y = (cur_y as i32 + dir.1) as usize;

        if cur_x >= map.shape()[0] || cur_y >= map.shape()[1] {
            return false;
        }

        if map[[cur_x, cur_y]] != q.peek().unwrap() {
            return false;
        }

        q.remove().unwrap();

        if q.size() == 0 {
            //println!("Found MAS at {}, {} in dir {:?}", x, y, dir);
            return true;
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input(input);
    // iterate over the map, everytime we hit an 'x' search the 8 directions for an mas
    // if we find one, increment the count
    let mut count = 0;
    for ((x, y), c) in map.indexed_iter() {
        //println!("Checking {}, {} which is: {}", x, y, c);
        if *c != 'A' {
            continue;
        }

        if search_for_mas_cross(&map, x, y) {
            count += 1;
        }
    }

    Some(count)
}

fn search_for_mas_cross(map: &Array2<char>, x: usize, y: usize) -> bool {
    // if we're at an edge there can't be a MAS cross
    if x < 1 || y < 1 || x >= map.shape()[0] - 1 || y >= map.shape()[1] - 1 {
        return false;
    }

    // check the 4 directions for MAS
    [[(1, 1), (-1, -1)], [(1, -1), (-1, 1)]].iter().all(|dirs| {
        let letters: HashSet<char> = HashSet::from_iter(dirs.iter().map(|dir| {
            map[[(x as i32 + dir.0) as usize, (y as i32 + dir.1) as usize]]
        }));

        letters == HashSet::from(['M', 'S'])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

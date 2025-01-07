pub mod template;

// Use this file to add helper functions and additional modules.

use ndarray::Array2;

pub fn parse_input(input: &str) -> Array2<char> {
    let mut map = Array2::from_elem(
        (input.lines().next().unwrap().len(), input.lines().count()),
        ' ',
    );
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[[x, y]] = c;
        }
    }
    map
}

pub type Coord = (usize, usize);
pub type Direction = (i32, i32);
pub const ALL_DIRECTIONS: [Direction; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub const ALL_DIRECTIONS_8: [Direction; 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

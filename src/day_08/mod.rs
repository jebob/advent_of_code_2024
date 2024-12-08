use std::{collections::HashMap, fs};

use itertools::Itertools;

pub fn part_1() {
    let contents = fs::read_to_string("src/day_08/input.txt")
        .unwrap()
        .replace("\r\n", "\n");

    let rows: Vec<_> = contents.split("\n").collect();
    let nrows = rows.len();
    let ncols = rows[0].len();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i_row, row) in contents.split("\n").enumerate() {
        for (i_col, el) in row.chars().enumerate() {
            if el != '.' {
                antennas.entry(el).or_default().push((i_row, i_col));
            }
        }
    }
    let result = antennas
        .iter()
        .map(|(_, locations)| find_antinodes(&locations, nrows, ncols))
        .flatten()
        .unique()
        .count();
    println!("{:?}", result);
}

fn find_antinodes(antennas: &[(usize, usize)], nrows: usize, ncols: usize) -> Vec<(usize, usize)> {
    let mut results = vec![];
    for (i_antenna, (antenna_1x, antenna_1y)) in antennas.iter().enumerate() {
        for (j_antenna, (antenna_2x, antenna_2y)) in antennas.iter().enumerate() {
            if j_antenna <= i_antenna {
                continue;
            }
            // first
            if (antenna_2x * 2 >= *antenna_1x) & (antenna_2y * 2 >= *antenna_1y) {
                let new_x = antenna_2x * 2 - antenna_1x;
                let new_y = antenna_2y * 2 - antenna_1y;
                if (new_x < ncols) & (new_y < nrows) {
                    results.push((new_x, new_y));
                }
            }
            // second
            if (antenna_1x * 2 >= *antenna_2x) & (antenna_1y * 2 >= *antenna_2y) {
                let new_x = antenna_1x * 2 - antenna_2x;
                let new_y = antenna_1y * 2 - antenna_2y;
                if (new_x < ncols) & (new_y < nrows) {
                    results.push((new_x, new_y));
                }
            }
        }
    }
    results
}
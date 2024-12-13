use std::fs;

use itertools::Itertools;

type Coord = (usize, usize);

pub fn part_1() {
    let contents = fs::read_to_string("src/day_10/input.txt")
        .unwrap()
        .replace("\r\n", "\n");

    let grid: Vec<Vec<usize>> = contents
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|char| char.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let mut result = 0usize;
    for i_row in 0..grid.len() {
        for i_col in 0..grid.len() {
            if grid[i_row][i_col] == 0 {
                result += find_nines_from_this_position(&grid, i_row, i_col)
                    .iter()
                    .unique()
                    .count();
            }
        }
    }
    println!("{:?}", result);
}

fn find_nines_from_this_position(
    grid: &Vec<Vec<usize>>,
    i_row: usize,
    i_col: usize,
) -> Vec<(usize, usize)> {
    let this_value = grid[i_row][i_col];
    // println!("{:?}", this_value);
    if this_value == 9 {
        return vec![(i_row, i_col)];
    }
    let mut totals = vec![];
    if i_row > 0 && grid[i_row - 1][i_col] == this_value + 1 {
        totals.append(&mut find_nines_from_this_position(grid, i_row - 1, i_col));
    }
    if i_col > 0 && grid[i_row][i_col - 1] == this_value + 1 {
        totals.append(&mut find_nines_from_this_position(grid, i_row, i_col - 1));
    }
    if i_row < grid.len() - 1 && grid[i_row + 1][i_col] == this_value + 1 {
        totals.append(&mut find_nines_from_this_position(grid, i_row + 1, i_col));
    }
    if i_col < grid.len() - 1 && grid[i_row][i_col + 1] == this_value + 1 {
        totals.append(&mut find_nines_from_this_position(grid, i_row, i_col + 1));
    }
    totals
}

pub fn part_2() {
    let contents = fs::read_to_string("src/day_10/input.txt")
        .unwrap()
        .replace("\r\n", "\n");

    let grid: Vec<Vec<usize>> = contents
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|char| char.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let mut result = 0usize;
    for i_row in 0..grid.len() {
        for i_col in 0..grid.len() {
            if grid[i_row][i_col] == 0 {
                result += find_nines_from_this_position(&grid, i_row, i_col).len();
            }
        }
    }
    println!("{:?}", result);
}

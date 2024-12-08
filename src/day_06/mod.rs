use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct PlayerState {
    x: usize,
    y: usize,
    dir: usize, // 0 is upwards, 1 is right etc
}



pub fn part_1() {
    let raw_maze = fs::read_to_string("src/day_06/input.txt")
        .unwrap()
        .replace("\r\n", "\n");

    let mut player_state: Option<PlayerState> = None;
    let mut maze: Vec<Vec<_>> = vec![];
    let player_regex = Regex::new(r"\^").unwrap();
    for (i_row, row) in raw_maze.split("\n").enumerate() {
        if let Some(match_) = player_regex.find(row) {
            player_state = Some(PlayerState {
                x: match_.start(),
                y: i_row,
                dir: 0,
            });
        }
        maze.push(row.chars().map(|char_| char_ == '#').collect());
    }
    let mut player_state = player_state.unwrap();
    let maze = maze;
    let mut visited = HashSet::new();
    visited.insert(player_state);
    // println!("{:?}", player_state);
    // println!("{:?}", maze);
    let nrows = maze.len();
    let ncols = maze[0].len();
    loop {
        let (next_x, next_y) = match player_state.dir {
            0 => {
                if player_state.y == 0 {
                    break;
                }
                (player_state.x, player_state.y - 1)
            }
            1 => {
                if player_state.x == ncols-1 {
                    break;
                }
                (player_state.x + 1, player_state.y)
            }
            2 => {
                if player_state.y == nrows-1 {
                    break;
                }
                (player_state.x, player_state.y + 1)
            }
            3 => {
                if player_state.x == 0 {
                    break;
                }
                (player_state.x - 1, player_state.y)
            }
            _ => panic!("what?"),
        };
        if maze[next_y][next_x] {
            player_state.dir = (player_state.dir + 1) % 4;
        } else {
            player_state.x = next_x;
            player_state.y = next_y;
            visited.insert(player_state);
        }
        println!("{:?}", player_state);
    }
    let x: usize = visited.iter().map(|state| (state.x, state.y)).unique().count();
    println!("{:?}", x);
}

fn is_row_valid(row: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    let mut forbidden_symbols = HashSet::<usize>::new();
    for symb in row.iter() {
        // println!("{:?} in {:?}?", symb, forbidden_symbols);
        if forbidden_symbols.contains(symb) {
            return false;
        }
        if let Some(other_symbols) = rules.get(symb) {
            forbidden_symbols.extend(other_symbols);
        }
    }
    true
}

pub fn part_2() {
    let top = fs::read_to_string("src/day_06/input.txt")
        .unwrap()
        .replace("\r\n", "\n");
    let bottom = fs::read_to_string("src/day_06/input2.txt")
        .unwrap()
        .replace("\r\n", "\n");

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    for (left, right) in top.split("\n").filter(|row| !row.is_empty()).map(|row| {
        let (left, right) = row.split_at(row.find("|").unwrap());
        (
            left.parse::<usize>().unwrap(),
            right[1..].parse::<usize>().unwrap(),
        )
    }) {
        rules.entry(right).or_default().push(left);
    }
    println!("{:?}", rules);

    let rows: Vec<_> = bottom
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|row| !row.is_empty())
        .filter(|row| !is_row_valid(row, &rules))
        .map(|row| reorder_row(&row, &rules))
        .collect();

    let total: usize = rows.iter().map(|row| row[row.len() / 2]).sum();
    println!("{:?}", total);
}

fn reorder_row(row: &[usize], rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut new_row = row.to_vec();
    println!("{:?}", new_row);
    for this_idx in 0..new_row.len() {
        let mut match_found = true;
        while match_found {
            match_found = false;
            if let Some(items_to_swap_with) = rules.get(&new_row[this_idx]) {
                for other_item in items_to_swap_with {
                    if let Some(other_idx) = new_row.iter().position(|&r| r == *other_item) {
                        if other_idx < this_idx {
                            continue;
                        }

                        (new_row[this_idx], new_row[other_idx]) =
                            (new_row[other_idx], new_row[this_idx]);
                        println!("{:?}", new_row);

                        match_found = true;
                        break;
                    }
                }
            }
        }
    }
    new_row
}

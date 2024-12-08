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
                if player_state.x == ncols - 1 {
                    break;
                }
                (player_state.x + 1, player_state.y)
            }
            2 => {
                if player_state.y == nrows - 1 {
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
    let x: usize = visited
        .iter()
        .map(|state| (state.x, state.y))
        .unique()
        .count();
    println!("{:?}", x);
}

fn is_row_valid(row: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    let mut forbidden_symbols = HashSet::<usize>::new();
    for symb in row.iter() {
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
    let raw_maze = fs::read_to_string("src/day_06/input.txt")
        .unwrap()
        .replace("\r\n", "\n");

    let mut player_state: Option<PlayerState> = None;
    let mut maze: Vec<Vec<_>> = vec![];
    let player_regex = Regex::new(r"\^").unwrap();
    for (i_row, row) in raw_maze.split("\n").enumerate() {
        if row.is_empty() {continue;}
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
    let mut obstruction_positions = vec![];
    visited.insert((player_state.x, player_state.y));
    loop {
        let next_player_state = match increment_state(&player_state, &maze) {
            Some(p_state) => p_state,
            None => break,
        };
        
        if !visited.contains(&(next_player_state.x, next_player_state.y)) {
            let mut alternative_maze: Vec<Vec<_>> = maze.iter().map(|row| row.clone()).collect();
            alternative_maze[next_player_state.y][next_player_state.x] = true;
            if is_infinite_loop(player_state, &alternative_maze) {
                obstruction_positions.push(next_player_state);
            }

        }
        visited.insert((player_state.x, player_state.y));
        player_state = next_player_state;
    }
    println!("{:?}", obstruction_positions.iter().map(|p_state| (p_state.x, p_state.y)).unique().count());
}

fn is_infinite_loop(
    mut player_state: PlayerState,
    maze: &Vec<Vec<bool>>,
) -> bool {
    let mut visited = HashSet::new();
    loop {
        let next_player_state = match increment_state(&player_state, &maze) {
            Some(p_state) => p_state,
            None => return false, // gone off the edge of the board: not an infinite loop
        };
        if visited.contains(&next_player_state) {
            return true;
        }
        player_state = next_player_state;
        // println!("{:?}", player_state);
        visited.insert(player_state);
    }
}

fn increment_state(player_state: &PlayerState, maze: &Vec<Vec<bool>>) -> Option<PlayerState> {
    let nrows = maze.len();
    let ncols = maze[0].len();
    let (next_x, next_y) = match player_state.dir {
        0 => {
            if player_state.y == 0 {
                return None;
            }
            (player_state.x, player_state.y - 1)
        }
        1 => {
            if player_state.x == ncols - 1 {
                return None;
            }
            (player_state.x + 1, player_state.y)
        }
        2 => {
            if player_state.y == nrows - 1 {
                return None;
            }
            (player_state.x, player_state.y + 1)
        }
        3 => {
            if player_state.x == 0 {
                return None;
            }
            (player_state.x - 1, player_state.y)
        }
        _ => panic!("what?"),
    };
    let mut next_player_state = player_state.clone();
    // println!("{:?}{:?}", next_x, next_y);
    if maze[next_y][next_x] {
        next_player_state.dir = (player_state.dir + 1) % 4;
    } else {
        next_player_state.x = next_x;
        next_player_state.y = next_y;
    };
    Some(next_player_state)
}

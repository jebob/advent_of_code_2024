use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn part_1() {
    let top = fs::read_to_string("src/day_05/input.txt")
        .unwrap()
        .replace("\r\n", "\n");
    let bottom = fs::read_to_string("src/day_05/input2.txt")
        .unwrap()
        .replace("\r\n", "\n");
    // 5121 too high

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
        .filter(|row| is_row_valid(row, &rules))
        .collect();

    let total: usize = rows.iter().map(|row| row[row.len() / 2]).sum();
    println!("{:?}", total);
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
    let top = fs::read_to_string("src/day_05/input.txt")
        .unwrap()
        .replace("\r\n", "\n");
    let bottom = fs::read_to_string("src/day_05/input2.txt")
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

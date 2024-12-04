use std::fs;

enum IsAscending {
    True,
    False,
    Unknown,
}

pub fn part_1() {
    let file_path = "src/day_02/small.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut total_safe: usize = 0;
    for row in contents.split("\r\n") {
        if row.is_empty() {
            continue;
        }
        let row_vals: Vec<usize> = row.split(" ").map(|x| x.parse().unwrap()).collect();
        println!("{}", row);
        if row_is_safe(&row_vals) {
            total_safe += 1;
        }
    }
    println!("{}", total_safe);
}

pub fn part_2() {
    let file_path = "src/day_02/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut total_safe: usize = 0;
    for row in contents.split("\r\n") {
        if row.is_empty() {
            continue;
        }
        println!("{}", row);
        let row_vals: Vec<usize> = row.split(" ").map(|x| x.parse().unwrap()).collect();
        let row_candidates = row_candidates(&row_vals);
        for candidate in row_candidates {
            if row_is_safe(&candidate) {
                total_safe += 1;
                break;
            }
        }
    }
    println!("{}", total_safe);
}

fn row_is_safe(row: &[usize]) -> bool {
    let mut is_ascending = IsAscending::Unknown;
    let mut last_level_option: Option<&usize> = None;
    let mut unsafety_found = false;
    for this_level in row {
        if let Some(last_level) = last_level_option {
            if (this_level + 3 < *last_level) | (*this_level > last_level + 3) {
                // Unsafe!
                unsafety_found = true;
                break;
            };
            match is_ascending {
                IsAscending::True if this_level <= last_level => {
                    // Unsafe!
                    unsafety_found = true;
                    break;
                }
                IsAscending::False if this_level >= last_level => {
                    // Unsafe!
                    unsafety_found = true;
                    break;
                }
                IsAscending::Unknown if this_level > last_level => {
                    is_ascending = IsAscending::True;
                }
                IsAscending::Unknown if this_level < last_level => {
                    is_ascending = IsAscending::False;
                }
                IsAscending::Unknown => {
                    // Unsafe!
                    unsafety_found = true;
                    break;
                }
                // must be ascending/descending in the correct direction
                _ => {}
            }
        }
        last_level_option = Some(this_level);
    }
    !unsafety_found
}

fn row_candidates(row: &[usize]) -> Vec<Vec<usize>> {
    let mut result = vec![];
    result.push(row.to_vec());
    for idx in 0..row.len() {
        let mut new_row = vec![];
        for (idx2, element) in row.iter().enumerate() {
            if idx2 != idx {
                new_row.push(*element);
            }
        }
        result.push(new_row);
    }
    result
}

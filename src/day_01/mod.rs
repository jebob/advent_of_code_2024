use std::collections::BTreeMap;
use std::fs;
use std::iter::zip;

pub fn part_1() {
    let file_path = "src/day_01/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut l_list: Vec<usize> = vec![];
    let mut r_list: Vec<usize> = vec![];
    for row in contents.split("\r\n") {
        if row.is_empty() {
            continue;
        }

        let left_and_right: Vec<_> = row.split("   ").collect();
        l_list.push(left_and_right.first().unwrap().parse().unwrap());
        r_list.push(left_and_right.last().unwrap().parse().unwrap());
    }
    l_list.sort();
    r_list.sort();

    let mut total_diff: usize = 0;
    for (l_el, r_el) in zip(l_list, r_list) {
        if l_el > r_el {
            total_diff += l_el - r_el;
        } else {
            total_diff += r_el - l_el;
        }
    }

    println!("{}", total_diff);
}

pub fn part_2() {
    let file_path = "src/day_01/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut l_dict: BTreeMap<usize, usize> = BTreeMap::new();
    let mut r_dict: BTreeMap<usize, usize> = BTreeMap::new();
    for row in contents.split("\r\n") {
        if row.is_empty() {
            continue;
        }

        let left_and_right: Vec<_> = row.split("   ").collect();
        let l_num: usize = left_and_right.first().unwrap().parse().unwrap();
        let r_num: usize = left_and_right.last().unwrap().parse().unwrap();
        l_dict.insert(l_num, l_dict.get(&l_num).unwrap_or(&0) + 1);
        r_dict.insert(r_num, r_dict.get(&r_num).unwrap_or(&0) + 1);
    }

    let mut total_similarity: usize = 0;
    for (l_el, l_freq) in l_dict.iter() {
        total_similarity += r_dict.get(l_el).unwrap_or(&0) * l_freq * l_el
    }

    println!("{}", total_similarity);
}

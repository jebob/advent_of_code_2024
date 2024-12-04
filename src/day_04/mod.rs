use std::fs;

use regex::Regex;

pub fn part_1() {
    let file_path = "src/day_04/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let rows = contents
        .split("\r\n")
        .map(|row| row.chars().collect::<Vec<_>>())
        .filter(|row| !row.is_empty())
        .collect::<Vec<_>>();
    let ncols = rows[0].len();

    let contents_no_spaces = contents.replace("\r\n", "\n");

    let regexes = vec![
        regex::Regex::new("XMAS").unwrap(),
        regex::Regex::new(&format!(
            r"X[\s\S]{{{}}}M[\s\S]{{{}}}A[\s\S]{{{}}}S",
            ncols + 1,
            ncols + 1,
            ncols + 1
        ))
        .unwrap(),
        regex::Regex::new(&format!(
            r"X[\s\S]{{{}}}M[\s\S]{{{}}}A[\s\S]{{{}}}S",
            ncols, ncols, ncols,
        ))
        .unwrap(),
        regex::Regex::new(&format!(
            r"X[\s\S]{{{}}}M[\s\S]{{{}}}A[\s\S]{{{}}}S",
            ncols - 1,
            ncols - 1,
            ncols - 1,
        ))
        .unwrap(),
        regex::Regex::new("SAMX").unwrap(),
        regex::Regex::new(&format!(
            r"S[\s\S]{{{}}}A[\s\S]{{{}}}M[\s\S]{{{}}}X",
            ncols + 1,
            ncols + 1,
            ncols + 1
        ))
        .unwrap(),
        regex::Regex::new(&format!(
            r"S[\s\S]{{{}}}A[\s\S]{{{}}}M[\s\S]{{{}}}X",
            ncols, ncols, ncols,
        ))
        .unwrap(),
        regex::Regex::new(&format!(
            r"S[\s\S]{{{}}}A[\s\S]{{{}}}M[\s\S]{{{}}}X",
            ncols - 1,
            ncols - 1,
            ncols - 1,
        ))
        .unwrap(),
    ];
    println!("{:?}", regexes);
    let total: usize = regexes
        .iter()
        .map(|some_regex| count_regex_matches_with_overlapping(some_regex, &contents_no_spaces))
        .sum();

    println!("{:?}", total);
}

fn count_regex_matches_with_overlapping(regex: &Regex, content: &str) -> usize {
    let mut total = 0;
    let mut remaining_content = content;
    let mut this_index = regex.find(content);
    let mut total_offset = 0;
    let nrows = 10;
    println!("{}", regex);
    while this_index.is_some() {
        total_offset += this_index.unwrap().start();
        println!(
            "{:?}, {:?}, {:?}",
            total_offset / nrows,
            total_offset % nrows,
            this_index.unwrap()
        );

        total += 1;
        let earliest_possible_start = this_index.unwrap().start() + 1;
        remaining_content = &remaining_content[earliest_possible_start..];
        this_index = regex.find(remaining_content);
    }
    total
}

pub fn part_2() {
    let file_path = "src/day_04/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let rows = contents
        .split("\r\n")
        .map(|row| row.chars().collect::<Vec<_>>())
        .filter(|row| !row.is_empty())
        .collect::<Vec<_>>();
    let ncols = rows[0].len();

    let contents_no_spaces = contents.replace("\r\n", "\n");

    let regexes = vec![
        regex::Regex::new(&format!(
            r"M.M[\s\S]{{{}}}A[\s\S]{{{}}}S.S",
            ncols - 1,
            ncols - 1
        ))
        .unwrap(),
        regex::Regex::new(&format!(
            r"M.S[\s\S]{{{}}}A[\s\S]{{{}}}M.S",
            ncols - 1,
            ncols - 1
        ))
        .unwrap(),
        regex::Regex::new(&format!(
            r"S.M[\s\S]{{{}}}A[\s\S]{{{}}}S.M",
            ncols - 1,
            ncols - 1
        ))
        .unwrap(),
        regex::Regex::new(&format!(
            r"S.S[\s\S]{{{}}}A[\s\S]{{{}}}M.M",
            ncols - 1,
            ncols - 1
        ))
        .unwrap(),
    ];
    println!("{:?}", regexes);
    let total: usize = regexes
        .iter()
        .map(|some_regex| count_regex_matches_with_overlapping(some_regex, &contents_no_spaces))
        .sum();

    println!("{:?}", total);
}

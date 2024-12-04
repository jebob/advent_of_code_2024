use std::fs;

pub fn part_1() {
    let file_path = "src/day_04/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let xmas_regex = regex::Regex::new(r"XMAS").unwrap();
    let samx_regex = regex::Regex::new(r"SAMX").unwrap();
    let mut total = 0usize;
    // left-right
    total += xmas_regex.find_iter(&contents).count();
    total += samx_regex.find_iter(&contents).count();

    // up-down
    let rows = contents
        .split("\r\n")
        .map(|row| row.chars().collect::<Vec<_>>())
        .filter(|row| !row.is_empty())
        .collect::<Vec<_>>();
    let ncols = rows[0].len();
    for i_col in 0..ncols {
        let str_to_check: String = rows.iter().map(|row| row[i_col]).collect();
        total += xmas_regex.find_iter(&str_to_check).count();
        total += samx_regex.find_iter(&str_to_check).count();
    }

    // diagonals on the top row
    let nrows = rows.len();
    for i_col in 0..ncols {
        // diagonals starting on the top row and going right
        let str_to_check: String = rows
            .iter()
            .enumerate()
            .filter_map(|(i_row, row)| row.get(i_col + i_row))
            .collect();
        total += xmas_regex.find_iter(&str_to_check).count();
        total += samx_regex.find_iter(&str_to_check).count();
        // diagonals starting on the top row and going left
        let str_to_check: String = rows
            .iter()
            .enumerate()
            .filter_map(|(i_row, row)| {
                if i_col < i_row {
                    None
                } else {
                    row.get(i_col - i_row)
                }
            })
            .collect();
        total += xmas_regex.find_iter(&str_to_check).count();
        total += samx_regex.find_iter(&str_to_check).count();
        // input is square, so skip the diagonals starting in the bottom corners, as they also end in the top corners which have already been counted
        if (i_col == 0) | (i_col == ncols - 1) {
            continue;
        }
        // diagonals starting on the bottom row and going left
        let str_to_check: String = rows
            .iter()
            .enumerate()
            .filter_map(|(i_row, row)| {
                if i_col + nrows < 1 + i_row {
                    None
                } else {
                    row.get(i_col + nrows - 1 - i_row)
                }
            })
            .collect();
        total += xmas_regex.find_iter(&str_to_check).count();
        total += samx_regex.find_iter(&str_to_check).count();
        // diagonals starting on the bottom row and going right
        let str_to_check: String = rows
            .iter()
            .enumerate()
            .filter_map(|(i_row, row)| {
                if i_col + 1 + i_row < nrows {
                    None
                } else {
                    row.get(i_col + 1 + i_row - nrows)
                }
            })
            .collect();
        total += xmas_regex.find_iter(&str_to_check).count();
        total += samx_regex.find_iter(&str_to_check).count();
    }

    println!("{:?}", total);
}

// pub fn part_2() {
//     // let file_path = "src/day_03/small2.txt";
//     let file_path = "src/day_03/input.txt";
//     let contents = fs::read_to_string(file_path).unwrap();
//     let do_regex = regex::Regex::new(r"do\(\)").unwrap();
//     let dont_regex = regex::Regex::new(r"don't\(\)").unwrap();
//     let do_indicies: Vec<_> = do_regex.find_iter(&contents).map(|x| x.start()).collect();
//     let dont_indicies: Vec<_> = dont_regex.find_iter(&contents).map(|x| x.start()).collect();
//     let my_regex = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

//     let mut total = 0usize;
//     for (idx, (_, [left, right])) in my_regex
//         .captures_iter(&contents)
//         .map(|c| (c.get(0).unwrap().start(), c.extract()))
//     {
//         let maybe_last_do = do_indicies.iter().filter(|x| x < &&idx).max();
//         let maybe_last_dont = dont_indicies.iter().filter(|x| x < &&idx).max();
//         println!("{:?}{:?}", maybe_last_do, maybe_last_dont);
//         if let Some(last_dont) = maybe_last_dont {
//             if last_dont > maybe_last_do.unwrap_or(&0) {
//                 continue;
//             }
//         }
//         let result = left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
//         total += result;
//     }
//     println!("{:?}", total);
// }

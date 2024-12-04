use std::fs;

pub fn part_1() {
    let file_path = "src/day_03/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let my_regex = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut total = 0usize;
    for (_, [left, right]) in my_regex.captures_iter(&contents).map(|c| c.extract()) {
        let result = left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
        total += result;
    }
    println!("{:?}", total);
}

pub fn part_2() {
    // let file_path = "src/day_03/small2.txt";
    let file_path = "src/day_03/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let do_regex = regex::Regex::new(r"do\(\)").unwrap();
    let dont_regex = regex::Regex::new(r"don't\(\)").unwrap();
    let do_indicies: Vec<_> = do_regex.find_iter(&contents).map(|x| x.start()).collect();
    let dont_indicies: Vec<_> = dont_regex.find_iter(&contents).map(|x| x.start()).collect();
    let my_regex = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut total = 0usize;
    for (idx, (_, [left, right])) in my_regex
        .captures_iter(&contents)
        .map(|c| (c.get(0).unwrap().start(), c.extract()))
    {
        let maybe_last_do = do_indicies.iter().filter(|x| x < &&idx).max();
        let maybe_last_dont = dont_indicies.iter().filter(|x| x < &&idx).max();
        println!("{:?}{:?}", maybe_last_do, maybe_last_dont);
        if let Some(last_dont) = maybe_last_dont {
            if last_dont > maybe_last_do.unwrap_or(&0) {
                continue;
            }
        }
        let result = left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
        total += result;
    }
    println!("{:?}", total);
}

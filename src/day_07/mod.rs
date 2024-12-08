use std::fs;

pub fn part_1() {
    let raw_maze = fs::read_to_string("src/day_07/input.txt")
        .unwrap()
        .replace("\r\n", "\n")
        .replace(":", "");

    let mut sums: Vec<(usize, Vec<usize>)> = vec![];
    for row in raw_maze.split("\n") {
        let mut numbers = row.split(" ").map(|el| el.parse::<usize>().unwrap());
        let target = numbers.next().unwrap();
        sums.push((target, numbers.collect()));
    }
    let x: usize = sums
        .iter()
        .map(|(target, numbers)| {
            if count_combinations(*target, numbers.clone()) > 0 {
                *target
            } else {
                0
            }
        })
        .sum();
    println!("{:?}", x);
}
pub fn part_2() {
    let raw_maze = fs::read_to_string("src/day_07/input.txt")
        .unwrap()
        .replace("\r\n", "\n")
        .replace(":", "");

    let mut sums: Vec<(usize, Vec<usize>)> = vec![];
    for row in raw_maze.split("\n") {
        let mut numbers = row.split(" ").map(|el| el.parse::<usize>().unwrap());
        let target = numbers.next().unwrap();
        sums.push((target, numbers.collect()));
    }
    let x: usize = sums
        .iter()
        .map(|(target, numbers)| {
            if count_combinations_part2(*target, numbers.clone()) > 0 {
                *target
            } else {
                0
            }
        })
        .sum();
    println!("{:?}", x);
}

fn count_combinations(target: usize, numbers: Vec<usize>) -> usize {
    let mut rev_numbers = numbers.to_vec();
    rev_numbers.reverse();
    let current = rev_numbers.pop().unwrap();
    count_combinations_inner(target, rev_numbers, current)
}

fn count_combinations_inner(target: usize, mut rev_numbers: Vec<usize>, current: usize) -> usize {
    match rev_numbers.pop() {
        None => (current == target) as usize,
        Some(_) if current > target => 0, // short-circuit for performance
        Some(next) => {
            count_combinations_inner(target, rev_numbers.clone(), current + next)
                + count_combinations_inner(target, rev_numbers.clone(), current * next)
        }
    }
}

fn count_combinations_part2(target: usize, numbers: Vec<usize>) -> usize {
    let mut rev_numbers = numbers.to_vec();
    rev_numbers.reverse();
    let current = rev_numbers.pop().unwrap();
    count_combinations_part2_inner(target, rev_numbers, current)
}

fn count_combinations_part2_inner(target: usize, mut rev_numbers: Vec<usize>, current: usize) -> usize {
    match rev_numbers.pop() {
        None => (current == target) as usize,
        Some(_) if current > target => 0, // short-circuit for performance
        Some(next) => {
            count_combinations_part2_inner(target, rev_numbers.clone(), current + next)
                + count_combinations_part2_inner(target, rev_numbers.clone(), current * next)
                + count_combinations_part2_inner(target, rev_numbers.clone(), (current.to_string() + &next.to_string()).parse::<usize>().unwrap())
        }
    }
}

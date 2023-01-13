use std::collections::{HashMap, HashSet};

pub fn task1() -> i32 {
    let test1: Vec<char> = ">".chars().collect();
    let test2: Vec<char> = "^>v<".chars().collect();
    let test3: Vec<char> = "^v^v^v^v^v".chars().collect();

    let data: Vec<char> = include_str!("input.txt").chars().collect();

    calc_houses(&data)
}

pub fn calc_houses(instructions: &Vec<char>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut my_set: HashSet<(i32, i32)> = HashSet::new();

    my_set.insert((x, y));
    for i in instructions {
        match i {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            _ => y -= 1,
        }
        my_set.insert((x, y));
    }

    my_set.len() as i32
}

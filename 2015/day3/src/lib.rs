use std::collections::HashSet;

pub fn task1() -> i32 {
    let data: Vec<char> = include_str!("input.txt").chars().collect();

    calc_houses1(&data)
}
pub fn task2() -> i32 {
    let data: Vec<char> = include_str!("input.txt").chars().collect();
    calc_houses2(&data)
}

fn calc_houses2(instructions: &Vec<char>) -> i32 {
    let (mut santa_x, mut santa_y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);
    let mut my_set: HashSet<(i32, i32)> = HashSet::new();

    my_set.insert((santa_x, santa_y));

    for (i, c) in instructions.iter().enumerate() {
        if i % 2 == 0 {
            match c {
                '>' => santa_x += 1,
                '<' => santa_x -= 1,
                '^' => santa_y += 1,
                _ => santa_y -= 1,
            }
        } else {
            match c {
                '>' => robo_x += 1,
                '<' => robo_x -= 1,
                '^' => robo_y += 1,
                _ => robo_y -= 1,
            }
        }
        my_set.insert((robo_x, robo_y));
        my_set.insert((santa_x, santa_y));
    }

    my_set.len() as i32
}

fn calc_houses1(instructions: &Vec<char>) -> i32 {
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

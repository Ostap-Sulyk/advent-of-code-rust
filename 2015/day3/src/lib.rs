use std::collections::HashSet;

pub fn task1() -> i32 {
    let data: Vec<char> = include_str!("input.txt").chars().collect();

    calc_houses1(&data)
}
pub fn task2() -> i32 {
    let data: Vec<char> = include_str!("input.txt").chars().collect();
    calc_houses2(&data)
}

fn calc_houses1(instructions: &Vec<char>) -> i32 {
    let (mut x, mut y) = (0, 0);
    let mut my_set: HashSet<(i32, i32)> = HashSet::new();
    my_set.insert((x, y));

    for i in instructions {
        matcher(i, (&mut x, &mut y));
        my_set.insert((x, y));
    }

    my_set.len() as i32
}

fn calc_houses2(instructions: &Vec<char>) -> i32 {
    let (mut santa_x, mut santa_y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);
    let mut my_set: HashSet<(i32, i32)> = HashSet::new();

    my_set.insert((santa_x, santa_y));

    for (i, c) in instructions.iter().enumerate() {
        match i % 2 {
            0 => matcher(c, (&mut santa_x, &mut santa_y)),
            _ => matcher(c, (&mut robo_x, &mut robo_y)),
        }

        my_set.insert((robo_x, robo_y));
        my_set.insert((santa_x, santa_y));
    }

    my_set.len() as i32
}

fn matcher(character: &char, coords: (&mut i32, &mut i32)) {
    match character {
        '>' => *coords.0 += 1,
        '<' => *coords.0 -= 1,
        '^' => *coords.1 += 1,
        _ => *coords.1 -= 1,
    }
}

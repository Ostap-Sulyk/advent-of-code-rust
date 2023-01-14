mod task1;
mod task2;

pub fn task1() -> i32 {
    let data: Vec<&str> = include_str!("input.txt").lines().collect();
    data.iter()
        .map(|&x| {
            (task1::nice_vowels(x) && task1::twice_in_a_row(x) && task1::does_not_contain_x(x))
                as i32
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

pub fn task2() -> i32 {
    0
}
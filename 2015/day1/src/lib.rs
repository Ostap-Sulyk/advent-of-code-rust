pub fn task1() -> i32 {
    include_str!("input.txt")
        .split("")
        .map(|c| match c {
            ")" => -1,
            "(" => 1,
            _ => 0,
        })
        .sum()
}

pub fn task2() -> i32 {
    let data: Vec<i32> = include_str!("input.txt")
        .split("")
        .map(|c| match c {
            ")" => -1,
            "(" => 1,
            _ => 0,
        })
        .collect();
    let mut floor = 0;
    let mut basement_index = 0;
    for instruction in data {
        floor += instruction;
        if floor == -1 {
            break;
        }
        basement_index += 1
    }
    basement_index
}

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

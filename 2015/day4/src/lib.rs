use md5;

pub fn task1() -> i32 {
    mine("iwrupvqb", "00000")
}
pub fn task2() -> i32 {
    mine("iwrupvqb", "000000")
}

fn mine(key: &str, matching: &str) -> i32 {
    let mut num = 0;
    loop {
        let digested = md5::compute(format!("{}{}", key, num));
        let hash = format!("{:x}", digested);

        if &hash[0..matching.len()] == matching {
            break;
        }
        num += 1;
    }
    num
}

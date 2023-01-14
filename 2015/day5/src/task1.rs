const VOWS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const BANNED_LIST: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn nice_vowels(s: &str) -> bool {
    let mut count = 0;
    for c in s.chars() {
        if VOWS.contains(&c) {
            count += 1;
        }
    }
    return count >= 3;
}

pub fn twice_in_a_row(s: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let mut counter = 0;
    loop {
        if s[counter] == s[counter + 1] {
            break true;
        }
        counter += 1;
        if counter == s.len() - 1 {
            break false;
        }
    }
}

pub fn does_not_contain_x(s: &str) -> bool {
    let mut no_banned = true;
    for banned in BANNED_LIST {
        if s.contains(banned) {
            no_banned = false;
        }
    }
    no_banned
}
mod helpers;
use helpers::read_lines;

use eyre::{eyre, Result};
use scan_fmt::scan_fmt;

pub fn check_passwd_old(min: i32, max: i32, letter: char, passwd: &str) -> bool {
    let mut count: i32 = 0;
    for ch in passwd.chars() {
        if ch == letter {
            count += 1;
        }
    }
    count >= min && count <= max
}

pub fn check_passwd_new(pos1: usize, pos2: usize, letter: char, passwd: &str) -> bool {
    let char_vec: Vec<char> = passwd.chars().collect();
    let mut count: i32 = 0;
    if char_vec[pos1 - 1] == letter {
        count += 1;
    }
    if char_vec[pos2 - 1] == letter {
        count += 1;
    }
    count == 1
}

fn main() -> Result<()> {
    let mut num_ok: i32 = 0;
    for line in read_lines("input.txt") {
        let line = line?;
        let scanned = scan_fmt!(&line, "{}-{} {}: {}", usize, usize, char, String);
        if let Ok((pos1, pos2, letter, passwd)) = scanned {
            if check_passwd_new(pos1, pos2, letter, &passwd) {
                num_ok += 1;
            }
        } else {
            return Err(eyre!("Line has the wrong format: {}", &line));
        }
    }
    println!("{}", num_ok);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_old() -> Result<()> {
        assert!(check_passwd_old(1, 3, 'a', "abcde"));
        assert!(!check_passwd_old(1, 3, 'b', "cdefg"));
        assert!(check_passwd_old(2, 9, 'c', "ccccccccc"));
        Ok(())
    }

    #[test]
    fn test_new() -> Result<()> {
        assert!(check_passwd_new(1, 3, 'a', "abcde"));
        assert!(!check_passwd_new(1, 3, 'b', "cdefg"));
        assert!(!check_passwd_new(2, 9, 'c', "ccccccccc"));
        Ok(())
    }

}

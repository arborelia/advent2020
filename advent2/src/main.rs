mod helpers;
use helpers::read_lines;

use eyre::{eyre, Result};
use scan_fmt::scan_fmt;

fn check_passwd(min: i32, max: i32, letter: char, passwd: &str) -> bool {
    let mut count: i32 = 0;
    for ch in passwd.chars() {
        if ch == letter {
            count += 1;
        }
    }
    count >= min && count <= max
}

fn main() -> Result<()> {
    let mut num_ok: i32 = 0;
    for line in read_lines("input.txt") {
        let line = line?;
        let scanned = scan_fmt!(&line, "{}-{} {}: {}", i32, i32, char, String);
        if let Ok((min, max, letter, passwd)) = scanned {
            if check_passwd(min, max, letter, &passwd) {
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
    fn it_works() -> Result<()> {
        Ok(())
    }
}

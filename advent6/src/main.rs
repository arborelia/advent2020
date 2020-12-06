use eyre::Result;
use std::collections::HashSet;

fn string_to_set(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn num_unique_letters(lines: &Vec<&str>) -> u32 {
    let union = lines
        .iter()
        .cloned()
        .map(string_to_set)
        .fold(HashSet::new(), |a, b| &a | &b);
    union.len() as u32
}

fn num_common_letters(lines: &Vec<&str>) -> u32 {
    let init: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let intersection = lines
        .iter()
        .cloned()
        .map(string_to_set)
        .fold(init, |a, b| &a & &b);
    intersection.len() as u32
}

fn main() -> Result<()> {
    let content = std::fs::read_to_string("input.txt")?;
    let groups: Vec<&str> = content.split("\n\n").collect();
    let mut total_unique: u32 = 0;
    let mut total_common: u32 = 0;
    for group in groups {
        let lines: Vec<&str> = group.trim().split("\n").collect();
        total_unique += num_unique_letters(&lines);
        total_common += num_common_letters(&lines);
    }
    println!("Total unique letters: {}", total_unique);
    println!("Total common letters: {}", total_common);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(num_common_letters(&vec!["abc"]), 3);
        assert_eq!(num_common_letters(&vec!["a", "b", "c"]), 0);
        assert_eq!(num_common_letters(&vec!["ab", "ac"]), 1);
        assert_eq!(num_common_letters(&vec!["a", "a", "a", "a"]), 1);
        assert_eq!(num_common_letters(&vec!["b"]), 1);
    }
}

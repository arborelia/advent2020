mod helpers;
use helpers::read_lines;
use std::io::{Result};

fn _find_pair_product(numbers: &Vec<i64>) -> Option<i64> {
    let n = numbers.len();
    for pos1 in 0..n {
        for pos2 in (pos1 + 1)..n {
            if numbers[pos1] + numbers[pos2] == 2020 {
                return Some(numbers[pos1] * numbers[pos2]);
            }
        }
    }
    None
}

fn find_triple_product(numbers: &Vec<i64>) -> Option<i64> {
    let n = numbers.len();
    for pos1 in 0..n {
        for pos2 in (pos1 + 1)..n {
            for pos3 in (pos2 + 1)..n {
                if numbers[pos1] + numbers[pos2] + numbers[pos3] == 2020 {
                    return Some(numbers[pos1] * numbers[pos2] * numbers[pos3]);
                }
            }
        }
    }
    None
}

fn main() -> Result<()> {
    let mut numbers: Vec<i64> = Vec::new();
    for line in read_lines("input.txt") {
        let num: i64 = line?.parse().unwrap();
        numbers.push(num);
    }
    let product = find_triple_product(&numbers).expect("No triple adds to 2020");
    println!("{}", product);
    Ok(())
}

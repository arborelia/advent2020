mod helpers;
use helpers::read_lines;
use eyre::{Result, eyre};


fn find_pair_product(numbers: &Vec<i64>, total: i64) -> Result<i64> {
    let n = numbers.len();
    for pos1 in 0..n {
        for pos2 in (pos1 + 1)..n {
            if numbers[pos1] + numbers[pos2] == total {
                return Ok(numbers[pos1] * numbers[pos2]);
            }
        }
    }
    Err(eyre!("No pair adds to {}", total))
}

fn find_triple_product(numbers: &Vec<i64>, total: i64) -> Result<i64> {
    let n = numbers.len();
    for pos1 in 0..n {
        for pos2 in (pos1 + 1)..n {
            for pos3 in (pos2 + 1)..n {
                if numbers[pos1] + numbers[pos2] + numbers[pos3] == total {
                    return Ok(numbers[pos1] * numbers[pos2] * numbers[pos3]);
                }
            }
        }
    }
    Err(eyre!("No triple adds to {}", total))
}

fn main() -> Result<()> {
    let mut numbers: Vec<i64> = Vec::new();
    for line in read_lines("input.txt") {
        let num: i64 = line?.parse()?;
        numbers.push(num);
    }
    let product = find_triple_product(&numbers, 2020)?;
    println!("{}", product);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pair() -> Result<()> {
        let nums = vec![1, 2, -2, -3, 4];
        assert_eq!(find_pair_product(&nums, 0)?, -4);
        assert_eq!(find_pair_product(&nums, -1)?, -2);
        assert_eq!(find_pair_product(&nums, 1)?, -12);
        Ok(())
    }

    #[test]
    fn find_triple() -> Result<()> {
        let nums = vec![1, 2, -2, -3, 4];
        assert_eq!(find_triple_product(&nums, 0)?, -6);
        assert_eq!(find_triple_product(&nums, -1)?, 24);
        Ok(())
    }
}
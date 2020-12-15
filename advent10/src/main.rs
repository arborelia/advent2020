mod helpers;
use helpers::get_lines;

pub fn joltage_rating(jolts: &Vec<i64>) -> i64 {
    let mut sorted_jolts = jolts.clone();
    sorted_jolts.push(0);
    sorted_jolts.sort();
    let mut diffs1: i64 = 0;
    let mut diffs3: i64 = 0;
    for i in 0..(sorted_jolts.len() - 1) {
        let diff = sorted_jolts[i + 1] - sorted_jolts[i];
        assert!(1 <= diff && diff <= 3);
        if diff == 1 {
            diffs1 += 1
        } else if diff == 3 {
            diffs3 += 1
        }
    }
    // implicitly add 1 to diffs3 for the last connection
    diffs1 * (diffs3 + 1)
}

pub fn joltage_arrangements(jolts: &Vec<i64>) -> i64 {
    let mut sorted_jolts = jolts.clone();
    sorted_jolts.push(0);
    sorted_jolts.sort();
    let highest_jolts = sorted_jolts[sorted_jolts.len() - 1];
    sorted_jolts.push(highest_jolts + 3);

    let n = sorted_jolts.len();
    let mut arrangements_up_to: Vec<i64> = vec![0; n];
    arrangements_up_to[0] = 1;

    for current in 1..n {
        let lower_bound: usize = if current >= 3 { current - 3 } else { 0 };
        for prev in lower_bound..current {
            if sorted_jolts[prev] >= sorted_jolts[current] - 3 {
                arrangements_up_to[current] += arrangements_up_to[prev];
            }
        }
    }
    arrangements_up_to[n - 1]
}

fn main() {
    let joltage: Vec<i64> = get_lines("input.txt")
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let rating = joltage_rating(&joltage);
    let arrangements = joltage_arrangements(&joltage);
    println!("Joltage rating of using all adapters: {}", rating);
    println!("Number of arrangements: {}", arrangements);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let example: Vec<i64> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(joltage_arrangements(&example), 8);

        let example: Vec<i64> = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(joltage_arrangements(&example), 19208);
    }
}

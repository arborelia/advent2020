mod helpers;
use helpers::get_lines;

fn sum_pair_in_window(window: &[i64], sum: i64) -> bool {
    // println!("window: {:?}\nnext: {}", window, sum);
    for num1 in window {
        for num2 in window {
            if num1 != num2 && num1 + num2 == sum {
                return true;
            }
        }
    }
    false
}

fn number_out_of_window(seq: &Vec<i64>, window_size: usize) -> i64 {
    for offset in 0..(seq.len() - window_size) {
        let window: &[i64] = &seq[offset..(offset + window_size)];
        let next: i64 = seq[offset + window_size];
        if !sum_pair_in_window(&window, next) {
            return next;
        }
    }
    panic!("all the numbers are okay")
}

fn find_contiguous_sum(seq: &Vec<i64>, target: i64) -> i64 {
    let mut running_sums: Vec<i64> = Vec::new();
    let mut total: i64 = 0;
    for num in seq {
        running_sums.push(total);
        total += num;
    }
    running_sums.push(total);

    let running_sums = running_sums;
    for start in 0..(running_sums.len() - 2) {
        for end in (start + 1)..running_sums.len() {
            if running_sums[end] - running_sums[start] == target {
                let slice: &[i64] = &seq[start..end];
                let min = slice.iter().min().unwrap();
                let max = slice.iter().max().unwrap();
                return min + max;
            }
        }
    }
    panic!("No contiguous sum found");
}

fn main() {
    let seq: Vec<i64> = get_lines("input.txt")
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    let num: i64 = number_out_of_window(&seq, 25);
    let weakness: i64 = find_contiguous_sum(&seq, num);
    println!("number out of window is {}", num);
    println!("encryption weakness is {}", weakness);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_out_of_window() {
        let seq = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(number_out_of_window(&seq, 5), 127);

        let seq = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127,
        ];
        assert_eq!(number_out_of_window(&seq, 5), 127);
        assert_eq!(number_out_of_window(&seq, 2), 15);
    }

    #[test]
    fn test_weakness() {
        let seq = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(find_contiguous_sum(&seq, 127), 62);
    }
}

mod helpers;
use helpers::read_lines;

// Get the first character and the remaining characters of a string.
fn string_head_tail(s: &str) -> (&str, &str) {
    (&s[0..1], &s[1..])
}

fn binary_seat_search(code: &str, front: i64, back: i64, left: i64, right: i64) -> i64 {
    if code == "" {
        assert!(front + 1 == back);
        assert!(left + 1 == right);
        front * 8 + left
    } else {
        let (head, tail) = string_head_tail(code);
        if head == "F" {
            binary_seat_search(tail, front, (front + back) / 2, left, right)
        } else if head == "B" {
            binary_seat_search(tail, (front + back) / 2, back, left, right)
        } else if head == "L" {
            binary_seat_search(tail, front, back, left, (left + right) / 2)
        } else if head == "R" {
            binary_seat_search(tail, front, back, (left + right) / 2, right)
        } else {
            panic!("what")
        }
    }
}

pub fn interpret_binary_seat(code: &str) -> i64 {
    binary_seat_search(code, 0, 128, 0, 8)
}

fn main() {
    let mut max_seat: i64 = 0;
    for line in read_lines("input.txt") {
        let line = line.unwrap();
        let seat_id = interpret_binary_seat(&line);
        if seat_id > max_seat {
            max_seat = seat_id;
        }
    }
    println!("{}", max_seat)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let seat_num: i64 = binary_seat_search("FBFBBFFRLR", 0, 128, 0, 8);
        assert_eq!(seat_num, 44 * 8 + 5)
    }

    #[test]
    fn test_top_level() {
        assert_eq!(interpret_binary_seat("FBFBBFFRLR"), 44 * 8 + 5);
        assert_eq!(interpret_binary_seat("FFFBBBFRRR"), 14 * 8 + 7);
        assert_eq!(interpret_binary_seat("BFFFBBFRRR"), 70 * 8 + 7);
        assert_eq!(interpret_binary_seat("BBFFBBFRLL"), 102 * 8 + 4);
    }
}
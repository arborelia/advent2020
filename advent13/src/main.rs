use std::fs::read_to_string;

use scan_fmt::scan_fmt;

mod chinese_remainder;
use chinese_remainder::chinese_remainder;

fn parse_bus_number(input: &str) -> Option<i64> {
    if input == "x" {
        None
    } else {
        Some(input.parse().unwrap())
    }
}

fn parse_indexed_bus_list(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut buses: Vec<i64> = Vec::new();
    let mut indices: Vec<i64> = Vec::new();
    for (index, bus) in input.split(",").enumerate() {
        match parse_bus_number(bus) {
            Some(num) => {
                buses.push(num);
                indices.push(index as i64);
            }
            None => {}
        }
    }
    (buses, indices)
}

#[derive(PartialEq, Debug)]
struct BusResult {
    bus_number: i64,
    wait_time: i64,
}

fn find_earliest_bus(start_time: i64, buses: &[i64]) -> BusResult {
    let mut earliest_bus: i64 = 0;
    let mut shortest_wait: i64 = i64::MAX;
    for &bus in buses {
        // get the time of the bus that's at or before the start time, using floor division
        let mut bus_time = start_time / bus * bus;
        // then adjust it to be the next bus at or after the start time
        if bus_time < start_time {
            bus_time += bus;
        }
        let wait_time = bus_time - start_time;
        if wait_time < shortest_wait {
            earliest_bus = bus;
            shortest_wait = wait_time;
        }
    }
    BusResult {
        bus_number: earliest_bus,
        wait_time: shortest_wait,
    }
}

fn solve_bus_puzzle(buses: &[i64], indices: &[i64]) -> i64 {
    // It appears they're giving us input where the bus numbers are co-prime.
    // The task would be harder if they didn't. As it is, we can use a known algorithm.
    // We just need each of the residues to be the negative of the time step, because the
    // first time step needs to be that number of minutes _before_ the bus arrives.
    let residues: Vec<i64> = indices.iter().map(|&idx| -idx).collect();
    chinese_remainder(&residues, buses).expect("Bus numbers weren't co-prime")
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (start_time, bus_list) = scan_fmt!(&input, "{d}\n{}", i64, String).unwrap();
    let (buses, indices) = parse_indexed_bus_list(&bus_list);
    let bus_result = find_earliest_bus(start_time, &buses);
    let solution = solve_bus_puzzle(&buses, &indices);
    println!(
        "earliest bus answer is {}",
        bus_result.bus_number * bus_result.wait_time
    );
    println!("bus puzzle solution is {}", solution);
}

#[test]
fn test_example() {
    let answer = BusResult {
        bus_number: 59,
        wait_time: 5,
    };
    assert_eq!(find_earliest_bus(939, &[7, 13, 59, 31, 19]), answer);
    assert_eq!(
        solve_bus_puzzle(&[7, 13, 59, 31, 19], &[0, 1, 4, 6, 7]),
        1068781
    );
}

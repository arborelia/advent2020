use std::fs::read_to_string;

use scan_fmt::scan_fmt;

fn parse_bus_number(input: &str) -> Option<i64> {
    if input == "x" {
        None
    } else {
        Some(input.parse().unwrap())
    }
}

fn parse_bus_list(input: &str) -> Vec<i64> {
    let mut buses: Vec<i64> = Vec::new();
    for bus in input.split(",") {
        match parse_bus_number(bus) {
            Some(num) => buses.push(num),
            None => {}
        }
    }
    buses
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

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (start_time, bus_list) = scan_fmt!(&input, "{d}\n{}", i64, String).unwrap();
    let buses = parse_bus_list(&bus_list);
    let result = find_earliest_bus(start_time, &buses);
    println!("{}", result.bus_number * result.wait_time);
}

#[test]
fn test_example() {
    let answer = BusResult {
        bus_number: 59,
        wait_time: 5,
    };
    assert_eq!(find_earliest_bus(939, &[7, 13, 59, 31, 19]), answer)
}

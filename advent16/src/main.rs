use ndarray::{Array, Array2};
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

#[derive(Clone, PartialEq, Debug)]
struct Field {
    name: String,
    lower1: u64,
    upper1: u64,
    lower2: u64,
    upper2: u64,
}

#[derive(Clone, PartialEq, Debug)]
struct Ticket {
    mine: bool,
    values: Vec<u64>,
}

#[derive(Debug)]
struct TicketNotes {
    fields: Vec<Field>,
    tickets: Vec<Ticket>,
}

enum ParseState {
    Fields,
    MyTicket,
    NearbyTickets,
}

fn parse_field_description(input: &str) -> Field {
    let (name, lower1, upper1, lower2, upper2) = scan_fmt!(
        input,
        "{/[a-zA-Z ]+/}: {d}-{d} or {d}-{d}",
        String,
        u64,
        u64,
        u64,
        u64
    )
    .unwrap();
    Field {
        name,
        lower1,
        lower2,
        upper1,
        upper2,
    }
}

fn parse_ticket(input: &str, mine: bool) -> Ticket {
    let numeric_strs = input.split(",");
    let values: Vec<u64> = numeric_strs.map(|numstr| numstr.parse().unwrap()).collect();
    Ticket { mine, values }
}

fn parse_ticket_notes(input: &str) -> TicketNotes {
    let mut fields: Vec<Field> = Vec::new();
    let mut tickets: Vec<Ticket> = Vec::new();
    let mut state = ParseState::Fields;

    for line in input.split("\n") {
        if line != "" {
            match state {
                ParseState::Fields => {
                    if line == "your ticket:" {
                        state = ParseState::MyTicket
                    } else {
                        let field = parse_field_description(&line);
                        fields.push(field);
                    }
                }
                ParseState::MyTicket => {
                    if line == "nearby tickets:" {
                        state = ParseState::NearbyTickets
                    } else {
                        let ticket = parse_ticket(&line, true);
                        tickets.push(ticket);
                    }
                }
                ParseState::NearbyTickets => {
                    let ticket = parse_ticket(&line, false);
                    tickets.push(ticket);
                }
            }
        }
    }
    TicketNotes { fields, tickets }
}

fn satisfies_field_constraint(field: &Field, value: u64) -> bool {
    (value >= field.lower1 && value <= field.upper1)
        || (value >= field.lower2 && value <= field.upper2)
}

fn satisfies_any_field(fields: &[Field], value: u64) -> bool {
    for field in fields {
        if satisfies_field_constraint(field, value) {
            return true;
        }
    }
    false
}

fn scan_error_rate(notes: &TicketNotes) -> u64 {
    let mut error_rate: u64 = 0;
    for ticket in &notes.tickets {
        if !ticket.mine {
            for value in &ticket.values {
                if !satisfies_any_field(&notes.fields, *value) {
                    error_rate += value;
                }
            }
        }
    }
    error_rate
}

fn filter_valid_tickets(notes: &TicketNotes) -> TicketNotes {
    let mut valid_tickets: Vec<Ticket> = Vec::new();
    for ticket in &notes.tickets {
        let mut is_valid: bool = true;
        if !ticket.mine {
            for value in &ticket.values {
                if !satisfies_any_field(&notes.fields, *value) {
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid {
            valid_tickets.push(ticket.clone())
        }
    }
    TicketNotes {
        fields: notes.fields.clone(),
        tickets: valid_tickets,
    }
}

fn find_field_order(notes: &TicketNotes) -> Vec<usize> {
    let ticket_size = notes.fields.len();
    assert_eq!(ticket_size, notes.tickets[0].values.len());
    let mut constraint_grid: Array2<u32> = Array::ones((ticket_size, ticket_size));

    // Make a table of which fields can have which indices
    for row in 0..ticket_size {
        for col in 0..ticket_size {
            let field = &notes.fields[row];
            for ticket in &notes.tickets {
                if !ticket.mine {
                    if !satisfies_field_constraint(field, ticket.values[col]) {
                        constraint_grid[(row, col)] = 0;
                        break;
                    }
                }
            }
        }
    }
    println!("{:?}", constraint_grid);
    // In the constraint grid, rows are which field to consider,
    // and columns are where they could be in order.

    for _iter in 0..ticket_size {
        for row in 0..ticket_size {
            let sum: u32 = constraint_grid.row(row).sum();
            if sum == 0 {
                panic!("Field {} has no possible locations", row);
            } else if sum == 1 {
                let row_content = constraint_grid.row(row);
                let unique_col = row_content.iter().position(|&val| val == 1).unwrap();
                // println!("Field {} must be in position {}", row, unique_col);
                for other_row in 0..ticket_size {
                    if other_row != row {
                        constraint_grid[(other_row, unique_col)] = 0;
                    }
                }
            }
        }
    }
    println!("{:?}", constraint_grid);

    let mut field_positions: Vec<usize> = Vec::new();
    for row in 0..ticket_size {
        let unique_col = constraint_grid
            .row(row)
            .iter()
            .position(|&val| val == 1)
            .unwrap();
        field_positions.push(unique_col);
    }
    field_positions
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let notes = parse_ticket_notes(&input);
    println!("Scan error rate: {}", scan_error_rate(&notes));

    let revised_notes = filter_valid_tickets(&notes);
    let field_positions = find_field_order(&revised_notes);

    let my_ticket = &revised_notes.tickets[0];
    assert!(my_ticket.mine);

    let mut product: u64 = 1;
    for (i, field) in revised_notes.fields.iter().enumerate() {
        if field.name.starts_with("departure ") {
            let val = my_ticket.values[field_positions[i]];
            product *= val;
            println!("{}: {}", field.name, val);
        }
    }
    println!("product: {}", product);
}

#[test]
fn test_scan_errors() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    let notes = parse_ticket_notes(input);
    assert_eq!(scan_error_rate(&notes), 71);

    let revised_notes = filter_valid_tickets(&notes);
    assert_eq!(revised_notes.tickets.len(), 2);
}

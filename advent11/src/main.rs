mod helpers;
use core::default::Default;
use helpers::get_lines;
use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Seat {
    Floor,
    Empty,
    Full,
}

impl Default for Seat {
    fn default() -> Self {
        Seat::Floor
    }
}

/// Read lines of the problem's file format into a 2d array of Seats.
fn lines_to_grid(lines: &[String]) -> Array2<Seat> {
    let n_rows = lines.len();
    let n_cols = lines[0].len();

    // Initialize the grid to all Floor, and leave a padding of Floor around
    // the part that will be assigned.
    let mut grid: Array2<Seat> = Array2::default((n_rows + 2, n_cols + 2));

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'L' {
                grid[(row + 1, col + 1)] = Seat::Empty;
            }
        }
    }
    grid
}

fn update_grid(grid: &Array2<Seat>) -> Array2<Seat> {
    let mut newgrid = grid.clone();
    let (n_rows, n_cols) = grid.dim();

    // Iterate over upper-left corners of windows we'll look at
    // (do this instead of iterating over centers so that we don't have to
    // subtract usizes)
    for row in 0..(n_rows - 2) {
        for col in 0..(n_cols - 2) {
            let center_seat = grid[(row + 1, col + 1)];
            if center_seat != Seat::Floor {
                // count neighbors of (row + 1, col + 1)
                let mut num_neighbors: u32 = 0;
                for d_row in 0..=2 {
                    for d_col in 0..=2 {
                        if d_row != 1 || d_col != 1 {
                            if grid[(row + d_row, col + d_col)] == Seat::Full {
                                num_neighbors += 1;
                            }
                        }
                    }
                }

                // make changes according to the rules
                if center_seat == Seat::Full && num_neighbors >= 4 {
                    newgrid[(row + 1, col + 1)] = Seat::Empty;
                } else if center_seat == Seat::Empty && num_neighbors == 0 {
                    newgrid[(row + 1, col + 1)] = Seat::Full;
                }
            }
        }
    }

    newgrid
}

fn update_by_visibility(grid: &Array2<Seat>) -> Array2<Seat> {
    let mut newgrid = grid.clone();
    let (n_rows, n_cols) = grid.dim();

    // Iterate over upper-left corners of windows we'll look at
    // (do this instead of iterating over centers so that we don't have to
    // subtract usizes)
    for row in 1..n_rows {
        for col in 1..n_cols {
            let center_seat = grid[(row, col)];
            if center_seat != Seat::Floor {
                // count visible full seats from (row + 1, col + 1)
                let mut num_visible: u32 = 0;
                for d_row in -1..=1 {
                    for d_col in -1..=1 {
                        if d_row != 0 || d_col != 0 {
                            let mut c_row = row as i32;
                            let mut c_col = col as i32;
                            loop {
                                c_row += d_row;
                                c_col += d_col;
                                if c_row < 0
                                    || c_row >= n_rows as i32
                                    || c_col < 0
                                    || c_col >= n_cols as i32
                                {
                                    break;
                                }
                                if grid[(c_row as usize, c_col as usize)] == Seat::Full {
                                    num_visible += 1;
                                    break;
                                } else if grid[(c_row as usize, c_col as usize)] == Seat::Empty {
                                    break;
                                }
                            }
                        }
                    }
                }

                // make changes according to the rules
                if center_seat == Seat::Full && num_visible >= 5 {
                    newgrid[(row, col)] = Seat::Empty;
                } else if center_seat == Seat::Empty && num_visible == 0 {
                    newgrid[(row, col)] = Seat::Full;
                }
                // let seat_char: char = match newgrid[(row, col)] {
                //     Seat::Full => '#',
                //     Seat::Empty => 'L',
                //     Seat::Floor => '.',
                // };
                // print!("{}", seat_char);
            }
        }
        // println!("");
    }

    newgrid
}

fn count_grid(grid: &Array2<Seat>) -> usize {
    grid.iter().filter(|&&seat| seat == Seat::Full).count()
}

/// set visibility=true to update by visibility,
fn iterate_until_fixed_point(grid: &Array2<Seat>, visibility: bool) -> Array2<Seat> {
    let mut grid = grid.clone();
    loop {
        println!("{}", count_grid(&grid));
        let newgrid: Array2<Seat> = if visibility {
            update_by_visibility(&grid)
        } else {
            update_grid(&grid)
        };
        if newgrid == grid {
            return newgrid;
        } else {
            grid = newgrid;
        }
    }
}

fn main() {
    let grid = lines_to_grid(&get_lines("input.txt"));
    let final_grid = iterate_until_fixed_point(&grid, false);
    println!("{} seats occupied by adjacency", count_grid(&final_grid));

    let final_grid = iterate_until_fixed_point(&grid, true);
    println!("{} seats occupied by visibility", count_grid(&final_grid));
}

#[test]
fn test_visibility() {
    let lines: Vec<String> = vec![
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ]
    .iter()
    .map(|&line| line.to_string())
    .collect();

    let grid = lines_to_grid(&lines);
    let final_grid = iterate_until_fixed_point(&grid, true);
    let count = count_grid(&final_grid);
    assert_eq!(count, 26);
}

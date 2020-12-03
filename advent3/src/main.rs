use array2d::Array2D;
use eyre::Result;
mod helpers;
use helpers::get_lines;

// Evaluate one path of the toboggan down a slope, returning how many trees it hits.
// In part 1, right=3 and down=1.
pub fn evaluate_path(grid: &Array2D<bool>, right: usize, down: usize) -> u64 {
    let mut hits: u64 = 0;
    let mut row: usize = 0;
    let mut col: usize = 0;
    let width: usize = grid.row_len();
    let height: usize = grid.column_len();
    loop {
        if row >= height {
            return hits;
        }
        if grid[(row, col)] {
            hits += 1;
        }
        row += down;
        col += right;
        col %= width;
    }
}

// Evaluate five specific paths and multiply them together, computing the result required
// for part 2.
pub fn evaluate_paths(grid: &Array2D<bool>) -> u64 {
    evaluate_path(&grid, 1, 1)
        * evaluate_path(&grid, 3, 1)
        * evaluate_path(&grid, 5, 1)
        * evaluate_path(&grid, 7, 1)
        * evaluate_path(&grid, 1, 2)
}

// Convert a line like '#...#.#' to a boolean vector, which is true when there is
// a tree (#).
fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|ch| ch == '#').collect()
}

// Convert an input file to an Array2D which is true where there are trees (#).
fn parse_file(filename: &str) -> Result<Array2D<bool>> {
    let lines: Vec<Vec<bool>> = get_lines(filename)
        .into_iter()
        .map(|line| parse_line(&line))
        .collect();
    Ok(Array2D::from_rows(&lines))
}

fn main() -> Result<()> {
    let grid: Array2D<bool> = parse_file("input.txt")?;
    let hits: u64 = evaluate_paths(&grid);
    println!("{}", hits);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    // The example grid provided in the problem statement
    fn make_example_grid() -> Array2D<bool> {
        let example_grid = vec![
            vec![
                false, false, true, true, false, false, false, false, false, false, false,
            ],
            vec![
                true, false, false, false, true, false, false, false, true, false, false,
            ],
            vec![
                false, true, false, false, false, false, true, false, false, true, false,
            ],
            vec![
                false, false, true, false, true, false, false, false, true, false, true,
            ],
            vec![
                false, true, false, false, false, true, true, false, false, true, false,
            ],
            vec![
                false, false, true, false, true, true, false, false, false, false, false,
            ],
            vec![
                false, true, false, true, false, true, false, false, false, false, true,
            ],
            vec![
                false, true, false, false, false, false, false, false, false, false, true,
            ],
            vec![
                true, false, true, true, false, false, false, true, false, false, false,
            ],
            vec![
                true, false, false, false, true, true, false, false, false, false, true,
            ],
            vec![
                false, true, false, false, true, false, false, false, true, false, true,
            ],
        ];
        Array2D::from_rows(&example_grid)
    }

    #[test]
    fn test_one_slope() -> Result<()> {
        let grid = make_example_grid();
        let hits: u64 = evaluate_path(&grid, 3, 1);
        assert_eq!(hits, 7);
        Ok(())
    }

    #[test]
    fn test_many_slopes() -> Result<()> {
        let grid = make_example_grid();
        let hits: u64 = evaluate_paths(&grid);
        assert_eq!(hits, 336);
        Ok(())
    }
}

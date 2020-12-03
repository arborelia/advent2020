use eyre::Result;
use array2d::Array2D;
mod helpers;
use helpers::read_lines;

pub fn evaluate_path(grid: &Array2D<bool>, right: usize, down: usize) -> u32 {
    let mut hits: u32 = 0;
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

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|ch| ch == '#').collect()
}

fn parse_file(filename: &str) -> Result<Array2D<bool>> {
    let mut lines: Vec<Vec<bool>> = Vec::new();
    for line in read_lines(filename) {
        let line = line?;
        lines.push(parse_line(&line));
    }
    Ok(Array2D::from_rows(&lines))
}

fn main() -> Result<()> {
    let grid: Array2D<bool> = parse_file("input.txt")?;
    let hits: u32 = evaluate_path(&grid, 3, 1);
    println!("{}", hits);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slope() -> Result<()> {
        let example_grid = vec![
            vec![false, false, true, true, false, false, false, false, false, false, false],
            vec![true, false, false, false, true, false, false, false, true, false, false],
            vec![false, true, false, false, false, false, true, false, false, true, false],
            vec![false, false, true, false, true, false, false, false, true, false, true],
            vec![false, true, false, false, false, true, true, false, false, true, false],
            vec![false, false, true, false, true, true, false, false, false, false, false],
            vec![false, true, false, true, false, true, false, false, false, false, true],
            vec![false, true, false, false, false, false, false, false, false, false, true],
            vec![true, false, true, true, false, false, false, true, false, false, false],
            vec![true, false, false, false, true, true, false, false, false, false, true],
            vec![false, true, false, false, true, false, false, false, true, false, true]
        ];
        let arr = Array2D::from_rows(&example_grid);
        let hits: u32 = evaluate_path(&arr, 3, 1);
        assert_eq!(hits, 7);
        Ok(())
    }
}


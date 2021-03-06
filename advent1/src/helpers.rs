use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};

pub fn read_lines_result(filename: &str) -> io::Result<Lines<impl BufRead>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

pub fn read_lines(filename: &str) -> Lines<impl BufRead> {
    read_lines_result(filename).unwrap()
}

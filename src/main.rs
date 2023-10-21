use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

mod formatter;
use formatter::Formatter;

fn main() {
    let input = File::open("in.py").unwrap();
    let buffered = BufReader::new(input);

    let lines = buffered
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| x.trim().len() > 0)
        .collect();

    let mut formatter = Formatter::new(lines);
    formatter.compute_blocks();

    let mut output = File::create("out.py").unwrap();
    write!(output, "{}", formatter.one_line_function()).unwrap();
}
